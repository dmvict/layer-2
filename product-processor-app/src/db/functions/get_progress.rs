use anyhow::Result;
use rand::Rng;

pub async fn get_progress(order_id: String) -> Result<String> {
  let progress = rand::thread_rng().gen_range(1..=100);
  Ok(format!("{}%", progress))
}
