use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use service_common::{jcr_full, config::models::{ServiceVariable, ConfigVariable}};

pub type MapContext = HashMap<String, Value>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  Generate {
    template: String,
    #[serde(default)]
    context: MapContext,
    #[serde(default)]
    cached: bool,
  },
  UpdateConfig {
    new_config: HashMap<ServiceVariable, ConfigVariable>,
  },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInTemplates {
  GetTemplate { template: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  Template {
    jinja: String
  },
  TODO,
  Success { message: String },
  Error { description: Vec<String> },
}

jcr_full!(JCROut {});
