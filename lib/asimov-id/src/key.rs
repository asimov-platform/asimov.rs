// This is free and unencumbered software released into the public domain.

use core::ops::RangeInclusive;

pub const KEY_LEN_MIN: usize = 1 + 32;
pub const KEY_LEN_MAX: usize = 1 + 44;
pub const KEY_LEN: RangeInclusive<usize> = KEY_LEN_MIN..=KEY_LEN_MAX;

#[deprecated(since = "25.3", note = "use `PublicKey` instead")]
pub type Key = crate::PublicKey; // TODO: remove in 26.0
