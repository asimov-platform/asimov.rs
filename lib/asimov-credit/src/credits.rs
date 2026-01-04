// This is free and unencumbered software released into the public domain.

use core::str::FromStr;
use derive_more::Display;
use rust_decimal::{Decimal, prelude::ToPrimitive};

#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[display("{:.9}", self.0)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Credits(pub(crate) Decimal);

impl Credits {
    pub const ZERO: Credits = Self(Decimal::ZERO);
    pub const ONE: Credits = Self(Decimal::ONE);
    pub const NEGATIVE_ONE: Credits = Self(Decimal::NEGATIVE_ONE);

    pub fn from_nanos(nanos: i64) -> Self {
        Self(Decimal::new(nanos, 9))
    }

    pub fn as_nanos(&self) -> i64 {
        let mut result = self.0.clone();
        result.rescale(0);
        result.to_i64().unwrap()
    }

    pub fn as_decimal(&self) -> &Decimal {
        &self.0
    }

    pub fn into_decimal(self) -> Decimal {
        self.0
    }

    pub fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl From<Decimal> for Credits {
    fn from(amount: Decimal) -> Self {
        Self(amount)
    }
}

impl From<i64> for Credits {
    fn from(amount: i64) -> Self {
        Self(amount.into())
    }
}

impl From<u64> for Credits {
    fn from(amount: u64) -> Self {
        Self(amount.into())
    }
}

impl FromStr for Credits {
    type Err = rust_decimal::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Decimal::from_str(input).map(Self)
    }
}

impl TryFrom<String> for Credits {
    type Error = rust_decimal::Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(&input)
    }
}

impl Into<String> for Credits {
    fn into(self) -> String {
        self.to_string()
    }
}
