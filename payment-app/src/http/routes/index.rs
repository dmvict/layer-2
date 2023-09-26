use actix_web::{get, post, web, HttpRequest, HttpResponse};
use service_common::{service_config::ServiceConfig, JCR};

use crate::{
  db::functions::{
    generate_file, get_required_account, handle_stripe, put_account, put_company, put_intent,
  },
  jcr::{JCRIn, JCROut, JCResult},
};

#[get("/")]
async fn get_index() -> JCResult {
  JCROut::success(chrono::Utc::now().format("%+").to_string())
}

#[post("/")]
async fn post_index(
  jcr: web::Json<JCRIn>,
  client: web::Data<reqwest::Client>,
  config: web::Data<ServiceConfig>,
) -> JCResult {
  let jcr = jcr.into_inner();
  let client = client.into_inner();
  let config = config.into_inner();
  match jcr {
    JCRIn::UpdateConfig { new_config } => {
      config.update_config(new_config)?;
      JCROut::success("Config updated.")
    }
    JCRIn::GenerateFile { payment_intent_id } => {
      let file = generate_file(payment_intent_id, client, config).await?;
      JCROut::File { file }.ok()
    }
    JCRIn::PutCompany { name, secret } => {
      put_company(name, secret, client, config).await?;
      JCROut::success("Company added")
    }
    JCRIn::PutIntent {
      currency,
      amount,
      email,
      description,
      name,
      account_id,
    } => {
      put_intent(
        currency,
        amount,
        email,
        description,
        name,
        account_id,
        client,
        config,
      )
      .await?;
      JCROut::success("Intent added")
    }
    JCRIn::GetRequiredAccount { company_name } => {
      let id = get_required_account(company_name, client, config).await?;
      JCROut::AccountId { id }.ok()
    }
    JCRIn::PutAccount {
      id,
      company_name,
      secret,
      picked,
    } => {
      put_account(id, company_name, secret, picked, client, config).await?;
      JCROut::success("Account updated")
    }
  }
}

#[post("stripe_webhooks")]
pub async fn webhook_handler(
  req: HttpRequest,
  payload: web::Bytes,
  client: web::Data<reqwest::Client>,
  config: web::Data<ServiceConfig>,
) -> HttpResponse {
  let client = client.into_inner();
  let config = config.into_inner();
  let _ = handle_stripe(req, payload, client, config).await;
  HttpResponse::Ok().finish()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(webhook_handler).service(post_index);
}
