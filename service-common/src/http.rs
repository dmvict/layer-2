use std::net::TcpListener;

use actix_web::{
  dev::Server,
  web::{Data, ServiceConfig},
  App, HttpServer,
};
use anyhow::Result;

pub async fn start<F>(
  http: TcpListener,
  client: reqwest::Client,
  config: crate::config::service_config::ServiceConfig,
  configure: F,
) -> Result<Server>
where
  F: FnOnce(&mut ServiceConfig) + Send + Sync + Clone + Copy + 'static,
{
  let client = Data::new(client);
  let config = Data::new(config);
  let server = HttpServer::new(move || {
    App::new()
      .app_data(client.clone())
      .app_data(config.clone())
      .configure(configure)
  })
  .listen(http)?
  .run();
  Ok(server)
}
