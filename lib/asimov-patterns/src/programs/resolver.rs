// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

/// URI resolver. Takes a URI (that is, URN or URL) input, produces a resolved URL output.
///
/// See: https://asimov-specs.github.io/program-patterns/#resolver
pub trait Resolver<T, E>: Execute<T, E> {}

/// Configuration options for [`Resolver`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::ResolverOptions;
///
/// let options = ResolverOptions::builder()
///     .limit(100)
///     .build();
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(derive(Debug), on(String, into))]
pub struct ResolverOptions {
    /// Extended nonstandard resolver options.
    #[builder(field)]
    pub other: Vec<String>,

    /// The maximum number of outputs.
    pub limit: Option<usize>,
}

impl<S: resolver_options_builder::State> ResolverOptionsBuilder<S> {
    pub fn other(mut self, flag: impl Into<String>) -> Self {
        self.other.push(flag.into());
        self
    }
}
