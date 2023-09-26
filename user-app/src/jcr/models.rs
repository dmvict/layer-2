use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use service_common::{config::models::{ServiceVariable, ConfigVariable}, jcr_full, models::EmailWebsite};

use crate::{
  db::models::{Address, AuthEntry, User},
  http::models::{Auth, NewUser, PublicAddress, PublicUser},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  Register {
    #[serde(flatten)]
    user: NewUser,
    auth: Auth,
  },
  Login {
    auth: Auth,
  },
  UpdateUser {
    user: EmailWebsite,
    new_user: NewUser,
  },
  GenerateUploadLink {
    user: PublicUser,
  },
  UpdateConfig {
    new_config: HashMap<ServiceVariable, ConfigVariable>,
  },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForUser {
  Register {
    #[serde(flatten)]
    user: NewUser,
  },
  UpdateUser {
    user: EmailWebsite,
    new_user: PublicUser,
  },
  UpdateAddress {
    user: EmailWebsite,
    new_address: PublicAddress,
  },
  FetchAddress {
    user_id: Option<String>,
    user: Option<EmailWebsite>,
  },
  FetchUser {
    user_id: Option<String>,
    user: Option<EmailWebsite>,
  },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForAuth {
  Register {
    #[serde(flatten)]
    auth: Auth,
  },
  Login {
    #[serde(flatten)]
    auth: Auth,
  },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForUpload {
  GenerateToken { user_id: String },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInForControl {
  GetConfig { service_name: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  Token {
    token: String,
  },
  StoredEntry {
    entry: AuthEntry,
  },
  ResponseUser {
    #[serde(flatten)]
    user: User,
  },
  ResponseAddress {
    #[serde(flatten)]
    address: Address,
  },
  UploadLink {
    link: String,
  },
  TODO,
  Success {
    message: String,
  },
  Error {
    description: Vec<String>,
  },
  Config {
    config: HashMap<String, String>,
  },
}

jcr_full!(JCROut {});
