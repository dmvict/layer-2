use std::sync::Arc;
use tera::Tera;
use tokio::sync::Mutex;

use anyhow::{bail, Context, Result};

use service_common::{
  config::models::ServiceVariable, hashmap, make_request, service_config::ServiceConfig,
};

use service_common::models::EmailTemplates;

use crate::{
  db::{
    mail_sender::{MailProvider, MailSender},
    models::Receipt,
  },
  jcr::models::{JCRInTemplates, JCROut},
};

pub async fn send_receipt(
  receipt: Receipt,
  file: Option<Vec<u8>>,
  mail_provider: Arc<MailProvider>,
  tera: Arc<Mutex<Tera>>,
  client: Arc<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let email_address = receipt.email.clone();
  let email_body = fetch_email_body(
    tera,
    client,
    receipt,
    &config.get_variable(&ServiceVariable::TemplatesDbUrl)?,
  )
  .await?;
  mail_provider
    .send_receipt(file, config, email_body, email_address)
    .await?;
  Ok(())
}

async fn fetch_email_body(
  tera: Arc<Mutex<Tera>>,
  client: Arc<reqwest::Client>,
  receipt: Receipt,
  template_db_url: &String,
) -> Result<String> {
  let context = hashmap! {
    "amount" => receipt.amount.to_string(),
    "currency" => receipt.currency,
    "description" => receipt.description,
    "name" => receipt.name,
  };
  let mut tera = tera.lock().await;

  if !tera
    .get_template_names()
    .any(|n| n.eq(EmailTemplates::Receipt.as_ref()))
  {
    let jcr = JCRInTemplates::GetTemplate {
      template: EmailTemplates::Receipt.to_string(),
    };
    let html = match make_request::<_, JCROut>(&client, &template_db_url, &jcr)
      .await?
      .result()?
    {
      JCROut::Template { jinja } => jinja,
      _ => bail!("Invalid response from templates db"),
    };
    tera
      .add_raw_template(&EmailTemplates::Receipt.as_ref(), &html)
      .context("Tera: Failed adding a template")?;
  }

  let context = tera::Context::from_serialize(context).context("Tera: Failed building context")?;
  Ok(
    tera
      .render(&EmailTemplates::Receipt.as_ref(), &context)
      .context("Tera: Failed rendering the template")?,
  )
}
