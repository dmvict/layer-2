use serde::{Deserialize, Serialize};

pub mod routes;

#[derive(Debug, Deserialize, Serialize, Default)] // remove default its for test
pub struct PublicUser {
  pub email: String,
  pub website: String,
  pub first_name: String,
  pub last_name: String,
  pub phone_number: String,
  pub company_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tokens {
  #[serde(rename = "m_token")]
  pub m: String,
  #[serde(rename = "n_token")]
  pub n: String,
  #[serde(rename = "j_token")]
  pub j: String,
}
