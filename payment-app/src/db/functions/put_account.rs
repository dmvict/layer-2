use std::sync::Arc;

use anyhow::{bail, Context, Result};
use chrono::Local;
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};

use crate::jcr::{JCRInIntentDb, JCROut};

pub async fn put_account(
  id: String,
  company_name: String,
  secret: String,
  picked: i32,
  client: Arc<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let company_jcr = JCRInIntentDb::PutAccount {
    id,
    company_name,
    secret,
    picked,
    registration_date: Local::now().format("%Y-%m-%d").to_string(),
  };
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
