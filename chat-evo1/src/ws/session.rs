use std::{
  collections::HashSet,
  sync::Arc,
  time::{Duration, Instant},
};

use actix::{
  fut, prelude::ContextFutureSpawner, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
  Handler, Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use service_common::service_config::ServiceConfig;
use uuid::Uuid;

use super::{
  models::{ClientMessage, Connect, Disconnect, Error},
  server::ChatServer,
};

pub struct WsChatSession {
  pub id: Uuid,
  pub hb: Instant,
  pub current_room_id: Option<Uuid>,
  pub available_rooms: HashSet<Uuid>,
  pub config: Arc<ServiceConfig>,
  pub addr: Addr<ChatServer>,
  pub user_id: Uuid,
}

impl Actor for WsChatSession {
  type Context = WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);
    let addr = ctx.address();
    self
      .addr
      .send(Connect {
        addr: addr.recipient(),
        available_rooms: self.available_rooms.clone(),
      })
      .into_actor(self)
      .then(|res, act, ctx| {
        match res {
          Ok(res) => act.id = res.unwrap_or(Uuid::new_v4()),
          Err(_) => ctx.stop(),
        }
        fut::ready(())
      })
      .wait(ctx);
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    // notify chat server
    self.addr.do_send(Disconnect {
      id: self.id.clone(),
    });
    Running::Stop
  }
}

impl WsChatSession {
  fn hb(&self, ctx: &mut WebsocketContext<Self>) {
    ctx.run_interval(Duration::from_secs(5), |act, ctx| {
      //get config
      if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
        // get config
        act.addr.do_send(Disconnect { id: act.id.clone() });
        ctx.stop();
        return;
      }
      ctx.ping(b"");
    });
  }
}

impl Handler<super::models::Message> for WsChatSession {
  type Result = ();

  fn handle(&mut self, msg: super::models::Message, ctx: &mut Self::Context) -> Self::Result {
    ctx.text(msg.0);
  }
}

impl StreamHandler<Result<Message, ProtocolError>> for WsChatSession {
  fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
    let msg = match item {
      Ok(msg) => msg,
      Err(_) => {
        ctx.stop();
        return;
      }
    };
    match msg {
      Message::Binary(_) => (),
      Message::Continuation(_) => {
        ctx.stop();
      }
      Message::Ping(message) => {
        self.hb = Instant::now();
        ctx.pong(&message)
      }
      Message::Pong(_) => {
        self.hb = Instant::now();
      }
      Message::Close(reason) => {
        ctx.close(reason);
        ctx.stop();
      }
      Message::Nop => (),
      Message::Text(text) => {
        if let Ok(message) = serde_json::from_str::<ClientMessage>(&text) {
          if self.available_rooms.contains(&message.room_id) {
            self.addr.do_send(ClientMessage {
              session_id: self.id.clone(),
              author_id: message.author_id,
              date: message.date,
              room_id: message.room_id,
              message: message.message,
              is_staff: message.is_staff,
            });
          } else {
            self.addr.do_send(Error {
              reason: "room unavailable".into(),
              addr: ctx.address().recipient(),
            });
          }
        } else {
          self.addr.do_send(Error {
            reason: "Unknown message".into(),
            addr: ctx.address().recipient(),
          });
        }
      }
    }
  }
}
