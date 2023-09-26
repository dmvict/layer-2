use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TokenKind {
  Auth,
  Upload,
  Download,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Token {
  pub user_id: String,
  pub kind: TokenKind,
  pub token: String,
  pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicFileMeta {
  pub name: String,
  pub ext: String,
  pub url: String,
}
