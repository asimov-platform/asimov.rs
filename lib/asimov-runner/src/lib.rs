// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use asimov_runner::*;
//! ```

pub use asimov_patterns::Execute;
pub use clientele::SysexitsError;
pub use tokio::process::Command;

mod adapter;
pub use adapter::*;

mod fetcher;
pub use fetcher::*;

mod importer;
pub use importer::*;

mod provider;
pub use provider::*;

mod reader;
pub use reader::*;

mod reasoner;
pub use reasoner::*;

mod registrar;
pub use registrar::*;

mod runner;
pub use runner::*;

mod writer;
pub use writer::*;
