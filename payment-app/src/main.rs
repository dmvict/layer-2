use std::net::TcpListener;

use payment_app::http::routes;
use service_common::config::models::ServiceName;
use service_common::{config::utils::get_configuration, http_start};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  let client = reqwest::Client::new();
  let config = get_configuration(&client, ServiceName::PaymentApp).await;
  let http = TcpListener::bind("0.0.0.0:8080")?;
  let server = http_start(http, client, config, routes::configure).await?;
  server.await?;
  Ok(())
}
