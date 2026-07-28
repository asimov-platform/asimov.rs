// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod blob_id;
pub use blob_id::*;

mod event_id;
pub use event_id::*;

mod id;
pub use id::*;

mod id_class;
pub use id_class::*;

mod id_error;
pub use id_error::*;

mod organization_id;
pub use organization_id::*;

mod person_id;
pub use person_id::*;
