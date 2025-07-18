// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{
    borrow::ToOwned,
    collections::btree_map::BTreeMap,
    string::String,
    vec::{self, Vec},
};
use bon::Builder;

/// Language runtime engine. Consumes text input conforming to a grammar,
/// executes it, and produces the execution result as output.
///
/// See: https://asimov-specs.github.io/program-patterns/#runner
pub trait Runner<T, E>: Execute<T, E> {}

/// Configuration options for [`Runner`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::RunnerOptions;
///
/// let options = RunnerOptions::builder()
///     .define("my_key", "my_value")
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct RunnerOptions {
    /// Extended nonstandard runner options.
    #[builder(field)]
    pub other: Vec<String>,

    /// Define key/value pairs.
    #[builder(field)]
    pub define: BTreeMap<String, String>,
}

impl<S: runner_options_builder::State> RunnerOptionsBuilder<S> {
    pub fn other(mut self, flag: impl Into<String>) -> Self {
        self.other.push(flag.into());
        self
    }

    pub fn maybe_other(mut self, flag: Option<impl Into<String>>) -> Self {
        if let Some(flag) = flag {
            self.other.push(flag.into());
        }
        self
    }

    pub fn define(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.define.insert(key.into(), val.into());
        self
    }
}
