use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};
use uuid::Uuid;

use crate::jcr::models::{JCRInChatDb, JCROut};

pub async fn invite_pool_to_room(
  pool_id: Uuid,
  room_id: Uuid,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let jcr = JCRInChatDb::InvitePoolToRoom { pool_id, room_id };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send invite pool to room request")?
  .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response from chat-db"),
  }
}
