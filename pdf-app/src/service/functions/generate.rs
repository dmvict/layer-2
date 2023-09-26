use std::{io::Read, sync::Arc};

use actix_web::web::Bytes;
use anyhow::{bail, Context, Result};
use reqwest::Client;
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};
use tera::Tera;
use tokio::sync::Mutex;
use wkhtmltopdf::PdfApplication;

use crate::jcr::models::{JCRInTemplates, JCROut, MapContext};

pub async fn generate(
  client: Arc<Client>,
  config: Arc<ServiceConfig>,
  tera: Arc<Mutex<Tera>>,
  template: String,
  context: MapContext,
  cached: bool,
) -> Result<Bytes> {
  let mut tera = tera.lock().await;

  if !cached || !tera.get_template_names().any(|n| n.eq(&template)) {
    let jcr = JCRInTemplates::GetTemplate {
      template: template.clone(),
    };
    let db_url = config
      .get_variable::<String>(&ServiceVariable::TemplatesDbUrl)
      .context("Config: Template db url not found")?;
    let html = match make_request::<_, JCROut>(&client, &db_url, &jcr) // TODO: add db url from config
      .await?
      .result()?
    {
      JCROut::Template { jinja } => jinja,
      _ => bail!("Invalid response from templates db"),
    };
    tera
      .add_raw_template(&template, &html)
      .context("Tera: Failed adding a template")?;
  }

  let context = tera::Context::from_serialize(context).context("Tera: Failed building context")?;
  let html = tera
    .render(&template, &context)
    .context("Tera: Failed rendering the template")?;

  let app = PdfApplication::new().context("PDF: Failed application initialization")?;

  let mut file = app
    .builder()
    .build_from_html(html)
    .context("PDF: Failed building the file")?;

  drop(app);

  let mut buffer = vec![];

  file
    .read_to_end(&mut buffer)
    .context("Failed reading built PDF")?;

  Ok(buffer.into())
}
