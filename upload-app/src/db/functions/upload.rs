use core::result::Result::Ok as Ok_r;
use std::{path::PathBuf, sync::Arc};

use actix_multipart::Multipart;
use actix_web::web;
use anyhow::{anyhow, bail, Context, Ok, Result};
use futures_util::StreamExt;
use service_common::{get_max_files, jwt_decode, make_request, service_config::ServiceConfig};

use crate::{
  db::models::Token,
  jcr::models::{JCRInForAuth, JCROutForAuth},
};
use service_common::config::models::ServiceVariable;

use reqwest::multipart::{Form, Part};

static ALLOWED_EXT: &[&str] = &["png", "jpg", "bmp"];

pub async fn upload(
  mut payload: Multipart,
  token: web::Path<String>,
  client: web::Data<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  // Parse token
  let token = jwt_decode::<Token>(&token).context("Failed to parse token")?;
  let jcr_auth = JCRInForAuth::CheckToken {
    user_id: token.user_id,
    kind: token.kind,
    token: token.token,
  };
  // Make request to auth-db for check token
  make_request::<JCRInForAuth, JCROutForAuth>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::AuthDbUrl)?,
    &jcr_auth,
  )
  .await
  .context("Fail to send check token request.")?
  .result()
  .context("Failed to verify the token")?;

  let mut count = 0; // initialization of the counter -file file meter

  // Cycle for processing each field (file) in the request
  while let Some(field) = payload.next().await {
    if count >= get_max_files() {
      // Checking for exceeding the loading file limit
      bail!("Max file amount: {}", get_max_files());
    }

    let mut field = field.map_err(|e| anyhow::anyhow!(format!("Upload failed: {}", e)))?;

    let content_disposition = field.content_disposition();

    let filename = content_disposition
      .get_filename()
      .ok_or(anyhow::anyhow!("No file name found"))?;

    PathBuf::from(filename)
      .extension()
      .map(|e| {
        let ext = e.to_str()?;
        ALLOWED_EXT.contains(&ext).then_some(ext.to_string())
      })
      .flatten()
      .ok_or(anyhow::anyhow!("Unsupported file extension",))?;

    // Cycle for reading and recording the contents of the file
    while let Some(Ok_r(chunk)) = field.next().await {
      let form = Form::new().part("file", Part::stream(chunk));
      client
        .post(config.get_variable::<String>(&ServiceVariable::ProductProcessorAppUrl)?)
        .multipart(form)
        .send()
        .await
        .map_err(|_| anyhow!("Fail to parse response."))?;
      count += 1;
    }
  }
  Ok(())
}
