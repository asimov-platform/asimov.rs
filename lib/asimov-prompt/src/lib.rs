// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use asimov_prompt::*;
//! ```

#![no_std]

mod prompt;
pub use prompt::*;

mod prompt_message;
pub use prompt_message::*;

mod prompt_role;
pub use prompt_role::*;
