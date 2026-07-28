// This is free and unencumbered software released into the public domain.

use core::ops::RangeInclusive;

pub const PUBLIC_KEY_PREFIX: &str = "ⒶY";

pub const PUBLIC_KEY_LEN_MIN: usize = 3 + 1 + 32; // "ⒶY"
pub const PUBLIC_KEY_LEN_MAX: usize = 3 + 1 + 44;
pub const PUBLIC_KEY_LEN: RangeInclusive<usize> = PUBLIC_KEY_LEN_MIN..=PUBLIC_KEY_LEN_MAX;

#[deprecated(since = "25.3", note = "use `PublicKey` instead")]
pub type Key = crate::PublicKey; // TODO: remove in 26.0
