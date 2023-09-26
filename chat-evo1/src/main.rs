use std::net::TcpListener;

use anyhow::Result;
use chat_evo1::http::routes;
use service_common::{
  config::{models::ServiceName, utils::get_configuration},
  http_start,
};

#[tokio::main]
async fn main() -> Result<()> {
  let client = reqwest::Client::new();
  let config = get_configuration(&client, ServiceName::ChatEvo1).await;
  let http = TcpListener::bind("0.0.0.0:8080")?;
  let server = http_start(http, client, config, routes::configure).await?;
  server.await?;
  Ok(())
}
