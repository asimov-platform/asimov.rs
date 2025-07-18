// This is free and unencumbered software released into the public domain.

//! ASIMOV Program Patterns
//!
//! See: https://asimov-specs.github.io/program-patterns/

#![allow(unused_imports)]

mod adapter;
pub use adapter::*;

mod cataloger;
pub use cataloger::*;

mod emitter;
pub use emitter::*;

mod fetcher;
pub use fetcher::*;

mod indexer;
pub use indexer::*;

mod prompter;
pub use prompter::*;

mod reader;
pub use reader::*;

mod reasoner;
pub use reasoner::*;

mod resolver;
pub use resolver::*;

mod runner;
pub use runner::*;

mod writer;
pub use writer::*;
