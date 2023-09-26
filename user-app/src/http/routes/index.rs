use actix_web::{get, post, web};
use service_common::service_config::ServiceConfig;
use service_common::{jwt_encode, JCR};

use crate::{
  db::functions::{generate_upload_link, login, register, update_user},
  jcr::models::{JCRIn, JCROut, JCResult},
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
  let client = client.into_inner();
  let config = config.into_inner();
  let jcr = jcr.into_inner();
  match jcr {
    JCRIn::Register { user, auth } => {
      let entry = register(client, user, auth, config).await?;
      JCROut::Token {
        token: jwt_encode(entry)?,
      }
      .ok()
    }
    JCRIn::Login { auth } => {
      let entry = login(client, auth, config).await?;
      JCROut::Token {
        token: jwt_encode(entry)?,
      }
      .ok()
    }
    JCRIn::UpdateUser { user, new_user } => {
      update_user(client, user, new_user, config).await?;
      JCROut::success("User updated.")
    }
    JCRIn::GenerateUploadLink { user } => {
      let link = generate_upload_link(client, user, config).await?;
      JCROut::UploadLink { link }.ok()
    }
    JCRIn::UpdateConfig { new_config } => {
      config.update_config(new_config)?;
      JCROut::success("Config updated.")
    }
  }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(get_index).service(post_index);
}
