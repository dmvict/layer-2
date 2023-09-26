use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};

use crate::{
  db::models::Room,
  jcr::models::{JCRInChatDb, JCROut},
};

pub async fn fetch_room_without_staff(
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<Vec<Room>> {
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &JCRInChatDb::FetchRoomWithoutStaff,
  )
  .await
  .context("Fail to send fetch rooms request  ")?
  .result()?
  {
    JCROut::Rooms { rooms } => Ok(rooms),
    _ => bail!("Invalid response from chat-db"),
  }
}
