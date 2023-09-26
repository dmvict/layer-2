use actix_web::{
  get, post,
  web::{self, Bytes},
};
use service_common::{service_config::ServiceConfig, JCR};
use tera::Tera;
use tokio::sync::Mutex;

use crate::{
  db::{
    functions::send_receipt,
    mail_sender::MailProvider,
  },
  jcr::models::{JCRIn, JCROut, JCResult},
};

#[get("/")]
async fn get_index() -> JCResult {
  JCROut::success(chrono::Utc::now().format("%+").to_string())
}

#[post("/")]
async fn post_index(
  jcr: web::Json<JCRIn>,
  config: web::Data<ServiceConfig>,
  mail_provider: web::Data<MailProvider>,
  client: web::Data<reqwest::Client>,
  tera: web::Data<Mutex<Tera>>,
  file: Option<web::Data<Bytes>>,
) -> JCResult {
  let tera = tera.into_inner();
  let jcr = jcr.into_inner();
  let config = config.into_inner();
  let client = client.into_inner();
  let mail_provider = mail_provider.into_inner();
  match jcr {
    JCRIn::SendReceipt { receipt } => {
        send_receipt(receipt, file.map(|file| file.to_vec()), mail_provider, tera, client, config).await?;// TODO: rework with optional
        JCROut::success("Mail sent")
    }
    JCRIn::UpdateConfig { new_config } => {
      config.update_config(new_config)?;
      JCROut::success("Config update")
    }
  }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(get_index).service(post_index);
}
