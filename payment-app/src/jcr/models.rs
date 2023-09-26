use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use service_common::{config::models::{ServiceVariable, ConfigVariable}, jcr_full};

use crate::db::models::{
  AccountMonthlyBalance, IntentEntity, PublicFileMeta, Receipt, SwitchMethodWithAccounts,
};

pub type MapContext = HashMap<String, Value>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  UpdateConfig {
    new_config: HashMap<ServiceVariable, ConfigVariable>,
  },
  GenerateFile {
    payment_intent_id: String,
  },
  PutIntent {
    currency: String,
    amount: i64,
    email: String,
    description: String,
    name: String,
    account_id: String,
  },
  PutCompany {
    name: String,
    secret: String,
  },
  GetRequiredAccount {
    company_name: String,
  },
  PutAccount {
    id: String,
    company_name: String,
    secret: String,
    picked: i32,
  },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForPdf {
  Generate {
    template: String,
    #[serde(default)]
    context: MapContext,
    #[serde(default)]
    cached: bool,
  },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForEmail {
  GenerateReceipt { receipt: Receipt },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForFilesMeta {
  NewDownload {
    #[serde(flatten)]
    download: PublicFileMeta,
  },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  Success {
    message: String,
  },
  Error {
    description: Vec<String>,
  },
  TODO,
  Config {
    config: HashMap<String, String>,
  },
  File {
    file: Vec<u8>,
  },
  PaymentIntent {
    intent: IntentEntity,
  },
  MethodAndAccounts {
    method_and_accounts: SwitchMethodWithAccounts,
  },
  AccountMonthlyBalances {
    balances: Vec<AccountMonthlyBalance>,
  },
  AccountId {
    id: String,
  },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForControl {
  GetConfig { service_name: String },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInIntentDb {
  FetchIntent {
    intent_id: String,
  },
  PutIntent {
    currency: String,
    amount: i64,
    email: String,
    description: String,
    name: String,
    company_id: String,
  },
  PutCompany {
    name: String,
    secret: String,
  },
  PutAccount {
    id: String,
    company_name: String,
    secret: String,
    picked: i32,
    registration_date: String,
  },
  FetchAccountsAndSwitchMethodByName {
    company_name: String,
  },
  GetActualMonthlyReport {
    company_name: String,
  },
}

jcr_full!(JCROut {});
