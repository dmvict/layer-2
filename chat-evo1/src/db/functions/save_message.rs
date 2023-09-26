use std::sync::Arc;

use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};

use anyhow::{bail, Context, Result};

use crate::{
  db::models::Message,
  jcr::models::{JCRInChatDb, JCROut},
};

pub async fn save_message(
  message: Message,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let jcr = JCRInChatDb::SaveMessage { message };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send save message request")?
  .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response from chat-db"),
  }
}
