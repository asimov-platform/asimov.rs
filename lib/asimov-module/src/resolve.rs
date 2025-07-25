// This is free and unencumbered software released into the public domain.

use crate::ModuleManifest;
use alloc::{
    collections::{btree_map::BTreeMap, btree_set::BTreeSet},
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::{borrow::Borrow, convert::Infallible};
use error::UrlParseError;

pub mod error;

#[derive(Clone, Debug, Default)]
pub struct Resolver {
    modules: BTreeMap<String, Rc<Module>>,
    file_extensions: BTreeMap<String, Vec<Rc<Module>>>,
    nodes: slab::Slab<Node>,
    roots: BTreeMap<Sect, usize>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver::default()
    }

    pub fn resolve(&self, url: &str) -> Result<Vec<Rc<Module>>, UrlParseError> {
        let input = split_url(url)?;

        let mut results: BTreeSet<Rc<Module>> = BTreeSet::new();

        if matches!(input.first(), Some(Sect::Protocol(proto)) if proto == "file") {
            if let Some(Sect::Path(filename)) = input.last() {
                if let Some((_, ext)) = filename.split_once(".") {
                    self.file_extensions
                        .get(ext)
                        .into_iter()
                        .flatten()
                        .for_each(|module| {
                            results.insert(module.clone());
                        });
                }
            }
        }

        let with_freemove = |node_idx: usize| {
            // Return the node ID
            core::iter::once(node_idx)
                // And the destination ID after following a `FreeMove` path from the node
                .chain(self.nodes[node_idx].paths.get(&Sect::FreeMove).copied())
        };

        // Initialize with start states that match the first input
        let start_states: BTreeSet<usize> = self
            .roots
            .iter()
            .filter_map(|(path, &node_idx)| path.matches_input(&input[0]).then_some(node_idx))
            .collect();

        let final_states = if input.len() == 1 {
            // There is no further input, just get free moves from the start_states
            start_states.into_iter().flat_map(with_freemove).collect()
        } else {
            // Process remaining input
            input[1..].iter().fold(start_states, |states, sect| {
                states
                    .into_iter()
                    .flat_map(|node_idx| &self.nodes[node_idx].paths)
                    .filter_map(|(path, &node_idx)| path.matches_input(sect).then_some(node_idx))
                    .flat_map(with_freemove)
                    .collect()
            })
        };

        // Collect all modules from final states
        for &state_idx in &final_states {
            for module in &self.nodes[state_idx].modules {
                results.insert(module.clone());
            }
        }

        Ok(results.into_iter().collect())
    }

    pub fn insert_file_extension(
        &mut self,
        module: &str,
        file_extension: &str,
    ) -> Result<(), Infallible> {
        let module = self.add_module(module);

        self.file_extensions
            .entry(file_extension.to_string())
            .or_default()
            .push(module);

        Ok(())
    }
    pub fn insert_manifest(&mut self, manifest: &ModuleManifest) -> Result<(), UrlParseError> {
        for protocol in &manifest.handles.url_protocols {
            self.insert_protocol(&manifest.name, protocol).ok();
        }
        for prefix in &manifest.handles.url_prefixes {
            self.insert_prefix(&manifest.name, prefix)?;
        }
        for pattern in &manifest.handles.url_patterns {
            self.insert_pattern(&manifest.name, pattern)?;
        }
        for file_extension in &manifest.handles.file_extensions {
            self.insert_file_extension(&manifest.name, file_extension)
                .ok();
        }
        Ok(())
    }
    pub fn insert_protocol(&mut self, module: &str, protocol: &str) -> Result<(), Infallible> {
        let path = &[Sect::Protocol(protocol.to_string()), Sect::FreeMove];
        let module = self.add_module(module);
        let node_idx = self.get_or_create_node(path);

        // Add a free move back to itself from the `FreeMove` node. (represents a protocol as a prefix):
        self.nodes[node_idx].paths.insert(Sect::FreeMove, node_idx);
        self.nodes[node_idx].modules.insert(module);

        Ok(())
    }
    pub fn insert_prefix(&mut self, module: &str, prefix: &str) -> Result<(), UrlParseError> {
        let mut path = split_url(prefix)?;
        // Add a `FreeMove` node at the end of the path to separate the prefix from
        // patterns at the same node
        path.push(Sect::FreeMove);
        let module = self.add_module(module);
        let node_idx = self.get_or_create_node(&path);

        // Add a free move back to itself from the `FreeMove` node. Enables matching
        // zero-or-more of anything:
        self.nodes[node_idx].paths.insert(Sect::FreeMove, node_idx);
        self.nodes[node_idx].modules.insert(module);

        Ok(())
    }
    pub fn insert_pattern(&mut self, module: &str, pattern: &str) -> Result<(), UrlParseError> {
        let path: Vec<Sect> = split_url(pattern)?
            .into_iter()
            .map(Sect::into_pattern)
            .collect();
        let module = self.add_module(module);
        let node_idx = self.get_or_create_node(&path);

        self.nodes[node_idx].modules.insert(module);

        Ok(())
    }

    #[cfg(all(feature = "std", feature = "serde"))]
    pub fn try_from_dir(path: impl AsRef<std::path::Path>) -> Result<Self, error::FromDirError> {
        use error::FromDirError;

        let path = path.as_ref();

        let dir = std::fs::read_dir(path).map_err(|source| FromDirError::ManifestDirIo {
            path: path.into(),
            source,
        })?;

        let mut resolver = Resolver::new();

        for entry in dir {
            let entry = entry.map_err(|source| FromDirError::ManifestDirIo {
                path: path.into(),
                source,
            })?;
            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                continue;
            }
            let filename = entry.file_name();
            let filename = filename.to_string_lossy();
            if !filename.ends_with(".yaml") && !filename.ends_with(".yml") {
                continue;
            }
            let path = entry.path();
            let file = std::fs::File::open(&path).map_err(|source| FromDirError::ManifestIo {
                path: path.clone(),
                source,
            })?;

            let manifest = serde_yml::from_reader(file).map_err(|source| FromDirError::Parse {
                path: path.clone(),
                source,
            })?;
            resolver
                .insert_manifest(&manifest)
                .map_err(|source| FromDirError::Insert {
                    path: path.clone(),
                    source,
                })?;
        }

        Ok(resolver)
    }

    pub fn try_from_iter<I, T>(mut iter: I) -> Result<Self, UrlParseError>
    where
        I: Iterator<Item = T>,
        T: Borrow<ModuleManifest>,
    {
        iter.try_fold(Resolver::default(), |mut r, m| {
            r.insert_manifest(m.borrow())?;
            Ok(r)
        })
    }

    fn get_or_create_node(&mut self, path: &[Sect]) -> usize {
        // Get or create the root node
        let root_idx = *self
            .roots
            .entry(path[0].clone())
            .or_insert_with(|| self.nodes.insert(Node::default()));

        path[1..].iter().fold(root_idx, |cur_idx, sect| {
            match (self.nodes[cur_idx].paths.get(sect), sect) {
                (Some(&idx), _sect) => idx,
                (None, Sect::WildcardDomain) => {
                    // If the sect is a wildcard domain add a link to self, this will also match multiple subdomains.
                    self.nodes[cur_idx].paths.insert(sect.clone(), cur_idx);
                    cur_idx
                },
                (None, sect) => {
                    // Create a new node
                    let new_node_idx = self.nodes.insert(Node::default());

                    // Add the transition from current node to new node
                    self.nodes[cur_idx].paths.insert(sect.clone(), new_node_idx);
                    new_node_idx
                },
            }
        })
    }

    fn add_module(&mut self, name: &str) -> Rc<Module> {
        let name = name.to_string();
        self.modules
            .entry(name.clone())
            .or_insert_with(|| Rc::new(Module { name }))
            .clone()
    }
}

impl TryFrom<&[ModuleManifest]> for Resolver {
    type Error = UrlParseError;

    fn try_from(value: &[ModuleManifest]) -> Result<Self, Self::Error> {
        Resolver::try_from_iter(value.iter())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Module {
    pub name: String,
}

#[derive(Clone, Debug, Default)]
struct Node {
    paths: BTreeMap<Sect, usize>,
    modules: BTreeSet<Rc<Module>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Sect {
    /// `https` from `https://example.org/`, matches the protocol (a.k.a. scheme) of an URL
    Protocol(String),
    /// `org` and `example` from `https://example.org/`, matches a single literal subdomain
    Domain(String),
    /// `*` from `https://*.example.org/`, matches zero-or-more subdomains
    WildcardDomain,
    /// `file` and `path` from `https://example.org/file/path`, match literal path segments
    Path(String),
    /// `:name` from `https://example.org/file/:name`, matches any single path segment
    WildcardPath,
    /// `q` from `https://example.org/?q=example`, matches a parameter name
    QueryParamName(String),
    /// `example` from `https://example.org/?q=example`, matches a literal parameter value
    QueryParamValue(String),
    /// `:query` from `https://example.org/?q=:query`, matches any query param value
    WildcardQueryParamValue,
    /// Matches a single section of any kind
    FreeMove,
}

impl Sect {
    /// Transform a sect that matches a pattern format to a wildcard.
    /// - If a domain section is "*", make it a wildcard domain pattern
    /// - If a path section begins with ":" ("/:foo/:bar"), make it a wildcard path pattern
    /// - If the value of a query parameter begins with ":" ("q=:query"), make it a wildcard query param pattern
    pub fn into_pattern(self) -> Self {
        match self {
            Sect::Domain(p) if p == "*" => Sect::WildcardDomain,
            Sect::Path(p) if p.starts_with(':') => Sect::WildcardPath,
            Sect::QueryParamValue(p) if p.starts_with(':') => Sect::WildcardQueryParamValue,
            _ => self,
        }
    }

    fn matches_input(&self, input: &Self) -> bool {
        use Sect::*;
        match (self, input) {
            (a, b) if a == b => true,
            (WildcardDomain, Domain(_)) => true,
            (WildcardPath, Path(_)) => true,
            (WildcardQueryParamValue, QueryParamValue(_)) => true,
            // As a special case if the path section is a `FreeMove` then always accept it.
            (FreeMove, _) => true,
            _ => false,
        }
    }
}

/// Split and URL into sections that we care about. This is effectively a tokenizer.
fn split_url(url: &str) -> Result<Vec<Sect>, UrlParseError> {
    if url.is_empty() {
        return Err(UrlParseError::EmptyUrl);
    }

    let mut res = Vec::new();

    if !url.contains(':') {
        res.push(Sect::Protocol(url.into()));
        return Ok(res);
    }

    let url: url::Url = url.parse().map_err(|e| UrlParseError::InvalidUrl {
        url: url.to_string(),
        source: e,
    })?;

    let proto = url.scheme();
    res.push(Sect::Protocol(proto.into()));

    if let Some(host) = url.host_str() {
        let mut host_parts: Vec<&str> = host.split('.').rev().collect();

        if (proto == "http" || proto == "https")
            && host_parts.last().is_some_and(|last| *last == "www")
        {
            // ignore a "www." at the beginning of the domain. The domain has been reversed so we're popping the last element
            let _www = host_parts.pop();
        }

        for part in host_parts {
            res.push(Sect::Domain(part.into()));
        }
    }

    if url.cannot_be_a_base() {
        res.push(Sect::Path(url.path().into()))
    } else if let Some(path_parts) = url.path_segments() {
        for part in path_parts {
            if part.is_empty() {
                continue;
            }
            res.push(Sect::Path(part.into()));
        }
    }

    for (k, v) in url.query_pairs() {
        res.push(Sect::QueryParamName(k.into()));
        if !v.is_empty() {
            res.push(Sect::QueryParamValue(v.into()));
        }
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate std;
    use std::{eprintln, vec};

    #[test]
    fn matching() {
        let mut resolver = Resolver::default();

        resolver.insert_protocol("near", "near").unwrap();
        resolver
            .insert_pattern("near-account", "near://account/:id")
            .unwrap();
        resolver.insert_pattern("near-tx", "near://tx/:id").unwrap();
        resolver
            .insert_prefix("google", "https://google.com/search?q=")
            .unwrap();
        resolver.insert_prefix("x", "https://x.com/").unwrap();
        resolver
            .insert_pattern("linkedin", "https://*.linkedin.com/in/:account/test")
            .unwrap();
        resolver
            .insert_pattern("youtube", "https://youtube.com/watch?v=:v")
            .unwrap();
        resolver
            .insert_pattern("subdomains", "https://*.baz.com/")
            .unwrap();
        resolver.insert_prefix("data", "data:text/plain").unwrap();
        resolver.insert_prefix("fs", "file://").unwrap();
        resolver.insert_prefix("fs2", "file:///2").unwrap();
        resolver.insert_file_extension("txt-ext", "txt").unwrap();
        resolver.insert_file_extension("tar-ext", "tar.gz").unwrap();

        eprintln!("{resolver:#?}");

        let tests = vec![
            ("near", "near"),
            ("near://tx/1234", "near-tx"),
            ("near://account/1234", "near-account"),
            ("near://other/1234", "near"),
            ("https://google.com/search?q=foobar", "google"),
            ("https://x.com/foobar", "x"),
            ("https://www.linkedin.com/in/foobar/test", "linkedin"),
            ("https://youtube.com/watch?v=foobar", "youtube"),
            ("https://multiple.subdomains.foo.bar.baz.com/", "subdomains"),
            ("data:text/plain?Hello+World", "data"),
            ("file:///foo/bar/baz", "fs"),
            ("file:///2/foo", "fs2"),
            ("file:///foobar.txt", "txt-ext"),
            ("file:///foobar.tar.gz", "tar-ext"),
        ];

        for (input, want) in tests {
            assert_eq!(
                resolver
                    .resolve(input)
                    .expect("resolve succeeds")
                    .iter()
                    .find(|out| out.name == want)
                    .unwrap_or_else(|| panic!(
                        "the wanted result should be returned, input={input} want={want}"
                    ))
                    .name,
                want
            );
        }
    }

    #[test]
    fn prefix_doesnt_turn_pattern_to_prefix() {
        let mut resolver = Resolver::new();

        resolver
            .insert_pattern("pattern", "https://foobar.com/")
            .unwrap();
        eprintln!("{resolver:#?}");

        let results = resolver.resolve("https://foobar.com/").unwrap();
        eprintln!("{results:?}");
        assert!(
            results
                .first()
                .is_some_and(|module| module.name == "pattern"),
            "the pattern should match"
        );

        let results = resolver.resolve("https://foobar.com/more").unwrap();
        eprintln!("{results:?}");
        assert!(results.is_empty(), "the pattern shouldn't be a prefix");

        resolver
            .insert_prefix("prefix", "https://foobar.com/")
            .unwrap();
        eprintln!("{resolver:#?}");

        let results = resolver.resolve("https://foobar.com/").unwrap();
        eprintln!("{results:?}");
        assert!(results.len() == 2, "both items should match");

        let results = resolver.resolve("https://foobar.com/more").unwrap();
        eprintln!("{results:?}");
        assert!(results.len() == 1, "only the prefix should match");
        assert!(
            results
                .first()
                .is_some_and(|module| module.name == "prefix"),
            "only the prefix should match"
        );
    }
}
