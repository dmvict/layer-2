use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use crate::{
  db::{functions::save_message::save_message, models::Message},
  jcr::models::{JCRInChatDb, JCROut},
};

use super::models::Error as WsError;
use super::models::*;
use actix::prelude::*;
use anyhow::{bail, Context as AContext};
use chrono::Local;
use service_common::{make_request, service_config::ServiceConfig};
use tokio::task;
use uuid::Uuid;

pub struct ChatServer {
  sessions: HashMap<Uuid, Recipient<super::models::Message>>,
  rooms: HashMap<Uuid, HashSet<Uuid>>,
  config: Arc<ServiceConfig>,
  client: Arc<reqwest::Client>,
}

impl ChatServer {
  pub fn new(config: Arc<ServiceConfig>, client: Arc<reqwest::Client>) -> ChatServer {
    ChatServer {
      sessions: HashMap::new(),
      rooms: HashMap::new(),
      config,
      client,
    }
  }
}

impl ChatServer {
  /// Send message to all users in the room
  fn send_message(&self, room: &Uuid, message: &str, self_id: Uuid) {
    if let Some(sessions) = self.rooms.get(room) {
      for id in sessions {
        if *id != self_id {
          if let Some(addr) = self.sessions.get(id) {
            addr.do_send(super::models::Message(message.to_owned()));
          }
        }
      }
    }
  }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
  /// We are going to use simple Context, we just need ability to communicate
  /// with other actors.
  type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
  type Result = Result<Uuid, WsError>;

  fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
    let id = Uuid::new_v4();
    self.sessions.insert(id, msg.addr);
    for ele in msg.available_rooms {
      self
        .rooms
        .entry(ele.clone())
        .or_default()
        .insert(id.clone());
    }
    Ok(id)
  }
}

impl Handler<Disconnect> for ChatServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
    let mut rooms: Vec<Uuid> = Vec::new();

    // remove address
    if self.sessions.remove(&msg.id).is_some() {
      // remove session from all rooms
      for (name, sessions) in &mut self.rooms {
        if sessions.remove(&msg.id) {
          rooms.push(name.to_owned());
        }
      }
    }
  }
}

impl Handler<ClientMessage> for ChatServer {
  type Result = ();

  fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
    if let Ok(m) = serde_json::to_string(&msg) {
      self.send_message(&msg.room_id, &m, msg.session_id.clone());
      let is_staff = || {
        if msg.is_staff {
          return 1;
        }
        0
      };
      let message = Message {
        message_sent_at: msg.date,
        content: msg.message,
        author_id: msg.author_id,
        room_id: msg.room_id,
        is_staff: is_staff(),
      };
      let _ = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(save_message(message, &self.client, self.config.clone()));
    }
  }
}

impl Handler<WsError> for ChatServer {
  type Result = ();

  fn handle(&mut self, msg: WsError, _: &mut Self::Context) -> Self::Result {
    if let Ok(message) = serde_json::to_string(&msg) {
      msg.addr.do_send(Message(message));
    }
  }
}
