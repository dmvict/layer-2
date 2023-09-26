use std::sync::Arc;

use anyhow::Result;
use service_common::service_config::ServiceConfig;

use super::generate_pdf;

pub async fn generate_file(
  payment_intent_id: String,
  client: Arc<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<Vec<u8>> {
  let (_, file) = generate_pdf(payment_intent_id, &client, config).await?;
  Ok(file)
}
