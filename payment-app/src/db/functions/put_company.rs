use std::sync::Arc;

use anyhow::{bail, Context, Result};
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};

use crate::jcr::{JCRInIntentDb, JCROut};

pub async fn put_company(
  name: String,
  secret: String,
  client: Arc<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let company_jcr = JCRInIntentDb::PutCompany { name, secret };
  match make_request::<_, JCROut>(
    &client,
    &config.get_variable::<String>(&ServiceVariable::IntentDbUrl)?,
    &company_jcr,
  )
  .await
  .context("Fail to put company")?
  .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response"),
  }
}
