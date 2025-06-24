// This is free and unencumbered software released into the public domain.

use crate::models::ModuleManifest;
use alloc::{
    boxed::Box,
    collections::{btree_map::BTreeMap, btree_set::BTreeSet},
    format,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::{borrow::Borrow, error::Error};

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

    pub fn resolve(&self, url: &str) -> Result<Vec<Rc<Module>>, Box<dyn Error>> {
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

        // Initialize with root states that match the first input
        let root_states: BTreeSet<usize> =
            BTreeSet::from_iter(self.roots.iter().filter_map(|(pattern, &idx)| {
                if pattern.matches_input(&input[0]) {
                    Some(idx)
                } else {
                    None
                }
            }));

        // Process remaining input
        let final_states = input[1..].iter().fold(root_states, |states, sect| {
            if states.is_empty() {
                return states;
            }

            states
                .iter()
                .flat_map(|&node_idx| &self.nodes[node_idx].paths)
                .filter_map(|(path, &next_idx)| {
                    if path.matches_input(sect) {
                        Some(next_idx)
                    } else {
                        None
                    }
                })
                .collect()
        });

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
    ) -> Result<(), Box<dyn Error>> {
        let module = self.add_module(module);

        self.file_extensions
            .entry(file_extension.to_string())
            .or_default()
            .push(module);

        Ok(())
    }
    pub fn insert_manifest(&mut self, manifest: &ModuleManifest) -> Result<(), Box<dyn Error>> {
        for protocol in &manifest.handles.url_protocols {
            self.insert_protocol(&manifest.name, protocol)?;
        }
        for prefix in &manifest.handles.url_prefixes {
            self.insert_prefix(&manifest.name, prefix)?;
        }
        for pattern in &manifest.handles.url_patterns {
            self.insert_pattern(&manifest.name, pattern)?;
        }
        for file_extension in &manifest.handles.file_extensions {
            self.insert_file_extension(&manifest.name, file_extension)?;
        }
        Ok(())
    }
    pub fn insert_protocol(&mut self, module: &str, protocol: &str) -> Result<(), Box<dyn Error>> {
        let module = self.add_module(module);
        let node_idx = self.get_or_create_node(&[Sect::Protocol(protocol.to_string())]);

        // add a free move back to self (represent a protocol as an prefix):
        self.nodes[node_idx].paths.insert(Sect::Any, node_idx);
        self.nodes[node_idx].modules.insert(module);

        Ok(())
    }
    pub fn insert_prefix(&mut self, module: &str, prefix: &str) -> Result<(), Box<dyn Error>> {
        let path = split_url(prefix)?;
        let module = self.add_module(module);
        let node_idx = self.get_or_create_node(&path);

        // add a free move back to self:
        // TODO: there is a possibility that non-prefixes have the same end node
        // and that this collides with them, causing the others to also be
        // turned into prefixes
        self.nodes[node_idx].paths.insert(Sect::Any, node_idx);
        self.nodes[node_idx].modules.insert(module);

        Ok(())
    }
    pub fn insert_pattern(&mut self, module: &str, pattern: &str) -> Result<(), Box<dyn Error>> {
        let path: Vec<Sect> = split_url(pattern)?
            .into_iter()
            .map(Sect::into_pattern)
            .collect();
        let module = self.add_module(module);
        let node_idx = self.get_or_create_node(&path);

        self.nodes[node_idx].modules.insert(module);

        Ok(())
    }

    pub fn try_from_iter<I, T>(mut iter: I) -> Result<Self, Box<dyn Error>>
    where
        I: Iterator<Item = T>,
        T: Borrow<ModuleManifest>,
    {
        iter.try_fold(Resolver::default(), |mut b, m| {
            b.insert_manifest(m.borrow())?;
            Ok(b)
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
                }
                (None, sect) => {
                    // Create a new node
                    let new_node_idx = self.nodes.insert(Node::default());

                    // Add the transition from current node to new node
                    self.nodes[cur_idx].paths.insert(sect.clone(), new_node_idx);
                    new_node_idx
                }
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
    type Error = Box<dyn Error>;

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
    Protocol(String),
    Domain(String),
    WildcardDomain,
    Path(String),
    WildcardPath,
    QueryParamName(String),
    QueryParamValue(String),
    WildcardQueryParamValue,
    Any,
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
            (Any, _) => true,
            _ => false,
        }
    }
}

/// Split and URL into sections that we care about. This is effectively a tokenizer.
fn split_url(url: &str) -> Result<Vec<Sect>, Box<dyn Error>> {
    if url.is_empty() {
        return Err("URL can not be empty".into());
    }

    let mut res = Vec::new();

    if !url.contains(':') {
        res.push(Sect::Protocol(url.into()));
        return Ok(res);
    }

    let url: url::Url = url
        .parse()
        .map_err(|e| format!("Unable to handle URL {url:?}: {e}"))?;

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

        eprintln!("{resolver:?}");

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
}
