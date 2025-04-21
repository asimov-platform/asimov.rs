// This is free and unencumbered software released into the public domain.

use std::borrow::Cow;

pub fn cargo() -> Option<Cow<'static, str>> {
    getenv::cargo()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("cargo")))
}
