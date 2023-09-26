//remove uuid

use serde::{Deserialize, Serialize};

use crate::http::models::PublicAddress;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Address {
  pub id: String,
  pub line1: String,
  pub line2: String,
  pub city: String,
  pub zip: i32,
  pub region: String,
  pub country: String,
  pub update_number: i64,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
  pub user_id: String,
  pub email: String, // email+website is unique identifier for the user !!!
  pub website: String,
  pub first_name: String,
  pub last_name: String,
  pub phone_number: String,
  pub address_id: String, // current address only
  pub company_id: String,
  pub country_unlock_approved: bool, // **
  pub email_confirmed: bool,
  pub is_active: bool, // activate after email confirmed and after address provided
  pub login_fails: i64, // set up env variable... to check limit if login success before limit erase that 12
  pub is_locked: bool,
  pub last_login: i64,      // b
  pub last_login_fail: i64, // update it at every fail
  pub update_number: i64,
  pub created_at: i64,
  pub updated_at: i64, // do not make it auto / update it with update mutation
}

#[derive(Debug, Deserialize, Serialize, Clone)] // remove default its for test
pub struct AuthEntry {
  pub user_id: String,
  pub public_id: i64,
  pub email: String,
  pub website: String,
  pub hashed_password: String,
  pub wrong_login: i64,
  pub update_number: i64,
  pub created_at: i64,
  pub updated_at: i64,
}

impl From<Address> for PublicAddress {
  fn from(value: Address) -> Self {
    Self {
      line1: value.line1,
      line2: value.line2,
      city: value.city,
      zip: value.zip,
      region: value.region,
      country: value.country,
    }
  }
}
