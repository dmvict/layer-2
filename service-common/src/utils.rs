use anyhow::Context;
use anyhow::Error;
use serde::{de::DeserializeOwned, Serialize};

pub fn hash(password: &str) -> String {
  password.into()
}

pub fn now() -> i64 {
  chrono::Utc::now().timestamp()
}

pub async fn make_request<T: ?Sized + Serialize, O: DeserializeOwned>(
  client: &reqwest::Client,
  url: &str,
  data: &T,
) -> Result<O, Error> {
  client
    .post(url)
    .json(data)
    .send()
    .await
    .context("Failed to send request.")?
    .json::<O>()
    .await
    .context("Failed to parse response.")
}

#[macro_export]
macro_rules! hashmap {
  (@single $($x:tt)*) => (());
  (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

  ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
  ($($key:expr => $value:expr),*) => {
      {
          let _cap = hashmap!(@count $($key),*);
          let mut _map = ::std::collections::HashMap::with_capacity(_cap);
          $(
              let _ = _map.insert($key, $value);
          )*
          _map
      }
  };
}
