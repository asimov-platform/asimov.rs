// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{collections::btree_map::BTreeMap, string::String, vec, vec::Vec};
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
#[builder(on(String, into))]
pub struct RunnerOptions {
    /// Define key/value pairs.
    #[builder(default, with = |k: &str, v: &str| vec![BTreeMap::new()])] // TODO
    pub define: Vec<BTreeMap<String, String>>,

    /// Extended nonstandard runner options.
    #[builder(default)]
    pub other: Vec<String>,
}
