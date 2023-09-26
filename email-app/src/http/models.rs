use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)] // remove default its for test add clone
pub struct PublicAddress {
  pub line1: String,
  pub line2: String,
  pub city: String,
  pub zip: i32,
  pub region: String,
  pub country: String,
}

#[derive(Debug, Deserialize, Serialize, Default)] // remove default its for test
pub struct PublicUser {
  pub email: String,
  pub website: String,
  pub first_name: String,
  pub last_name: String,
  pub phone_number: String,
  pub company_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)] // remove default its for test
pub struct NewUser {
  pub user: PublicUser,
  pub address: PublicAddress,
}

#[derive(Debug, Deserialize, Serialize, Default)] // remove default its for test
pub struct Auth {
  pub email: String,
  pub website: String,
  pub password: String,
}
