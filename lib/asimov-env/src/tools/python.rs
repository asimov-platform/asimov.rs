// This is free and unencumbered software released into the public domain.

use std::borrow::Cow;

pub fn python() -> Option<Cow<'static, str>> {
    clientele::envs::python()
        .map(Cow::from)
        .or_else(|| Some(Cow::from("python3")))
        .or_else(|| Some(Cow::from("python")))
}
