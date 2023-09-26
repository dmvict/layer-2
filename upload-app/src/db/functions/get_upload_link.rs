use std::sync::Arc;

use anyhow::{bail, Context, Result};

use reqwest::Client;
use service_common::{jwt_encode, make_request, service_config::ServiceConfig};

use crate::jcr::models::{JCRInForAuth, JCROutForAuth};
use service_common::config::models::ServiceVariable;

pub async fn get_upload_link(
  client: Arc<Client>,
  user_id: String,
  config: Arc<ServiceConfig>,
) -> Result<String> {
  let auth_jcr = JCRInForAuth::GenerateToken {
    user_id,
    kind: crate::db::models::TokenKind::Upload,
  };
  let jct_out = make_request::<_, JCROutForAuth>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::AuthDbUrl)?,
    &auth_jcr,
  )
  .await
  .context("Failed to send token creation request.")?
  .result()?;
  match jct_out {
    JCROutForAuth::Token { token } => Ok(format!(
      "{}/upload/{}",
      config.get_variable::<String>(&ServiceVariable::SelfUrl)?,
      jwt_encode(token.token)?
    )),
    _ => bail!("Failed to create Token."),
  }
}
