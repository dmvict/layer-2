use serde_json::json;
use std::collections::HashMap;

use crate::{config::models::ServiceName, hashmap, make_request, service_config::ServiceConfig};

use super::models::{JCROut, ServiceVariable, ConfigVariable};

pub async fn get_configuration(
  client: &reqwest::Client,
  service_name: ServiceName,
) -> ServiceConfig {
  let jcr = json!({
  "subject": "get_config",
  "data": {
  "service_name": service_name,
  },
  });
  let config = match make_request::<_, JCROut>(client, "http://localhost:8085", &jcr).await {
    Ok(JCROut::Config { config }) => config,
    _ => get_default_config(service_name),
  };
  ServiceConfig::new(config)
}

pub fn get_default_config(service_name: ServiceName) -> HashMap<ServiceVariable, ConfigVariable> {
  match service_name {
    ServiceName::DownloadApp => prepare_download_app_vars(),
    ServiceName::EmailApp => prepare_email_app_vars(),
    ServiceName::PaymentApp => prepare_payment_app_vars(),
    ServiceName::ProductProcessorApp => prepare_product_processor_app_vars(),
    ServiceName::UploadApp => prepare_upload_app_vars(),
    ServiceName::UserApp => prepare_user_app_vars(),
    ServiceName::PdfApp => prepare_pdf_app_vars(),
    ServiceName::ChatEvo1 => prepare_chat_evo1_vars(),
  }
}

fn prepare_download_app_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8087".into(),
    ServiceVariable::AuthDbUrl =>               "http://localhost:8081".into(),
    ServiceVariable::FilesmetaDbUrl =>          "http://localhost:8083".into(),
    ServiceVariable::PaymentAppUrl =>           "http://localhost:8089".into(),
  }
}

fn prepare_email_app_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8088".into(),
    ServiceVariable::CompanyName =>             "Test company".into(),
    ServiceVariable::CompanyEmail =>            "test_company@gmail.com".into(),
    ServiceVariable::SparkpostApiKey =>         "SPARKPOST_API_KEY".into(),
    ServiceVariable::SmtpLogin =>               "test_login".into(),
    ServiceVariable::SmtpPassword =>            "test_password".into(),
    ServiceVariable::SmtpUrl =>                 "smtp.test.com".into(),
    ServiceVariable::EmailProvider =>           "sparkpost".into(),
    ServiceVariable::TemplatesDbUrl =>           "http://localhost:8088".into(),
  }
}

fn prepare_payment_app_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8089".into(),
    ServiceVariable::StripeWebhookSecret =>     "WEBHOOK_SERCET".into(),
    ServiceVariable::StripeSecretKey =>         "SERCET_KEY".into(),
    ServiceVariable::EmailAppUrl =>             "http://localhost:8088".into(),
    ServiceVariable::ShouldSendEmail =>         true.into(),
    ServiceVariable::PdfGeneratorAppUrl =>      "http:/localhost:8093".into(),
    ServiceVariable::FilesmetaDbUrl =>          "http://localhost:8083".into(),
    ServiceVariable::IntentDbUrl =>             "http://localhost:9080".into(),
  }
}

fn prepare_product_processor_app_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8090".into(),
    ServiceVariable::FilesmetaDbUrl =>          "http://localhost:8083".into(),
  }
}

fn prepare_upload_app_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8091".into(),
    ServiceVariable::AuthDbUrl =>               "http://localhost:8081".into(),
    ServiceVariable::ProductProcessorAppUrl =>  "http://localhost:8090".into(),
  }
}

fn prepare_user_app_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8092".into(),
    ServiceVariable::UserDbUrl =>               "http://localhost:8085".into(),
    ServiceVariable::UploadAppUrl =>            "http://localhost:8091".into(),
    ServiceVariable::AuthDbUrl =>               "http://localhost:8081".into(),
  }
}

fn prepare_pdf_app_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8093".into(),
    ServiceVariable::TemplatesDbUrl =>          "http://localhost:9080".into(),
  }
}

fn prepare_chat_evo1_vars() -> HashMap<ServiceVariable, ConfigVariable> {
  hashmap! {
    ServiceVariable::SelfUrl =>                 "http://localhost:8094".into(),
    ServiceVariable::ChatDbUrl =>               "http://localhost:9081".into(),
  }
}
