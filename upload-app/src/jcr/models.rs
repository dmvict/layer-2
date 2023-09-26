use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use service_common::{config::models::{ServiceVariable, ConfigVariable}, jcr_error, jcr_full};

use crate::db::models::{PublicFileMeta, Token, TokenKind};

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  GenerateToken {
    user_id: String,
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

#[derive(Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROutForAuth {
  Token {
    #[serde(flatten)]
    token: Token,
  },
  TODO,
  Success {
    message: String,
  },
  Error {
    description: Vec<String>,
  },
}

jcr_error!(JCROutForAuth {});

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  Success { message: String },
  Error { description: Vec<String> },
  UploadLink { link: String },
  TODO,
  Config { config: HashMap<String, String> },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForControl {
  GetConfig { service_name: String },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForFileMeta {
  NewUpload {
    #[serde(flatten)]
    upload: PublicFileMeta,
  },
}

jcr_full!(JCROut {});
