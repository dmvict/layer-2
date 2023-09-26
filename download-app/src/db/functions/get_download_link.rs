use anyhow::{bail, Context, Result};
use service_common::service_config::ServiceConfig;
use service_common::{jwt_encode, make_request};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::db::models::{PublicFileMeta, Token};
use crate::jcr::models::{JCRInForFileMeta, JCRInForPayment, JCROut};
use crate::{db::models::FileToken, jcr::models::JCRInForAuth};

use service_common::config::models::ServiceVariable;

pub async fn get_download_link(
  client: Arc<reqwest::Client>,
  user_id: String,
  intent_id: String,
  config: Arc<ServiceConfig>,
) -> Result<String> {
  let token = create_token(
    &client,
    &config.get_variable(&ServiceVariable::AuthDbUrl)?,
    user_id,
  )
  .await?;
  let file_data = download_file(
    &client,
    &config.get_variable(&ServiceVariable::PaymentAppUrl)?,
    intent_id,
  )
  .await?;
  let file_name = save_file(file_data).await?;
  update_file_meta(
    &client,
    &config.get_variable(&ServiceVariable::FilesmetaDbUrl)?,
    file_name.clone(),
  )
  .await?;
  let file_token = FileToken {
    token,
    file_id: file_name,
  };
  Ok(format!(
    "{}/download/{}",
    config.get_variable::<String>(&ServiceVariable::SelfUrl)?,
    jwt_encode(file_token)?
  ))
}

async fn create_token(
  client: &Arc<reqwest::Client>,
  auth_db_url: &String,
  user_id: String,
) -> Result<Token> {
  let auth_jcr = JCRInForAuth::GenerateToken {
    user_id,
    kind: crate::db::models::TokenKind::Download,
  };
  match make_request::<_, JCROut>(client, &auth_db_url, &auth_jcr)
    .await
    .context("Fail to make create token request")?
    .result()?
  {
    JCROut::Token { token } => Ok(token),
    _ => bail!("Fail to create token"),
  }
}

async fn download_file(
  client: &Arc<reqwest::Client>,
  payment_app_url: &String,
  intent_id: String,
) -> Result<Vec<u8>> {
  let payment_jcr = JCRInForPayment::GenerateFile {
    payment_intent_id: intent_id,
  };
  match make_request::<_, JCROut>(&client, payment_app_url, &payment_jcr)
    .await
    .context("Fail to send generate file request")?
    .result()?
  {
    JCROut::File { file } => Ok(file),
    _ => bail!("Fail to get file"),
  }
}

async fn save_file(file_data: Vec<u8>) -> Result<String> {
  let base_path = PathBuf::from("uploads");
  tokio::fs::create_dir_all(&base_path).await?;
  let file_name = &Uuid::new_v4().to_string();
  let mut file = tokio::fs::File::create(base_path.join(format!("{file_name}.pdf"))).await?;
  file.write_all(&file_data).await?;
  Ok(format!("{file_name}.pdf"))
}

async fn update_file_meta(
  client: &Arc<reqwest::Client>,
  files_meta_db_url: &String,
  file_name: String,
) -> Result<()> {
  let file_jcr = JCRInForFileMeta::NewDownload {
    download: PublicFileMeta {
      name: file_name,
      ext: "pdf".to_string(),
      url: "??".to_string(), //?
    },
  };
  match make_request::<_, JCROut>(client, &files_meta_db_url, &file_jcr)
    .await
    .context("Fail to send new upload request")?
    .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Fail to add new upload"),
  }
}
