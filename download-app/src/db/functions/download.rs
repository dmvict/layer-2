use actix_web::web;
use anyhow::{bail, Context, Ok, Result};
use service_common::{jwt_decode, make_request};
use std::sync::Arc;

use crate::{
  db::models::FileToken,
  jcr::models::{JCRInForAuth, JCROut},
};
use service_common::config::models::ServiceVariable;
use service_common::service_config::ServiceConfig;

pub async fn download(
  token: web::Path<String>,
  client: Arc<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<String> {
  // Parse token
  let token = jwt_decode::<FileToken>(&token).context("Failed to parse token")?;
  let jcr_auth = JCRInForAuth::CheckToken {
    user_id: token.token.user_id,
    kind: token.token.kind,
    token: token.token.token,
  };

  match make_request::<_, JCROut>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::AuthDbUrl)?,
    &jcr_auth,
  )
  .await
  .context("Failed to send check token request.")?
  .result()?
  {
    JCROut::Success { message: _ } => Ok(token.file_id),
    _ => bail!("Failed to check token."),
  }
}
