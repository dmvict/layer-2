use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use service_common::{jcr_full, config::models::ConfigVariable};

use crate::db::models::{PublicFileMeta, Token, TokenKind};
use service_common::config::models::ServiceVariable;

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  GenerateReceiptLink {
    user_id: String,
    intent_id: String,
  },
  UpdateConfig {
    new_config: HashMap<ServiceVariable, ConfigVariable>,
  },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForAuth {
  GenerateToken {
    user_id: String,
    kind: TokenKind,
  },
  CheckToken {
    user_id: String,
    kind: TokenKind,
    token: String,
  },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  Success {
    message: String,
  },
  Error {
    description: Vec<String>,
  },
  DownloadLink {
    link: String,
  },
  TODO,
  Config {
    config: HashMap<ServiceVariable, String>,
  },
  File {
    file: Vec<u8>,
  },
  Token {
    #[serde(flatten)]
    token: Token,
  },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForFileMeta {
  NewDownload {
    #[serde(flatten)]
    download: PublicFileMeta,
  },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForControl {
  GetConfig { service_name: String },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForPayment {
  GenerateFile { payment_intent_id: String },
}

jcr_full!(JCROut {});
