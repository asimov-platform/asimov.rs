// This is free and unencumbered software released into the public domain.

use crate::error::AccountBalanceError;
use asimov_credit::Credits;
use core::{error::Error, str::FromStr};
use derive_more::Display;

pub use asimov_id::{Id, IdError};

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[display("@{}", self.0)]
pub struct Account(pub(crate) Id);

impl FromStr for Account {
    type Err = IdError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.parse()?))
    }
}

impl Account {
    pub async fn open(id: Id) -> Result<Self, Box<dyn Error>> {
        Ok(Self(id))
    }

    pub async fn has_credits(&self) -> Result<bool, AccountBalanceError> {
        Ok(self.balance().await? > Credits::ZERO)
    }

    pub async fn balance(&self) -> Result<Credits, AccountBalanceError> {
        let client = reqwest::Client::builder().user_agent("ASIMOV.rs").build()?;

        let url = format!("https://asimov.credit/{}/balance.txt", self.0);
        let response = client.get(url).send().await?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(AccountBalanceError::UnexpectedResponse(response.status()));
        }

        let response_text = response.text().await?;

        Ok(response_text.parse::<Credits>()?)
    }
}
