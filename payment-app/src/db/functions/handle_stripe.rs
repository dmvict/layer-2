use std::{borrow::Borrow, sync::Arc};

use actix_web::{web, HttpRequest};
use service_common::service_config::ServiceConfig;

use anyhow::{bail, Context, Ok, Result};

use crate::{
  db::models::{Event, Receipt, WebhookEventVerifier},
  jcr::{JCRInForEmail, JCROut},
};
use service_common::config::models::ServiceVariable;

use super::generate_pdf;

pub async fn handle_stripe(
  req: HttpRequest,
  payload: web::Bytes,
  client: Arc<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let payload_str = std::str::from_utf8(payload.borrow()).unwrap();
  let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();
  if config.get_variable::<bool>(&ServiceVariable::ShouldSendEmail)? {
    let intent_id = extract_intent_id(payload_str)?;
    let email_app_url = config.get_variable(&ServiceVariable::EmailAppUrl)?;
    let (intent_entity, file) = generate_pdf(intent_id, &client, config).await?;
    let secret = intent_entity.secret.clone();
    WebhookEventVerifier::verify_request(payload_str, stripe_signature, secret.as_str())?;
    let mail_jcr = JCRInForEmail::GenerateReceipt {
      receipt: Receipt::from(intent_entity),
    };
    send_mail(&mail_jcr, &client, email_app_url, file)
      .await
      .context("fail to send mail")?;
  }
  Ok(())
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
  req.headers().get(key)?.to_str().ok()
}

async fn send_mail(
  data: &JCRInForEmail,
  client: &reqwest::Client,
  mail_app_url: String,
  file: Vec<u8>,
) -> Result<()> {
  match client
    .post(mail_app_url)
    .json(data)
    .body(file)
    .send()
    .await
    .context("Failed to send request.")?
    .json::<JCROut>()
    .await
    .context("Failed to parse response.")?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response from email app"),
  }
}

fn extract_intent_id(raw_json: &str) -> Result<String> {
  Ok(serde_json::from_str::<Event>(raw_json)?.data.object.id)
}
