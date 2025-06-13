// This is free and unencumbered software released into the public domain.

use alloc::{
    collections::btree_map::BTreeMap,
    format,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use trie_rs::{
    inc_search::{IncSearch, Position},
    map::{Trie, TrieBuilder},
};

#[derive(Clone, Debug)]
pub struct Resolver {
    pattern_trie: Trie<Sect, Vec<Rc<Module>>>,
    prefix_trie: Trie<u8, Vec<Rc<Module>>>,
}

impl Resolver {
    pub fn resolve(&self, url: &str) -> Result<Vec<Rc<Module>>, ()> {
        Ok(self.find(url)?.collect())
    }

    pub fn find<'r, 'u>(&'r self, url: &'u str) -> Result<impl Iterator<Item = Rc<Module>>, ()> {
        Ok(SearchIter {
            resolver: self,
            input: url,
            items: &[],
            stage: SearchStage::Prefix {
                input: url.as_bytes(),
                search: self.prefix_trie.inc_search(),
            },
        })
    }
}

#[derive(Clone, Debug)]
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

    pub fn build(self) -> Resolver {
        let mut prefix_trie = TrieBuilder::new();
        for (k, v) in self.prefix_modules {
            prefix_trie.push(k, v);
        }
        let mut pattern_trie = TrieBuilder::new();
        for (k, v) in self.protocol_modules {
            let k = Sect::Protocol(k);
            pattern_trie.push([k], v);
        }
        for (k, v) in self.pattern_modules {
            let k = split_url(&k);
            pattern_trie.push(k, v);
        }

        Resolver {
            pattern_trie: pattern_trie.build(),
            prefix_trie: prefix_trie.build(),
        }
    }

    pub fn insert_protocol(&mut self, module: &str, protocol: &str) -> Result<(), ()> {
        let module = self.add_module(module);
        let mods = self
            .protocol_modules
            .entry(protocol.to_string())
            .or_default();
        mods.push(module);
        Ok(())
    }
    pub fn insert_prefix(&mut self, module: &str, prefix: &str) -> Result<(), ()> {
        let module = self.add_module(module);
        let mods = self.prefix_modules.entry(prefix.to_string()).or_default();
        mods.push(module.clone());
        Ok(())
    }
    pub fn insert_pattern(&mut self, module: &str, pattern: &str) -> Result<(), ()> {
        let module = self.add_module(module);
        let mods = self.pattern_modules.entry(pattern.to_string()).or_default();
        mods.push(module.clone());
        Ok(())
    }

    fn add_module(&mut self, name: &str) -> Rc<Module> {
        let name = name.to_string();
        self.modules
            .entry(name.clone())
            .or_insert_with(|| Rc::new(Module { name }))
            .clone()
    }
}

struct SearchIter<'r, 'a> {
    resolver: &'r Resolver,
    input: &'a str,
    items: &'r [Rc<Module>],
    stage: SearchStage<'r, 'a>,
}

enum SearchStage<'r, 'a> {
    Prefix {
        input: &'a [u8],
        search: IncSearch<'r, u8, Vec<Rc<Module>>>,
    },
    Pattern {
        input: Vec<Sect>,
        input_idx: usize,
        search: IncSearch<'r, Sect, Vec<Rc<Module>>>,
        save_stack: Vec<(Position, usize)>,
    },
}

impl<'r, 'a> Iterator for SearchIter<'r, 'a> {
    type Item = Rc<Module>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, rest)) = self.items.split_first() {
            self.items = rest;
            return Some(first.clone());
        }

        loop {
            match self.stage {
                SearchStage::Prefix {
                    ref mut input,
                    ref mut search,
                } => {
                    let Some((first, rest)) = input.split_first() else {
                        self.stage = SearchStage::Pattern {
                            input: split_url(self.input),
                            input_idx: 0,
                            search: self.resolver.pattern_trie.inc_search(),
                            save_stack: Vec::new(),
                        };
                        continue;
                    };
                    *input = &rest;

                    let Some(answer) = search.query(first) else {
                        continue;
                    };

                    if !answer.is_prefix() {
                        *input = &[];
                    }

                    if let Some(cur) = search.value() {
                        self.items = cur;
                        if let Some((first, rest)) = self.items.split_first() {
                            self.items = rest;
                            return Some(first.clone());
                        }
                    }
                }
                SearchStage::Pattern {
                    ref mut input,
                    ref mut input_idx,
                    ref mut search,
                    ref mut save_stack,
                } => {
                    // Try to get current part or backtrack
                    let part = loop {
                        if let Some(part) = input.get(*input_idx) {
                            break part;
                        }

                        // No more input, try to backtrack
                        if let Some(save_state) = save_stack.pop() {
                            // Restore saved state
                            *search = IncSearch::resume(&self.resolver.pattern_trie, save_state.0);
                            *input_idx = save_state.1;

                            // Check if the resumed state has values to return
                            if let Some(cur) = search.value() {
                                self.items = cur;
                                if let Some((first, rest)) = self.items.split_first() {
                                    self.items = rest;
                                    return Some(first.clone());
                                }
                            }

                            // otherwise continue consuming input from the resumed state
                            continue;
                        };

                        return None; // No more save states, we're done
                    };

                    // Try different matching strategies based on the part type
                    let answer = match part {
                        Sect::Protocol(_) => search.query(part),
                        Sect::Domain(_) => {
                            {
                                let mut search = search.clone();
                                if search.query(&Sect::WildcardDomain).is_some() {
                                    let pos = Position::from(search);
                                    save_stack.push((pos, *input_idx + 1));
                                }
                            };
                            search.query(part)
                        }
                        Sect::Path(_) => {
                            {
                                let mut search = search.clone();
                                if search.query(&Sect::WildcardPath).is_some() {
                                    // We matched a wildcard path element.
                                    // Save the position that represents a consumed input.
                                    let pos = Position::from(search);
                                    save_stack.push((pos, *input_idx + 1));
                                }
                            }
                            {
                                // TODO: multiple
                                let mut search = search.clone();
                                if input
                                    .get(*input_idx - 1)
                                    .is_some_and(|prev| matches!(prev, Sect::Domain(_)))
                                    && search.query(&Sect::WildcardDomain).is_some()
                                {
                                    // The previous input was a domain and we
                                    // matched a wildcard domain element.
                                    // Save the position that represents an *unconsumed* path input
                                    let pos = Position::from(search);
                                    save_stack.push((pos, *input_idx));
                                }
                            }
                            search.query(part)
                        }
                        Sect::Query(q) => {
                            if let Some((name, _)) = q.split_once('=') {
                                let mut search = search.clone();
                                if search.query(&Sect::WildcardQuery(name.into())).is_some() {
                                    let pos = Position::from(search);
                                    save_stack.push((pos, *input_idx + 1));
                                };
                            };
                            search.query(part)
                        }
                        _ => unreachable!(),
                    };

                    *input_idx += 1;

                    if let Some(answer) = answer {
                        if !answer.is_prefix() {
                            // Current node is not a prefix, i.e. complete match found.
                            // Consume remaining input.
                            *input_idx = input.len();
                        }

                        // Check if current node has values (could use `answer.is_match()`).
                        if let Some(cur) = search.value() {
                            self.items = cur;
                            if let Some((first, rest)) = self.items.split_first() {
                                self.items = rest;
                                return Some(first.clone());
                            }
                        }
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
    Query(String),
    WildcardQuery(String),
}

fn split_url(url: &str) -> Vec<Sect> {
    let mut res = Vec::new();

    let Some((proto, rest)) = url.split_once("://") else {
        res.push(Sect::Protocol(url.into()));
        return res;
    };
    res.push(Sect::Protocol(proto.into()));

    let Some((host, rest)) = rest.split_once('/') else {
        let path_parts = rest.split('/');
        for part in path_parts {
            if part.starts_with(':') {
                res.push(Sect::WildcardPath)
            } else {
                res.push(Sect::Path(part.into()));
            }
        }
        return res;
    };

    let mut host_parts = host.split('.').rev().collect::<Vec<&str>>();
    if proto == "http" || proto == "https" {
        // ignore a "www." at the beginning of the domain. The domain has been reversed so we're popping the last element
        let _www = host_parts.pop_if(|last| *last == "www");
    }

    for part in host_parts {
        if part == "*" {
            res.push(Sect::WildcardDomain)
        } else {
            res.push(Sect::Domain(part.into()));
        }
    }

    let Some((path, query)) = rest.split_once('?') else {
        let path_parts = rest.split('/');
        for part in path_parts {
            if part.starts_with(':') {
                res.push(Sect::WildcardPath)
            } else {
                res.push(Sect::Path(part.into()));
            }
        }
        return res;
    };

    let path_parts = path.split('/');
    for part in path_parts {
        if part.starts_with(':') {
            res.push(Sect::WildcardPath)
        } else {
            res.push(Sect::Path(part.into()));
        }
    }

    let query_parts = query.split('&');
    let mut query_parts: Vec<(&str, &str)> =
        query_parts.filter_map(|q| q.split_once('=')).collect();
    query_parts.sort_by_key(|(a, _b)| *a);
    for (k, v) in query_parts {
        if v.starts_with(":") {
            res.push(Sect::WildcardQuery(k.into()))
        } else {
            let param = format!("{}={}", k, v);
            res.push(Sect::Query(param));
        }
    }

    res
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
            .insert_prefix("google", "https://google.com/search?q=")
            .unwrap();
        builder
            .insert_pattern("linkedin", "https://*.linkedin.com/in/:account/test")
            .unwrap();
        builder
            .insert_pattern("youtube", "https://youtube.com/watch?v=:v")
            .unwrap();

        let resolver = builder.build();

        eprintln!("{resolver:?}");

        let tests = vec![
            ("near", "near"),
            ("near://", "near"),
            ("near://transaction/12345", "near"),
            ("https://google.com/search?q=foobar", "google"),
            ("https://www.linkedin.com/in/foobar/test", "linkedin"),
            ("https://youtube.com/watch?v=foobar", "youtube"),
        ];

        for (input, want) in tests {
            assert_eq!(
                resolver
                    .resolve(input)
                    .expect("resolve succeeds")
                    .first()
                    .expect("there should be at least one result")
                    .name,
                want
            );
        }
    }
}
