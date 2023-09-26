use std::net::TcpListener;

use actix_web::{dev::Server, web::Data, App, HttpServer};
use anyhow::Result;
use pdf_app::http::routes;
use service_common::{
  config::{models::ServiceName, utils::get_configuration},
  service_config::ServiceConfig,
};
use tera::Tera;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
  let http = TcpListener::bind("0.0.0.0:8080")?;
  let client = reqwest::Client::new();
  let config = get_configuration(&client, ServiceName::PdfApp).await;
  let tera = Tera::default();
  let server = http_start(http, client, config, tera, routes::configure).await?;
  server.await?;
  Ok(())
}

pub async fn http_start<F>(
  http: TcpListener,
  client: reqwest::Client,
  config: ServiceConfig,
  tera: Tera,
  configure: F,
) -> Result<Server>
where
  F: FnOnce(&mut actix_web::web::ServiceConfig) + Send + Sync + Clone + Copy + 'static,
{
  let client = Data::new(client);
  let config = Data::new(config);
  let tera = Data::new(Mutex::new(tera));
  let server = HttpServer::new(move || {
    App::new()
      .app_data(client.clone())
      .app_data(config.clone())
      .app_data(tera.clone())
      .configure(configure)
  })
  .listen(http)?
  .run();
  Ok(server)
}
