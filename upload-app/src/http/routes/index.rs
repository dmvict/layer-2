use actix_multipart::Multipart;
use actix_web::{get, post, web};
use service_common::{service_config::ServiceConfig, JCR};

use crate::{
  db::functions::{get_upload_link, upload},
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
    JCRIn::GenerateToken { user_id } => {
      let link = get_upload_link(client, user_id, config).await?;
      JCROut::UploadLink { link }.ok()
    }
    JCRIn::UpdateConfig { new_config } => {
      config.update_config(new_config)?;
      JCROut::success("Config updated.")
    }
  }
}

#[post("/upload/{token}")]
async fn upload_file(
  payload: Multipart,
  token: web::Path<String>,
  client: web::Data<reqwest::Client>,
  config: web::Data<ServiceConfig>,
) -> JCResult {
  let config = config.into_inner();
  upload(payload, token, client, config).await?;
  JCROut::success("File uploaded")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_index)
    .service(post_index)
    .service(upload_file);
}
