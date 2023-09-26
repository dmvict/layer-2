use std::net::TcpListener;

use anyhow::Result;
use service_common::config::models::ServiceName;
use service_common::{config::utils::get_configuration, http_start};
use user_app::http::routes;

#[tokio::main]
async fn main() -> Result<()> {
  let client = reqwest::Client::new();
  let config = get_configuration(&client, ServiceName::UserApp).await;
  let http = TcpListener::bind("0.0.0.0:8080")?;
  let server = http_start(http, client, config, routes::configure).await?;
  server.await?;
  Ok(())
}
