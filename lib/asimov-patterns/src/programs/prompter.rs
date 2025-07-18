// This is free and unencumbered software released into the public domain.

use crate::Execute;
use alloc::{string::String, vec::Vec};
use bon::Builder;

/// LLM inference provider. Consumes prompt input, produces response output.
///
/// See: https://asimov-specs.github.io/program-patterns/#prompter
pub trait Prompter<T, E>: Execute<T, E> {}

/// Configuration options for [`Prompter`].
///
/// # Examples
///
/// ```rust
/// use asimov_patterns::PrompterOptions;
///
/// let options = PrompterOptions::builder()
///     .input("text")
///     .output("text")
///     .model("gemma3:1b")
///     .build();
///
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Builder)]
#[builder(on(String, into))]
pub struct PrompterOptions {
    /// The input format.
    pub input: Option<String>,

    /// The inference model.
    pub model: Option<String>,

    /// The output format.
    pub output: Option<String>,

    /// Extended nonstandard prompter options.
    #[builder(default)]
    pub other: Vec<String>,
}
