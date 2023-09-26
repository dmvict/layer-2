use email_app::db::mail_sender::MailProvider;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use service_common::config::models::ServiceName;
use service_common::config::service_config::ServiceConfig as Conf;
use service_common::config::{models::ServiceVariable, utils::get_configuration};
use sparkpost::transmission::Transmission;
use std::net::TcpListener;
use tera::Tera;
use tokio::sync::Mutex;

use actix_web::{
  dev::Server,
  web::{Data, ServiceConfig},
  App, HttpServer,
};
use anyhow::{bail, Result};
use email_app::http::routes;

#[tokio::main]
async fn main() -> Result<()> {
  let client = reqwest::Client::new();
  let config = get_configuration(&client, ServiceName::EmailApp).await;
  let http = TcpListener::bind("0.0.0.0:8080")?;
  let mail_provider = get_email_provider(&config)?;
  let tera = Tera::default();
  let server = start(http, client, mail_provider, tera, routes::configure).await?;
  server.await?;
  Ok(())
}

async fn start<F>(
  http: TcpListener,
  client: reqwest::Client,
  mail_provider: MailProvider,
  tera: Tera,
  configure: F,
) -> Result<Server>
where
  F: FnOnce(&mut ServiceConfig) + Send + Sync + Clone + Copy + 'static,
{
  let client = Data::new(client);
  let tera = Data::new(Mutex::new(tera));
  let mail_provider = Data::new(mail_provider);
  let server = HttpServer::new(move || {
    App::new()
      .app_data(client.clone())
      .app_data(tera.clone())
      .app_data(mail_provider.clone())
      .configure(configure)
  })
  .listen(http)?
  .run();
  Ok(server)
}

fn get_email_provider(config: &Conf) -> Result<MailProvider> {
  match config
    .get_variable::<String>(&ServiceVariable::EmailProvider)?
    .as_str()
  {
    "sparkpost" => Ok(MailProvider::Sparkpost(Transmission::new(
      config.get_variable::<String>(&ServiceVariable::SparkpostApiKey)?,
    ))),
    "smtp_server" => {
      let creds = Credentials::new(
        config.get_variable(&ServiceVariable::SmtpLogin)?,
        config.get_variable(&ServiceVariable::SmtpPassword)?,
      );
      Ok(MailProvider::Smtp(
        AsyncSmtpTransport::<Tokio1Executor>::relay(
          &config.get_variable::<String>(&ServiceVariable::SmtpUrl)?,
        )?
        .credentials(creds)
        .build(),
      ))
    }
    _ => bail!("Unknown provider name"),
  }
}
