use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use actix::Addr;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use service_common::service_config::ServiceConfig;
use service_common::JCR;
use uuid::Uuid;

use crate::db::functions::create_pool::create_pool;
use crate::db::functions::fetch_room_without_staff::fetch_room_without_staff;
use crate::db::functions::get_messages::get_messages;
use crate::db::functions::get_pool_invitations::get_pool_invitations;
use crate::db::functions::get_room_invitations::get_room_invitations;
use crate::db::functions::get_staff_rooms::get_staff_rooms;
use crate::db::functions::get_user_rooms::get_user_rooms;
use crate::db::functions::invite_pool_to_room::invite_pool_to_room;
use crate::db::functions::join_pool::join_pool;
use crate::db::functions::join_room::join_room;
use crate::db::functions::put_room::put_room;
use crate::db::functions::put_topic::put_topic;
use crate::db::models::Room;
use crate::jcr::models::{JCRIn, JCROut, JCResult};
use crate::ws::server::ChatServer;
use crate::ws::session::WsChatSession;

#[get("/")]
async fn get_index() -> JCResult {
  JCROut::success(chrono::Utc::now().format("%+").to_string())
}

#[post("/")]
async fn post_index(
  jcr: web::Json<JCRIn>,
  client: web::Data<reqwest::Client>,
  config: web::Data<ServiceConfig>,
) -> JCResult {
  let client = client.into_inner();
  let config = config.into_inner();
  let jcr = jcr.into_inner();
  match jcr {
    JCRIn::CreateNewRoom { room } => {
      put_room(room, &client, config).await?;
      JCROut::success("Room added")
    }
    JCRIn::FetchUserRooms { user_id } => {
      let rooms = get_user_rooms(user_id, &client, config).await?; 
      JCROut::Rooms { rooms }.ok()
    }
    JCRIn::FetchMessages { room_id, n, size } => {
      let messages = get_messages(room_id, n, size, &client, config).await?;
      JCROut::Messages { messages }.ok()
    }
    JCRIn::CreatePool { pool } => {
      create_pool(pool, &client, config).await?;
      JCROut::success("Pool created")
    }
    JCRIn::FetchStaffRooms { staff_id } => {
      let rooms = get_staff_rooms(staff_id, &client, config).await?;
      JCROut::Rooms { rooms }.ok()
    }
    JCRIn::PutRoom { room } => {
      put_room(room, &client, config).await?;
      JCROut::success("Room added")
    }
    JCRIn::PutTopic { topic } => {
      put_topic(topic, &client, config).await?;
      JCROut::success("Topic added")
    }
    JCRIn::FetchRoomWithoutStaff => {
      let rooms = fetch_room_without_staff(&client, config).await?;
      JCROut::Rooms { rooms }.ok()
    }
    JCRIn::JoinRoom { room_id, self_id } => {
      join_room(room_id, self_id, &client, config).await?;
      JCROut::success("Joined")
    }
    JCRIn::JoinPool { pool_id, self_id } => {
      join_pool(pool_id, self_id, &client, config).await?;
      JCROut::success("Joined")
    }
    JCRIn::InvitePoolToRoom { pool_id, room_id } => {
      invite_pool_to_room(pool_id, room_id, &client, config).await?;
      JCROut::success("Invited")
    }
    JCRIn::GetRoomInvitations { self_id } => {
      let invitations = get_room_invitations(self_id, &client, config).await?;
      JCROut::RoomInvitations { invitations }.ok()
    }
    JCRIn::GetPoolInvitations { self_id } => {
      let invitations = get_pool_invitations(self_id, &client, config).await?;
      JCROut::PoolInvitations { invitations }.ok()
    }
  }
}

#[get("/ws/{user_id}/{is_staff}")]
async fn chat_route(
  req: HttpRequest,
  stream: web::Payload,
  config: web::Data<ServiceConfig>,
  client: web::Data<reqwest::Client>,
  srv: web::Data<Addr<ChatServer>>,
  user_id: web::Path<Uuid>,
  is_stuff: web::Path<bool>, //need to fetch available rooms
) -> Result<HttpResponse, Error> {
  let config = config.into_inner();
  let user_id = user_id.into_inner();
  let is_staff = is_stuff.into_inner();
  let client = client.into_inner();
  let available_rooms = fetch_rooms(user_id, is_staff, &client, config.clone())
    .await?
    .into_iter()
    .map(|r| r.room_id.unwrap_or_default())
    .collect();
  ws::start(
    WsChatSession {
      addr: srv.get_ref().clone(),
      id: Uuid::new_v4(),
      hb: Instant::now(),
      current_room_id: None,
      available_rooms,
      config,
      user_id,
    },
    &req,
    stream,
  )
}

async fn fetch_rooms(
  id: Uuid,
  is_staff: bool,
  client: &reqwest::Client,
  config: Arc<ServiceConfig>,
) -> Result<Vec<Room>, Error> {
  if is_staff {
    get_staff_rooms(id, client, config)
      .await
      .map_err(|_| error::ErrorNotFound("Fail to fetch rooms"))
  } else {
    get_user_rooms(id, client, config)
      .await
      .map_err(|_| error::ErrorNotFound("Fail to fetch rooms"))
  }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(get_index).service(post_index);
}
