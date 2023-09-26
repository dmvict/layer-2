use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct EmailWebsite {
  pub email: String,
  pub website: String,
}

pub enum EmailTemplates {
  SayHi,
  Receipt,
}

impl From<EmailTemplates> for String {
  fn from(value: EmailTemplates) -> Self {
    value.to_string()
  }
}

impl AsRef<str> for EmailTemplates {
  fn as_ref(&self) -> &str {
    match self {
      EmailTemplates::SayHi => "say_hi",
      EmailTemplates::Receipt => "receipt",
    }
  }
}

impl Display for EmailTemplates {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      EmailTemplates::SayHi => "say_hi_id",
      EmailTemplates::Receipt => "receipt",
    })
  }
}
