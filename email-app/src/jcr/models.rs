use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use service_common::{config::models::{ServiceVariable, ConfigVariable}, jcr_full};

use crate::db::models::Receipt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  SendReceipt {
    receipt: Receipt,
  },
  UpdateConfig {
    new_config: HashMap<ServiceVariable, ConfigVariable>,
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
  Config {
    config: HashMap<ServiceVariable, String>,
  },
  Template {
    jinja: String,
  },
  TODO,
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
pub enum JCRInTemplates {
  GetTemplate { template: String },
}

jcr_full!(JCROut {});
