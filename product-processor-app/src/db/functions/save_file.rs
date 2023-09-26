use core::result::Result::Ok as Ok_r;
use std::{path::PathBuf, sync::Arc};

use actix_multipart::Multipart;
use actix_web::web;
use anyhow::{bail, Context, Ok, Result};
use futures_util::StreamExt;
use service_common::config::models::ServiceVariable;
use service_common::{make_request, service_config::ServiceConfig};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
  db::models::PublicFileMeta,
  jcr::models::{JCRInForFileMeta, JCROut},
};

pub async fn save_file(
  mut payload: Multipart,
  client: web::Data<reqwest::Client>,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  let base_path = PathBuf::from("uploads").join(&Uuid::new_v4().to_string()); // Formation of the path to save files
  tokio::fs::create_dir_all(&base_path).await?; // Creating all the necessary directory
  let base_dir_path = base_path.to_str().unwrap();
  // Cycle for processing each field (file) in the request
  while let Some(field) = payload.next().await {
    let mut field = field.map_err(|e| anyhow::anyhow!(format!("Upload failed: {}", e)))?;
    let content_disposition = field.content_disposition();

    // Eliminating the name of the Content-Disposition title
    let filename = content_disposition
      .get_filename()
      .ok_or(anyhow::anyhow!("No file name found"))?;

    // Obtaining a file extension
    let ext = filename.rsplit_once('.').unwrap().1.to_string();

    // Generation of a unique file name
    let file_name = Uuid::new_v4();

    // Creating a new file for recording
    let mut file = tokio::fs::File::create(base_path.join(format!("{file_name}.{ext}"))).await?;

    // Cycle for reading and recording the contents of the file
    while let Some(Ok_r(chunk)) = field.next().await {
      file.write_all(&chunk).await?;
    }
    let jcr = JCRInForFileMeta::NewUpload {
      upload: PublicFileMeta {
        name: file_name.to_string(),
        ext,
        url: format!(
          "{}/download/{}/{}",
          config.get_variable::<String>(&ServiceVariable::SelfUrl)?,
          base_dir_path,
          file_name
        ), //?
      },
    };
    //Make request to filesmeta-db
    match make_request(
      &client,
      &config.get_variable::<String>(&ServiceVariable::FilesmetaDbUrl)?,
      &jcr,
    )
    .await
    .context("Fail to add upload.")?
    {
      JCROut::Success { message: _ } => Ok(()),
      _ => bail!("Incorrect response."),
    }?
  }
  Ok(())
}
