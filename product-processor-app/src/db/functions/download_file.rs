use std::{path::Path, sync::Arc};

use actix_web::web::{self};
use anyhow::{Context, Result};
use service_common::{make_request, service_config::ServiceConfig};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
  db::models::PublicFileMeta,
  jcr::models::{JCRInForFileMeta, JCROut},
};
use service_common::config::models::ServiceVariable;

pub async fn download(
  file_id: String,
  client: web::Data<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<FramedRead<File, BytesCodec>> {
  let file = File::open(Path::new(&file_id)).await?;
  let jcr = JCRInForFileMeta::NewDownload {
    download: PublicFileMeta {
      name: file_id,
      ext: "ext".to_string(),
      url: "url".to_string(),
    },
  };
  match make_request(
    &client,
    &config.get_variable::<String>(&ServiceVariable::FilesmetaDbUrl)?,
    &jcr,
  )
  .await
  .context("Fail to add upload.")?
  {
    JCROut::Success { message: _ } => Ok(FramedRead::new(file, BytesCodec::new())),
    _ => Err(anyhow::anyhow!("Incorrect response.")),
  }
}
