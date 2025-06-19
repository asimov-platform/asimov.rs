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
use trie_rs::{
    inc_search::{IncSearch, Position},
    map::{Trie, TrieBuilder},
};

#[derive(Clone, Debug)]
pub struct Resolver {
    trie: Trie<Sect, Vec<Rc<Module>>>,
}

impl Resolver {
    pub fn resolve(&self, url: &str) -> Result<Vec<Rc<Module>>, Box<dyn Error>> {
        Ok(self.find(url)?.collect())
    }

    pub fn find(&self, url: &str) -> Result<impl Iterator<Item = Rc<Module>>, Box<dyn Error>> {
        Ok(SearchIter {
            trie: &self.trie,
            input_idx: 0,
            input: split_url(url)?,
            items: &[],
            save_stack: Vec::new(),
            search: self.trie.inc_search(),
            unique: BTreeSet::new(),
        })
    }

    pub fn try_from_iter<I, T>(iter: I) -> Result<Self, Box<dyn Error>>
    where
        I: Iterator<Item = T>,
        T: Borrow<ModuleManifest>,
    {
        ResolverBuilder::try_from_iter(iter)?.build()
    }
}

impl TryFrom<&[ModuleManifest]> for Resolver {
    type Error = Box<dyn Error>;

    fn try_from(value: &[ModuleManifest]) -> Result<Self, Self::Error> {
        ResolverBuilder::try_from(value)?.build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Module {
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct ResolverBuilder {
    modules: BTreeMap<String, Rc<Module>>,
    protocol_modules: BTreeMap<String, Vec<Rc<Module>>>,
    pattern_modules: BTreeMap<String, Vec<Rc<Module>>>,
    prefix_modules: BTreeMap<String, Vec<Rc<Module>>>,
}

impl ResolverBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Resolver, Box<dyn Error>> {
        let mut trie = TrieBuilder::new();
        for (k, v) in self.prefix_modules {
            let k = split_url(&k)?;
            trie.push(k, v);
        }
        for (k, v) in self.protocol_modules {
            let k = Sect::Protocol(k);
            trie.push([k], v);
        }
        for (k, v) in self.pattern_modules {
            let k = split_url(&k)?.into_iter().map(Sect::into_pattern);
            trie.insert(k, v);
        }
        let trie = trie.build();

        Ok(Resolver { trie })
    }

    pub fn insert_protocol(&mut self, module: &str, protocol: &str) -> Result<(), Box<dyn Error>> {
        let module = self.add_module(module);
        let mods = self
            .protocol_modules
            .entry(protocol.to_string())
            .or_default();
        mods.push(module);
        Ok(())
    }
    pub fn insert_prefix(&mut self, module: &str, prefix: &str) -> Result<(), Box<dyn Error>> {
        let _ = split_url(prefix)?;
        let module = self.add_module(module);
        let mods = self.prefix_modules.entry(prefix.to_string()).or_default();
        mods.push(module.clone());
        Ok(())
    }
    pub fn insert_pattern(&mut self, module: &str, pattern: &str) -> Result<(), Box<dyn Error>> {
        let _ = split_url(pattern)?;
        let module = self.add_module(module);
        let mods = self.pattern_modules.entry(pattern.to_string()).or_default();
        mods.push(module.clone());
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
        Ok(())
    }

    pub fn try_from_iter<I, T>(mut iter: I) -> Result<Self, Box<dyn Error>>
    where
        I: Iterator<Item = T>,
        T: Borrow<ModuleManifest>,
    {
        iter.try_fold(ResolverBuilder::new(), |mut b, m| {
            b.insert_manifest(m.borrow())?;
            Ok(b)
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

impl TryFrom<&[ModuleManifest]> for ResolverBuilder {
    type Error = Box<dyn Error>;

    fn try_from(value: &[ModuleManifest]) -> Result<Self, Self::Error> {
        Self::try_from_iter(value.iter())
    }
}

struct SearchIter<'r> {
    trie: &'r Trie<Sect, Vec<Rc<Module>>>,
    input_idx: usize,
    input: Vec<Sect>,
    items: &'r [Rc<Module>],
    save_stack: Vec<(Position, usize)>,
    search: IncSearch<'r, Sect, Vec<Rc<Module>>>,
    unique: BTreeSet<Rc<Module>>,
}

impl<'r> Iterator for SearchIter<'r> {
    type Item = Rc<Module>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((first, rest)) = self.items.split_first() {
            self.items = rest;
            if self.unique.insert(first.clone()) {
                return Some(first.clone());
            }
        }

        loop {
            // Try to get current part or backtrack
            let part = loop {
                if let Some(part) = self.input.get(self.input_idx) {
                    break part;
                }

                // No more input, try to backtrack
                if let Some(save_state) = self.save_stack.pop() {
                    // Restore saved state
                    self.search = IncSearch::resume(self.trie, save_state.0);
                    self.input_idx = save_state.1;

                    // Check if the resumed state has values to return
                    if let Some(cur) = self.search.value() {
                        self.items = cur;
                        while let Some((first, rest)) = self.items.split_first() {
                            self.items = rest;
                            if self.unique.insert(first.clone()) {
                                return Some(first.clone());
                            }
                        }
                    }

                    // otherwise continue consuming input from the resumed state
                    continue;
                };

                return None; // No more save states, we're done
            };

            // Try different matching strategies based on the part type
            let answer = match part {
                Sect::Protocol(_) => self.search.query(part),
                Sect::Domain(_) => {
                    let answer = self.search.query(part);

                    // *after* matching the current domain section try to match
                    // a wildcard domain. If it matches, consume inputs as
                    // long as there are domain sections.
                    let mut search = self.search.clone();
                    if search.query(&Sect::WildcardDomain).is_some() {
                        let mut n = 1;
                        while self
                            .input
                            .get(self.input_idx + n)
                            .is_some_and(|i| matches!(i, Sect::Domain(_)))
                        {
                            n += 1;
                        }

                        // save a state with (matched wildcard, all subdomains consumed)
                        let pos = Position::from(search);
                        self.save_stack.push((pos, self.input_idx + n));
                    }

                    answer
                }
                Sect::Path(_) => {
                    {
                        let mut search = self.search.clone();
                        if search.query(&Sect::WildcardPath).is_some() {
                            // We matched a wildcard path element.
                            // Save the position that represents a consumed input.
                            let pos = Position::from(search);
                            self.save_stack.push((pos, self.input_idx + 1));
                        }
                    }
                    self.search.query(part)
                }
                Sect::QueryParamName(_) => self.search.query(part),
                Sect::QueryParamValue(_) => {
                    {
                        let mut search = self.search.clone();
                        if search.query(&Sect::WildcardQueryParamValue).is_some() {
                            let pos = Position::from(search);
                            self.save_stack.push((pos, self.input_idx + 1));
                        };
                    };
                    self.search.query(part)
                }
                _ => unreachable!(),
            };

            self.input_idx += 1;

            if !answer.is_some_and(|a| a.is_prefix()) {
                // Current node is not a prefix, i.e. complete match found.
                // Consume remaining input.
                self.input_idx = self.input.len();
            }

            // Check if current node has values (could use `answer.is_match()`).
            if let Some(cur) = self.search.value() {
                self.items = cur;
                while let Some((first, rest)) = self.items.split_first() {
                    self.items = rest;
                    if self.unique.insert(first.clone()) {
                        return Some(first.clone());
                    }
                }
            }
        }
    }
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
}

/// Split and URL into sections that we care about. This is effectively a tokenizer.
fn split_url(url: &str) -> Result<Vec<Sect>, Box<dyn Error>> {
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
    } else {
        if let Some(path_parts) = url.path_segments() {
            for part in path_parts {
                if part.is_empty() {
                    continue;
                }
                res.push(Sect::Path(part.into()));
            }
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
        let mut builder = ResolverBuilder::default();

        builder.insert_protocol("near", "near").unwrap();
        builder
            .insert_pattern("near-account", "near://account/:id")
            .unwrap();
        builder.insert_pattern("near-tx", "near://tx/:id").unwrap();
        builder
            .insert_prefix("google", "https://google.com/search?q=")
            .unwrap();
        builder.insert_prefix("x", "https://x.com/").unwrap();
        builder
            .insert_pattern("linkedin", "https://*.linkedin.com/in/:account/test")
            .unwrap();
        builder
            .insert_pattern("youtube", "https://youtube.com/watch?v=:v")
            .unwrap();
        builder
            .insert_pattern("subdomains", "https://*.baz.com/")
            .unwrap();
        builder.insert_pattern("data", "data:text/plain").unwrap();
        builder.insert_pattern("fs", "file://").unwrap();
        builder.insert_pattern("fs2", "file:///2").unwrap();

        let resolver = builder.build().expect("resolver should build");

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
        ];

        for (input, want) in tests {
            assert_eq!(
                resolver
                    .find(input)
                    .expect("resolve succeeds")
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
