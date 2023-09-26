use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use lettre::{
  message::{header, MultiPart, SinglePart},
  AsyncSmtpTransport, AsyncTransport, Message as SmtpMessage, Tokio1Executor,
};
use service_common::{config::models::ServiceVariable, service_config::ServiceConfig};
use sparkpost::transmission::{
  Attachment, EmailAddress, Message, Options, Recipient, Transmission,
};

use service_common::models::EmailTemplates;

#[async_trait]
pub trait MailSender {
  async fn send_receipt(
    &self,
    file: Option<Vec<u8>>,
    config: Arc<ServiceConfig>,
    email_body: String,
    email_address: String,
  ) -> Result<()>;
}

pub enum MailProvider {
  Sparkpost(Transmission),
  Smtp(AsyncSmtpTransport<Tokio1Executor>),
}

#[async_trait]
impl MailSender for MailProvider {
  async fn send_receipt(
    &self,
    file: Option<Vec<u8>>,
    config: Arc<ServiceConfig>,
    email_body: String,
    email_address: String,
  ) -> Result<()> {
    match self {
      Self::Sparkpost(transmission) => {
        send_by_sparkpost(transmission, file, config, email_body, email_address).await
      }
      Self::Smtp(mailer) => send_by_smpt(mailer, file, config, email_body, email_address).await,
    }
  }
}

async fn send_by_sparkpost(
  transmission: &Transmission,
  file: Option<Vec<u8>>,
  config: Arc<ServiceConfig>,
  email_body: String,
  email_address: String,
) -> Result<()> {
  let mut email = Message::new(EmailAddress::new(
    config.get_variable::<String>(&ServiceVariable::CompanyEmail)?, // company email
    config.get_variable::<String>(&ServiceVariable::CompanyName)?,  // company name
  ));
  let options = Options::default();
  let recipient = Recipient::from(email_address); // email

  email
    .add_recipient(recipient)
    .subject("Receipt")
    .options(options)
    .html(email_body);

  if let Some(file) = file {
    email.add_attachment(Attachment::from_data(
      format!("{}.pdf", EmailTemplates::Receipt).as_str(),
      "application/pdf",
      std::str::from_utf8(&file)?,
    ));
  }

  transmission.send(&email).context("Fail to send email.")?;
  Ok(())
}

async fn send_by_smpt(
  mailer: &AsyncSmtpTransport<Tokio1Executor>,
  file: Option<Vec<u8>>,
  config: Arc<ServiceConfig>,
  email_body: String,
  email_address: String,
) -> Result<()> {
  let mut attachment = MultiPart::builder().singlepart(
    SinglePart::builder()
      .header(header::ContentType::TEXT_HTML)
      .body(email_body),
  );
  if let Some(file) = file {
    attachment = attachment.singlepart(
      SinglePart::builder()
        .header(header::ContentType::parse(&format!(
          "application/pdf; name={}.pdf",
          EmailTemplates::Receipt
        ))?)
        .body(file),
    );
  }
  let email = SmtpMessage::builder()
    .from(
      format!(
        "{} <{}>",
        config.get_variable::<String>(&ServiceVariable::CompanyName)?,
        config.get_variable::<String>(&ServiceVariable::CompanyEmail)?
      )
      .parse()?,
    )
    .to(format!("<{email_address}>").parse()?) // email
    .subject("Test") // subject
    .multipart(attachment)?;
  mailer.send(email).await.context("Fail to send email")?;
  Ok(())
}
