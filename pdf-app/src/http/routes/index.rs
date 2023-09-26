use actix_web::{
  get, post,
  web::{self, Bytes},
};
use reqwest::Client;
use service_common::{service_config::ServiceConfig, JCR};
use tera::Tera;
use tokio::sync::Mutex;

use crate::{
  jcr::models::{JCRIn, JCROut, JCResult},
  service::functions::generate,
};

#[get("/")]
async fn get_index() -> JCResult {
  JCROut::success(chrono::Utc::now().format("%+").to_string())
}

#[post("/")]
async fn post_index(
  jcr: web::Json<JCRIn>,
  client: web::Data<Client>,
  config: web::Data<ServiceConfig>,
  tera: web::Data<Mutex<Tera>>,
) -> Result<Bytes, JCROut> {
  let client = client.into_inner();
  let config = config.into_inner();
  let tera = tera.into_inner();
  let jcr = jcr.into_inner();
  match jcr {
    JCRIn::Generate {
      template,
      context,
      cached,
    } => {
      let bytes = generate(client, config, tera, template, context, cached).await?;
      Ok(bytes)
    }
    JCRIn::UpdateConfig { new_config } =>{
      config.update_config(new_config)?;
      Err(JCROut::success("Config updated.")?)//not confident but...
    }
  }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(get_index).service(post_index);
}
