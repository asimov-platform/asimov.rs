// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod handle;
pub use handle::*;

mod handle_error;
pub use handle_error::*;

mod key;
pub use key::*;

mod key_error;
pub use key_error::*;

mod id;
pub use id::*;

mod id_error;
pub use id_error::*;

mod public_key;
pub use public_key::*;

mod secret_key;
pub use secret_key::*;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use core::iter::repeat;
    use core::str::FromStr;

    #[test]
    fn parse_handle_jhacker() {
        let h = Handle::from_str("jhacker").expect("failed to parse handle");
        assert_eq!(h.as_str(), "jhacker");
        assert_eq!(h.to_string_with_glyph(), "Ⓐjhacker");
    }

    #[test]
    fn parse_public_key_ones() {
        let s = "ⒶY11111111111111111111111111111111";
        let pk = PublicKey::from_str(s).expect("failed to parse public key");
        let expected = [0u8; 32];
        assert_eq!(pk.as_bytes(), expected.as_slice());
    }

    // Negative tests for handles

    #[test]
    fn handle_empty_rejected() {
        assert_eq!(Handle::from_str("").unwrap_err(), HandleError::EmptyInput);
    }

    #[test]
    fn handle_invalid_first_char() {
        assert_eq!(
            Handle::from_str("-foo").unwrap_err(),
            HandleError::InvalidFirstChar('-')
        );
    }

    #[test]
    fn handle_invalid_char() {
        assert_eq!(
            Handle::from_str("bad!name").unwrap_err(),
            HandleError::InvalidChar('!')
        );
    }

    #[test]
    fn handle_too_long() {
        let s: String = repeat('a').take(HANDLE_LEN_MAX + 1).collect();
        assert_eq!(
            Handle::from_str(&s).unwrap_err(),
            HandleError::InvalidLength(s.len())
        );
    }

    // Negative tests for public keys
    #[test]
    fn public_key_empty_rejected() {
        assert_eq!(PublicKey::from_str("").unwrap_err(), KeyError::EmptyInput);
    }

    #[test]
    fn public_key_missing_glyph() {
        // valid length but missing leading glyph
        let s: String = repeat('1').take(PUBLIC_KEY_LEN_MIN).collect();
        let input = s; // no glyph
        assert_eq!(
            PublicKey::from_str(&input).unwrap_err(),
            KeyError::InvalidPrefix
        );
    }

    #[test]
    fn public_key_short_length() {
        let s = String::from("ⒶY11");
        assert_eq!(
            PublicKey::from_str(&s).unwrap_err(),
            KeyError::InvalidLength
        );
    }

    #[test]
    fn public_key_invalid_encoding() {
        // length is within allowed range but contains invalid base58 characters (e.g. '0')
        let mut s = String::from("ⒶY");
        s.push_str(&"0".repeat(PUBLIC_KEY_LEN_MIN - 1));
        assert!(matches!(
            PublicKey::from_str(&s).unwrap_err(),
            KeyError::InvalidEncoding(_)
        ));
    }
}
