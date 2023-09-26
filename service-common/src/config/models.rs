use std::{collections::HashMap, fmt::Display};

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServiceVariable {
  AuthDbUrl,
  PaymentAppUrl,
  FilesmetaDbUrl,
  SelfUrl,
  SparkpostApiKey,
  CompanyEmail,
  CompanyName,
  PdfGeneratorAppUrl,
  StripeSecretKey,
  StripeWebhookSecret,
  ShouldSendEmail,
  EmailAppUrl,
  ProductProcessorAppUrl,
  UserDbUrl,
  UploadAppUrl,
  SmtpLogin,
  SmtpPassword,
  SmtpUrl,
  EmailProvider,
  TemplatesDbUrl,
  ChatDbUrl,
  IntentDbUrl,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ServiceName {
  DownloadApp,
  EmailApp,
  PaymentApp,
  ProductProcessorApp,
  UploadApp,
  UserApp,
  PdfApp,
  ChatEvo1,
}

impl From<ServiceName> for String {
  fn from(value: ServiceName) -> Self {
    value.to_string()
  }
}

impl Display for ServiceName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      ServiceName::DownloadApp => "download-app",
      ServiceName::EmailApp => "email-app",
      ServiceName::PaymentApp => "payment-app",
      ServiceName::ProductProcessorApp => "product-processor-app",
      ServiceName::UploadApp => "upload-app",
      ServiceName::UserApp => "user-app",
      ServiceName::PdfApp => "pdf-app",
      ServiceName::ChatEvo1 => "chat-evo1",
    })
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConfigVariable {
  String(String),
  Bool(bool),
  VecOfStrings(Vec<String>),
}

impl TryFrom<ConfigVariable> for String {
  type Error = anyhow::Error;

  fn try_from(value: ConfigVariable) -> Result<Self, Self::Error> {
    match value {
      ConfigVariable::String(value) => Ok(value),
      _ => bail!("Type mismatch"),
    }
  }
}

impl TryFrom<ConfigVariable> for bool {
  type Error = anyhow::Error;

  fn try_from(value: ConfigVariable) -> Result<Self, Self::Error> {
    match value {
      ConfigVariable::Bool(value) => Ok(value),
      _ => bail!("Type mismatch"),
    }
  }
}

impl From<&str> for ConfigVariable {
  fn from(value: &str) -> Self {
    ConfigVariable::String(value.into())
  }
}

impl From<bool> for ConfigVariable {
  fn from(value: bool) -> Self {
    ConfigVariable::Bool(value)
  }
}

impl From<Vec<String>> for ConfigVariable {
  fn from(value: Vec<String>) -> Self {
    ConfigVariable::VecOfStrings(value)
  }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  Error {
    description: Vec<String>,
  },
  Config {
    config: HashMap<ServiceVariable, ConfigVariable>,
  },
  TODO,
}
