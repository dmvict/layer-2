use std::collections::HashSet;

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, Default)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New chat session is created
#[derive(Message)]
#[rtype(result = "Result<Uuid, Error>")]
pub struct Connect {
  pub addr: Recipient<Message>,
  pub available_rooms: HashSet<Uuid>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub id: Uuid,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ClientMessage {
  #[serde(skip)]
  pub session_id: Uuid,
  pub author_id: Uuid,
  pub date: String,
  pub room_id: Uuid,
  pub message: String,
  pub is_staff: bool,
}

#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub struct Error {
  #[serde(skip_serializing)]
  pub addr: Recipient<Message>,
  pub reason: String,
}
