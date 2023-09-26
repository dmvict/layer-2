use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCR {
  ReqUpload { data: String },
}
