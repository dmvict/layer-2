use base64::{
  alphabet::Alphabet,
  engine::{general_purpose, GeneralPurpose},
};
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref ENGINE: GeneralPurpose = {
    let alphabet =
      Alphabet::new("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ+_").unwrap();
    GeneralPurpose::new(&alphabet, general_purpose::PAD)
  };
  pub static ref BASLANGIC_TS: i64 = Utc
    .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
    .unwrap()
    .timestamp();
  pub static ref SIGNATURE: Vec<u8> =
    "awqd!'4cqT!ViCAD*(wq_-AU0KvhUvVc.gUxpjK5,3zy2(wWE)a*VJ$W-s-4HuHD"
      .as_bytes()
      .to_vec();
}

pub static CHARSET: &str =
  "!#$%&()*+,-./:;<>=?@[]^_`~{}|ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub static KEY_LIST: &[&str] = &["sY1A0QZ@brO!c@(Fy:^]alSIg)J,-lJ2"];
pub static KEY_X_LIST: &[&str] = &["].L5i~1R=H]!&]wyo_?{<!UO:_|fL6b_"];
