use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use service_common::{config::models::{ServiceVariable, ConfigVariable}, jcr_full};

use crate::db::models::PublicFileMeta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  GetProgress {
    order_id: String,
  },
  UpdateConfig {
    new_config: HashMap<ServiceVariable, ConfigVariable>,
  },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  OrderProgress { progress: String },
  TODO,
  Success { message: String },
  Error { description: Vec<String> },
  Config { config: HashMap<String, String> },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForUpload {
  GenerateToken { user_id: String },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForFileMeta {
  NewDownload {
    #[serde(flatten)]
    download: PublicFileMeta,
  },
  NewUpload {
    #[serde(flatten)]
    upload: PublicFileMeta,
  },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForControl {
  GetConfig { service_name: String },
}

jcr_full!(JCROut {});
