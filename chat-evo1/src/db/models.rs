use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
  pub message_sent_at: String,
  pub content: String,
  pub author_id: Uuid,
  pub room_id: Uuid,
  pub is_staff: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Room {
  pub room_id: Option<Uuid>,
  pub title: String,
  #[serde(skip_serializing)]
  pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Topic {
  pub topic_id: Option<Uuid>,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
  pub name: String,
  #[serde(skip_serializing)]
  pub topic_id: Uuid,
  #[serde(skip_serializing)]
  pub initiator_id: Uuid,
  #[serde(skip_deserializing)]
  pub topic_name: String,
  #[serde(skip_deserializing)]
  pub initiator_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolInvitation {
  pub pool_id: Uuid,
  pub name: String,
  pub initiator_name: String,
  pub initiator_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInvitation {
  pub room_id: Uuid,
  pub name: String,
}
