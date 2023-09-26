use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};

use crate::{
  db::models::Topic,
  jcr::models::{JCRInChatDb, JCROut},
};

pub async fn put_topic(
  topic: Topic,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let jcr = JCRInChatDb::PutTopic { topic };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send put topic request to chat-db")?
  .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response from chat-db"),
  }
}
