use std::sync::Arc;

use crate::{
  db::models::PoolInvitation,
  jcr::models::{JCRInChatDb, JCROut},
};

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};
use uuid::Uuid;

pub async fn get_pool_invitations(
  self_id: Uuid,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<Vec<PoolInvitation>> {
  let jcr = JCRInChatDb::GetPoolInvitations { self_id };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send fetch pool invitation request")?
  .result()?
  {
    JCROut::PoolInvitations { invitations } => Ok(invitations),
    _ => bail!("Invalid response from chat-db"),
  }
}
