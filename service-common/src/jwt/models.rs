use anyhow::Result;
use chrono::{TimeZone, Utc};
use ed25519_compact::{KeyPair, Seed};
use random_string::generate;

use super::statics::*;

pub trait VerifySignature {
  fn verify_or_default(self) -> Vec<u8>;
}

#[rustfmt::skip]
impl VerifySignature for Vec<u8> {
  fn verify_or_default(self) -> Vec<u8> {
    if self.len() != 64 { SIGNATURE.clone() } else { self }
  }
}

pub struct Keys;

impl Keys {
  pub fn key_pair(_dx: u32) -> Result<KeyPair> {
    let key = KEY_LIST[0_usize];
    let se_seed = Seed::from_slice(key.as_bytes())?;
    Ok(KeyPair::from_seed(se_seed))
  }

  pub fn key_xcha(_dx: u32) -> [u8; 32] {
    let key_st = KEY_X_LIST[0_usize];
    let mut key = [0u8; 32];
    key.copy_from_slice(key_st.as_bytes());
    key
  }

  pub fn nonce_st() -> ([u8; 24], String) {
    let mut nonce = [0u8; 24];
    let nb = generate(24, CHARSET);
    nonce.copy_from_slice(nb.as_bytes());
    (nonce, nb)
  }
}

pub struct Fu;

impl Fu {
  pub fn get_day() -> u32 {
    let dur = Utc::now().timestamp() - *BASLANGIC_TS;
    let bugun = dur / 86400 + 1;
    bugun as u32
  }

  pub fn get_today() -> String {
    let dd = Utc::now().date_naive();
    dd.format("%m%d%y").to_string()
  }

  pub fn day_from_today(today: &str) -> u32 {
    let d_utc = Utc
      .datetime_from_str(&format!("{today}120000"), "%m%d%y%H%M%S")
      .unwrap();
    let dur = d_utc.timestamp() - *BASLANGIC_TS;
    let bugun = dur / 86400 + 1;
    bugun as u32
  }
}
