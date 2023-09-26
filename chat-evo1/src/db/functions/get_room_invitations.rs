use std::sync::Arc;

use crate::{
  db::models::RoomInvitation,
  jcr::models::{JCRInChatDb, JCROut},
};

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};
use uuid::Uuid;

pub async fn get_room_invitations(
  self_id: Uuid,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<Vec<RoomInvitation>> {
  let jcr = JCRInChatDb::GetRoomInvitations { self_id };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send fetch room invitation request")?
  .result()?
  {
    JCROut::RoomInvitations { invitations } => Ok(invitations),
    _ => bail!("Invalid response from chat-db"),
  }
}
