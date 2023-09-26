use anyhow::{anyhow, Ok, Result};
use base64::Engine;
use chacha20poly1305::{
  aead::{Aead, KeyInit},
  XChaCha20Poly1305,
};
use chrono::{DateTime, Duration, Utc};
use ed25519_compact::{Noise, Signature};
use random_string::generate;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::models::{Fu, Keys, VerifySignature};
use super::statics::*;

#[derive(Debug, Deserialize, Serialize)]
struct Jwt {
  data: String,
  signature: String,
}

fn decode_encrypted_line(line: &str) -> Result<String> {
  if line.len() < 60 {
    anyhow::bail!("Invalid encrypted line")
  }
  let (encrypted_registry_60, ec60) = line.split_at(line.len() - 60);
  //
  let ec: String = ENGINE.decode(ec60)?.iter().map(|b| *b as char).collect();
  let st_nonce = &ec[..24];
  let today = &ec[24..30];
  let dx = Fu::day_from_today(today);
  //
  let key = Keys::key_xcha(dx);
  let mut nonce = [0u8; 24];
  nonce.copy_from_slice(st_nonce.as_bytes());
  //
  let encrypted = ENGINE.decode(encrypted_registry_60)?;
  XChaCha20Poly1305::new(&key.into())
    .decrypt(&nonce.into(), &*encrypted)
    .map(|dec| dec.into_iter().map(|b| b as char).collect())
    .map_err(|_| anyhow!("Token decryption failed"))
}

fn make_ec60(nstring: String, day: &str) -> String {
  let brr = generate(15, CHARSET); // 60 say
  let ec = nstring + day + brr.as_ref();
  let ec60 = ENGINE.encode(ec.as_bytes());
  ec60
}

pub fn jwt_encode<T: Serialize>(jc: T) -> Result<String> {
  let data = serde_json::to_string(&jc)?;
  // sign data make signature
  let day = Fu::get_today();
  let dx = Fu::get_day();
  let ky = Keys::key_pair(dx)?;
  let key = Keys::key_xcha(dx);
  let (nonce, nstring) = Keys::nonce_st();
  //
  let signature = ky.sk.sign(data.as_bytes(), Some(Noise::generate()));
  let sgvu = *signature;
  let sig_st = ENGINE.encode(sgvu);
  let jwt = Jwt {
    data,
    signature: sig_st,
  };
  let jwt_st = serde_json::to_string(&jwt)?;
  XChaCha20Poly1305::new(&key.into())
    .encrypt(&nonce.into(), jwt_st.as_bytes())
    .map(|enc| ENGINE.encode(enc) + &make_ec60(nstring, &day))
    .map_err(|_| anyhow!("Token encryption failed"))
}

pub fn jwt_decode<T: DeserializeOwned>(token: &str) -> Result<T> {
  let jwt_st = decode_encrypted_line(token)?;
  let jwt: Jwt = serde_json::from_str(&jwt_st)?;

  let signature_vec = ENGINE.decode(jwt.signature)?.verify_or_default();
  let signature = Signature::from_slice(&signature_vec[..])?;

  let current_day = Fu::get_day();
  let key_pair = Keys::key_pair(current_day)?;
  key_pair.pk.verify(&jwt.data, &signature)?;

  let data = serde_json::from_str(&jwt.data)?;
  Ok(data)
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

impl Tokens {
  pub fn generate() -> Self {
    Self {
      m: generate_m_token(),
      n: generate_n_token(),
      j: generate_j_token(),
    }
  }

  pub fn decode<T: DeserializeOwned>(&self) -> Result<T> {
    jwt_decode(&self.j)
  }

  pub fn build_upload_link(&self, base: &str) -> Result<String> {
    let jcr = super::jcr::JCR::ReqUpload {
      data: self.j.clone(),
    };
    let jt = jwt_encode(&jcr)?;

    Ok(format!("{base}/{}/{}/{}", self.m, self.n, jt))
  }
}

static MNJ_CHARSET: &str = "QWERTYUIOPASDFGHJKLpoiuytrewqlkjhgfdsamnbvcxzZXCVBNM1234567890";
// j_token 32 chars
pub fn generate_j_token() -> String {
  generate(32, MNJ_CHARSET)
}

// m_token 8 chars
pub fn generate_m_token() -> String {
  generate(8, MNJ_CHARSET)
}
// n_token 12 chars
pub fn generate_n_token() -> String {
  generate(12, MNJ_CHARSET)
}

#[inline]
pub fn get_upload_link_duration() -> i64 {
  10
}

#[inline]
pub fn token_exp(minutes: i64) -> DateTime<Utc> {
  Utc::now() + Duration::minutes(minutes)
}

#[inline]
pub fn get_max_length() -> u64 {
  1024 * 1024 * 10
}

#[inline]
pub fn get_max_files() -> u64 {
  10
}
