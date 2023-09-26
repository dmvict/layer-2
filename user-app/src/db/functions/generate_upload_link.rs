use std::sync::Arc;

use anyhow::{bail, Context, Result};
use reqwest::Client;
use service_common::service_config::ServiceConfig;
use service_common::{make_request, models::EmailWebsite};

use crate::{
  http::models::PublicUser,
  jcr::models::{JCRInForUpload, JCRInForUser, JCROut},
};
use service_common::config::models::ServiceVariable;

pub async fn generate_upload_link(
  client: Arc<Client>,
  user: PublicUser,
  config: Arc<ServiceConfig>,
) -> Result<String> {
  let user_jcr = JCRInForUser::FetchUser {
    user_id: None,
    user: Some(EmailWebsite {
      email: user.email,
      website: user.website,
    }),
  };
  let user_response = make_request::<_, JCROut>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::UserDbUrl)?,
    &user_jcr,
  )
  .await
  .context("Fail to fetch user")?
  .result()?;
  let user_id = match user_response {
    JCROut::ResponseUser { user } => user.user_id,
    _ => bail!("Incorrect response from user DB"),
  };

  let upload_jcr = JCRInForUpload::GenerateToken { user_id };

  let upload_response = make_request::<_, JCROut>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::UploadAppUrl)?,
    &upload_jcr,
  )
  .await
  .context("Failed to send token generation request")?
  .result()?;
  match upload_response {
    JCROut::UploadLink { link } => Ok(link),
    _ => bail!("Incorrect response from upload-app"),
  }
}
