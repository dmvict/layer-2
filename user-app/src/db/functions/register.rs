use std::sync::Arc;

use crate::jcr::models::{JCRInForAuth, JCRInForUser, JCROut};
use anyhow::{bail, Context, Result};
use reqwest::Client;
use service_common::{make_request, service_config::ServiceConfig};

use crate::{
  db::models::AuthEntry,
  http::models::{Auth, NewUser},
};
use service_common::config::models::ServiceVariable;

pub async fn register(
  client: Arc<Client>,
  user: NewUser,
  auth: Auth,
  config: Arc<ServiceConfig>,
) -> Result<AuthEntry> {
  let user_jrc = JCRInForUser::Register { user };
  let user_jrc_response = make_request::<_, JCROut>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::UserDbUrl)?,
    &user_jrc,
  )
  .await
  .context("Failed to send register request")?
  .result()?;

  if let JCROut::Success { message: _ } = user_jrc_response {
    let auth_jrc = JCRInForAuth::Register { auth };
    let auth_jrc_response = make_request::<_, JCROut>(
      &client,
      &config.get_variable::<String>(&ServiceVariable::AuthDbUrl)?,
      &auth_jrc,
    )
    .await
    .context("Failed to send auth register request")?
    .result()?;

    if let JCROut::StoredEntry { entry } = auth_jrc_response {
      return Ok(entry);
    } else {
      bail!("Invalid response from DB.")
    }
  }

  bail!("Registration failed")
}
