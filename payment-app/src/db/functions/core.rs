use std::sync::Arc;

use crate::{
  db::models::{IntentEntity, PublicFileMeta},
  jcr::{JCRInForFilesMeta, JCRInForPdf, JCRInIntentDb, JCROut},
};
use anyhow::{bail, Context, Ok, Result};
use serde_json::json;
use service_common::models::EmailTemplates;
use service_common::{hashmap, make_request, service_config::ServiceConfig};

use service_common::config::models::ServiceVariable;

async fn fetch_receipt_data(
  id: String,
  client: &reqwest::Client,
  intent_db_url: &String,
) -> Result<IntentEntity> {
  let data = JCRInIntentDb::FetchIntent { intent_id: id };
  match make_request(client, intent_db_url, &data)
    .await
    .context("Fail to send fetch intent request")?
  {
    JCROut::PaymentIntent { intent } => Ok(intent),
    _ => bail!("Invalid response from intent-db"),
  }
}

async fn generate_pdf_receipt(
  receipt_data: &JCRInForPdf,
  client: &reqwest::Client,
  pdf_app_url: String,
) -> Result<Vec<u8>> {
  Ok(
    client
      .post(pdf_app_url)
      .json(receipt_data)
      .send()
      .await
      .context("Fail to send request")?
      .bytes()
      .await
      .context("Fail to get bytes")?
      .into_iter()
      .collect(),
  )
}

async fn update_files_meta(
  data: &JCRInForFilesMeta,
  client: &reqwest::Client,
  files_meta_db_url: String,
) -> Result<()> {
  match make_request::<_, JCROut>(&client, &files_meta_db_url, &data)
    .await
    .context("Failed to send new download request")?
    .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Incorrect response from files-meta-db"),
  }
}

pub async fn generate_pdf(
  intent_id: String,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<(IntentEntity, Vec<u8>)> {
  // fetch user name and email
  let receipt = fetch_receipt_data(
    intent_id,
    &client,
    &config.get_variable(&ServiceVariable::IntentDbUrl)?,
  )
  .await?;
  let context = hashmap!{
    "amount".to_string() => json!(receipt.amount),
    "description".to_string() => json!(receipt.description),
    "name".to_string() => json!(receipt.name),
    "currency".to_string() =>  json!(receipt.currency),
  };
  let file_jcr = JCRInForPdf::Generate {
    template: EmailTemplates::Receipt.to_string(),
    context,
    cached: true,
  };
  // send request to to pdf generator app
  let file = generate_pdf_receipt(
    &file_jcr,
    &client,
    config.get_variable(&ServiceVariable::PdfGeneratorAppUrl)?,
  )
  .await
  .context("Fail to generate file")?;
  let meta_jcr = JCRInForFilesMeta::NewDownload {
    download: PublicFileMeta {
      name: format!("{}_receipt.pdf", receipt.email), //??
      ext: "pdf".into(),
      url: "???".into(), //??
    },
  };
  // update metadata in files meta db
  update_files_meta(
    &meta_jcr,
    &client,
    config.get_variable(&ServiceVariable::FilesmetaDbUrl)?,
  )
  .await
  .context("fail to update files meta")?;
  Ok((receipt, file))
}
