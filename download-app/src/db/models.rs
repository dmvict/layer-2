use std::ops::Mul;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TokenKind {
  Auth,
  Upload,
  Download,
}

impl TokenKind {
  pub fn time(&self) -> i64 {
    match self {
      TokenKind::Auth => 60i64,
      TokenKind::Upload => 10,
      TokenKind::Download => 10,
    }
    .mul(60)
  }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct FileToken {
  pub token: Token,
  pub file_id: String,
}
