use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};

use crate::jcr::{JCRIn, JCROut};

pub async fn put_intent(
  currency: String,
  amount: i64,
  email: String,
  description: String,
  name: String,
  account_id: String,
  client: Arc<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let intent_jcr = JCRIn::PutIntent {
    currency,
    amount,
    email,
    description,
    name,
    account_id,
  };
  match make_request::<_, JCROut>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::IntentDbUrl)?,
    &intent_jcr,
  )
  .await
  .context("Fail to put intent")?
  .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response."),
  }
}
