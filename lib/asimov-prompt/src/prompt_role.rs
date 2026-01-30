// This is free and unencumbered software released into the public domain.

use derive_more::{Display, From, FromStr};
use dogma::{
    prelude::{Cow, String, ToString},
    traits::{Labeled, Named},
};

#[derive(Clone, Debug, Display, Eq, From, FromStr, Hash, Ord, PartialEq, PartialOrd)]
pub enum PromptRole {
    #[display("system")]
    System,

    #[display("developer")]
    Developer,

    #[display("user")]
    User,

    #[display("assistant")]
    Assistant,
}

impl Named for PromptRole {
    fn name(&self) -> Cow<'_, str> {
        self.to_string().into()
    }
}

impl Labeled for PromptRole {
    fn label(&self) -> Cow<'_, str> {
        use PromptRole::*;
        Cow::from(match self {
            System => "System",
            Developer => "Developer",
            User => "User",
            Assistant => "Assistant",
        })
    }
}

impl Into<String> for PromptRole {
    fn into(self) -> String {
        self.to_string()
    }
}
