use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};
use uuid::Uuid;

use crate::{
  db::models::Room,
  jcr::models::{JCRInChatDb, JCROut},
};

pub async fn get_user_rooms(
  id: Uuid,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<Vec<Room>> {
  let jcr = JCRInChatDb::FetchUserRooms { user_id: id };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send fetch messages request to chat-db")?
  .result()?
  {
    JCROut::Rooms { rooms } => Ok(rooms),
    _ => bail!("Invalid response from chat-db"),
  }
}
