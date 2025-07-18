// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};
use typed_builder::TypedBuilder;

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
/// let options = RunnerOptions::builder().build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, TypedBuilder)]
pub struct RunnerOptions {
    /// Define key/value pairs.
    pub define: Vec<BTreeMap<String, String>>,

    /// Extended nonstandard program options.
    pub other: Vec<String>,
}
