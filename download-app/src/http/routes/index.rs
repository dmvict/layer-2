use actix_web::{get, post, web, HttpRequest, HttpResponse};

use service_common::JCR;

use service_common::service_config::ServiceConfig;

use crate::{
  db::functions::{download, get_download_link},
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
  let jcr = jcr.into_inner();
  let config = config.into_inner();
  let client = client.into_inner();
  match jcr {
    JCRIn::GenerateReceiptLink { user_id, intent_id } => {
      let link = get_download_link(client, user_id, intent_id, config).await?;
      JCROut::DownloadLink { link }.ok()
    }
    JCRIn::UpdateConfig { new_config } => {
      config.update_config(new_config)?;
      JCROut::success("Config updated.")
    }
  }
}

#[get("/download/{token}")]
async fn download_file(
  req: HttpRequest,
  client: web::Data<reqwest::Client>,
  token: web::Path<String>,
  config: web::Data<ServiceConfig>,
) -> HttpResponse {
  let client = client.into_inner();
  let config = config.into_inner();
  if let Ok(file_name) = download(token, client, config).await {
    let file_path = std::path::PathBuf::from("./upload").join(&file_name);
    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();
    file.into_response(&req)
  } else {
    HttpResponse::NotFound().finish()
  }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_index)
    .service(post_index)
    .service(download_file);
}
