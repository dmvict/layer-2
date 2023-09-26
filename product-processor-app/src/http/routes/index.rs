use actix_multipart::Multipart;
use actix_web::Result;
use actix_web::{
  get, post,
  web::{self, Bytes},
  HttpResponse,
};

use service_common::JCR;

use service_common::service_config::ServiceConfig;
use tokio_stream::StreamExt;

use crate::{
  db::functions::{download, get_progress, save_file},
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
  let _client = client.into_inner();
  let config = config.into_inner();
  let jcr = jcr.into_inner();
  match jcr {
    JCRIn::GetProgress { order_id } => {
      let progress = get_progress(order_id).await?;
      JCROut::OrderProgress { progress }.ok()
    }
    JCRIn::UpdateConfig { new_config } => {
      config.update_config(new_config)?;
      JCROut::success("Config updated.")
    }
  }
}

#[post("/upload")]
async fn upload_file(
  payload: Multipart,
  client: web::Data<reqwest::Client>,
  config: web::Data<ServiceConfig>,
) -> JCResult {
  let config = config.into_inner();
  save_file(payload, client, config).await?;
  JCROut::success("File saved.")
}

#[get("/download/{file_id}")]
async fn download_file(
  client: web::Data<reqwest::Client>,
  file_id: String,
  config: web::Data<ServiceConfig>,
) -> Result<HttpResponse> {
  let config = config.into_inner();
  let stream = download(file_id, client, config)
    .await
    .map_err(|err| actix_web::error::ErrorNotFound(err))?;
  Ok(HttpResponse::Ok().streaming(stream.map(|result| result.map(|bytes| Bytes::from(bytes)))))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_index)
    .service(post_index)
    .service(download_file);
}
