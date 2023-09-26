use std::net::TcpListener;

use anyhow::Result;
use download_app::http::routes;
use service_common::config::models::ServiceName;
use service_common::config::utils::get_configuration;
use service_common::http_start;

#[tokio::main]
async fn main() -> Result<()> {
  let client = reqwest::Client::new();
  let config = get_configuration(&client, ServiceName::DownloadApp).await;
  let http = TcpListener::bind("0.0.0.0:8080")?;
  let server = http_start(http, client, config, routes::configure).await?;
  server.await?;
  Ok(())
}
