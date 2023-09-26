use std::str::FromStr;

use anyhow::{bail, Context, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Receipt {
  pub amount: i64,
  //in cent
  pub currency: String,
  pub email: String,
  pub description: String,
  pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicFileMeta {
  pub name: String,
  pub ext: String,
  pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntentEntity {
  pub id: String,
  pub currency: String,
  pub amount: i64,
  pub email: String,
  pub description: String,
  pub name: String,
  pub secret: String,
}

impl From<IntentEntity> for Receipt {
  fn from(value: IntentEntity) -> Self {
    Self {
      amount: value.amount,
      currency: value.currency,
      email: value.email,
      description: value.description,
      name: value.name,
    }
  }
}

pub struct WebhookEventVerifier {
  current_timestamp: i64,
}

impl WebhookEventVerifier {
  pub fn verify_request(raw_json: &str, sig: &str, secret: &str) -> Result<()> {
    Self {
      current_timestamp: Utc::now().timestamp(),
    }
    .do_construct_event(raw_json, sig, secret)
  }

  fn do_construct_event(self, raw_json: &str, sig: &str, secret: &str) -> Result<()> {
    // Get Stripe signature from header
    let signature = Signature::parse(sig)?;
    let signed_payload = format!("{}.{}", signature.t, raw_json);

    // Compute HMAC with the SHA256 hash function, using endpoing secret as key
    // and signed_payload string as the message.
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
    mac.update(signed_payload.as_bytes());

    let sig = hex::decode(signature.v1)?;

    mac.verify_slice(sig.as_slice())?;

    // Get current timestamp to compare to signature timestamp
    if (self.current_timestamp - signature.t).abs() > 300 {
      bail!("Signature expired")
    }
    Ok(())
  }
}

#[derive(Debug)]
struct Signature<'r> {
  t: i64,
  v1: &'r str,
}

impl<'r> Signature<'r> {
  fn parse(raw: &'r str) -> Result<Signature<'r>> {
    use std::collections::HashMap;
    let headers: HashMap<&str, &str> = raw
      .split(',')
      .map(|header| {
        let mut key_and_value = header.split('=');
        let key = key_and_value.next();
        let value = key_and_value.next();
        (key, value)
      })
      .filter_map(|(key, value)| match (key, value) {
        (Some(key), Some(value)) => Some((key, value)),
        _ => None,
      })
      .collect();
    let t = headers.get("t").context("Bad signature")?;
    let v1 = headers.get("v1").context("Bad signature")?;
    Ok(Signature {
      t: t.parse::<i64>()?,
      v1,
    })
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct Object {
  pub(crate) id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Data {
  pub(crate) object: Object,
}

#[derive(Debug, serde::Deserialize)]
pub struct Event {
  pub(crate) data: Data,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SwitchMethodWithAccounts {
  pub switch_method: String,
  pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
  pub id: String,
  pub company_name: String,
  pub secret: String,
  pub picked: i32,
  pub registration_date: String,
}

pub enum SwitchMethod {
  MANUAL,
  DEFAULT,
  ROUNDROBIN,
  LEASTPAID,
  CAP { amount: i64 },
}

impl FromStr for SwitchMethod {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "MANUAL" => Ok(Self::MANUAL),
      "DEFAULT" => Ok(Self::DEFAULT),
      "ROUNDROBIN" => Ok(Self::ROUNDROBIN),
      "LEASTPAID" => Ok(Self::LEASTPAID),
      _ => {
        if s.starts_with("CAP_") {
          if let Ok(amount) = &s[4..s.len()].parse::<i64>() {
            return Ok(Self::CAP { amount: *amount });
          }
        }
        bail!("Fail to parse switch method from '{}' str", s)
      }
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountMonthlyBalance {
  pub account_id: String,
  pub balance: i64,
}
