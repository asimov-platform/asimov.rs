// This is free and unencumbered software released into the public domain.

use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, Default, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PublicKeyEncoding {
    #[default]
    Asimov,

    Base58,

    #[cfg(feature = "base64")]
    Base64,

    #[cfg(feature = "base64")]
    Base64Url,

    #[cfg(feature = "hex")]
    Hex,

    Jwk,

    Near,

    OpenSsh,

    Pem,

    #[cfg(feature = "z32")]
    Z32,
}

#[cfg(any(feature = "base64", feature = "hex", feature = "z32"))]
impl TryFrom<PublicKeyEncoding> for data_encoding::Encoding {
    type Error = ();

    fn try_from(input: PublicKeyEncoding) -> Result<Self, Self::Error> {
        Ok(match input {
            PublicKeyEncoding::Asimov => return Err(()),
            PublicKeyEncoding::Base58 => return Err(()),
            PublicKeyEncoding::Base64 => data_encoding::BASE64,
            PublicKeyEncoding::Base64Url => data_encoding::BASE64URL_NOPAD,
            PublicKeyEncoding::Hex => data_encoding::HEXLOWER,
            PublicKeyEncoding::Jwk => return Err(()),
            PublicKeyEncoding::Near => return Err(()),
            PublicKeyEncoding::OpenSsh => return Err(()),
            PublicKeyEncoding::Pem => return Err(()),
            PublicKeyEncoding::Z32 => data_encoding::BASE32,
        })
    }
}
