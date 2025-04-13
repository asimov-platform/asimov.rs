// This is free and unencumbered software released into the public domain.

use std::borrow::Cow;

pub fn ruby() -> Option<Cow<'static, str>> {
    clientele::envs::ruby()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("ruby")))
}

pub fn gem() -> Option<Cow<'static, str>> {
    clientele::envs::gem()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("gem")))
}
