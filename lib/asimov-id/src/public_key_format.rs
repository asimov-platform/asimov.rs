// This is free and unencumbered software released into the public domain.

use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, Default, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PublicKeyFormat {
    #[default]
    Asimov,
    Base58,
    Base64,
    Base64Url,
    Hex,
    Jwk,
    Near,
    OpenSsh,
    Pem,
    Z32,
}
