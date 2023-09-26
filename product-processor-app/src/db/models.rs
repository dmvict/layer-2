use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicFileMeta {
  pub name: String,
  pub ext: String,
  pub url: String,
}
