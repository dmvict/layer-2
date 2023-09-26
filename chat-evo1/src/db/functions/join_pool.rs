use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};
use uuid::Uuid;

use crate::jcr::models::{JCRInChatDb, JCROut};
pub async fn join_pool(
  pool_id: Uuid,
  self_id: Uuid,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let jcr = JCRInChatDb::JoinPool { pool_id, self_id };
  match make_request::<_, JCROut>(
    client,
    &config.get_variable::<String>(&ServiceVariable::ChatDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to send join pool request")?
  .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response from chat-db"),
  }
}
