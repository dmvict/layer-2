use std::sync::Arc;

use crate::{
  db::models::AuthEntry,
  http::models::Auth,
  jcr::models::{JCRInForAuth, JCROut},
};
use anyhow::{bail, Context, Result};
use reqwest::Client;
use service_common::config::models::ServiceVariable;
use service_common::{make_request, service_config::ServiceConfig};

pub async fn login(
  client: Arc<Client>,
  auth: Auth,
  config: Arc<ServiceConfig>,
) -> Result<AuthEntry> {
  //auth 8081
  //send request to auth db
  let auth_jrc = JCRInForAuth::Login { auth };
  match make_request::<_, JCROut>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::AuthDbUrl)?,
    &auth_jrc,
  )
  .await
  .context("Failed to send login request")?
  .result()?
  {
    JCROut::StoredEntry { entry } => Ok(entry),
    _ => bail!("Invalid response from DB."),
  }
}
