use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};
use uuid::Uuid;

use crate::{
  db::models::Message,
  jcr::models::{JCRInChatDb, JCROut},
};

pub async fn get_messages(
  id: Uuid,
  n: usize,
  size: usize,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<Vec<Message>> {
  let jcr = JCRInChatDb::FetchRoomMessages {
    room_id: id,
    n,
    size,
  };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send fetch messages request to chat-db")?
  .result()?
  {
    JCROut::Messages { messages } => Ok(messages),
    _ => bail!("Invalid response from chat-db"),
  }
}
