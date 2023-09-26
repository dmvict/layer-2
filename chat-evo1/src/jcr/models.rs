use serde::{Deserialize, Serialize};
use service_common::jcr_full;
use uuid::Uuid;

use crate::db::models::{Message, Pool, PoolInvitation, Room, RoomInvitation, Topic};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRIn {
  FetchUserRooms { user_id: Uuid },
  CreateNewRoom { room: Room },
  FetchMessages { room_id: Uuid,
    n: usize,
    size: usize, },
  FetchStaffRooms { staff_id: Uuid },
  PutRoom { room: Room },
  PutTopic { topic: Topic },
  FetchRoomWithoutStaff,
  CreatePool { pool: Pool },
  JoinRoom { room_id: Uuid, self_id: Uuid },
  JoinPool { pool_id: Uuid, self_id: Uuid },
  InvitePoolToRoom { pool_id: Uuid, room_id: Uuid },
  GetRoomInvitations { self_id: Uuid },
  GetPoolInvitations { self_id: Uuid },
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCRInChatDb {
  SaveMessage {
    message: Message,
  },
  FetchRoomMessages {
    room_id: Uuid,
    n: usize,
    size: usize,
  },
  FetchUserRooms {
    user_id: Uuid,
  },
  FetchStaffRooms {
    staff_id: Uuid,
  },
  PutRoom {
    room: Room,
  },
  PutTopic {
    topic: Topic,
  },
  FetchRoomWithoutStaff,
  CreatePool {
    pool: Pool,
  },
  JoinRoom {
    room_id: Uuid,
    self_id: Uuid,
  },
  JoinPool {
    pool_id: Uuid,
    self_id: Uuid,
  },
  InvitePoolToRoom {
    pool_id: Uuid,
    room_id: Uuid,
  },
  GetRoomInvitations {
    self_id: Uuid,
  },
  GetPoolInvitations {
    self_id: Uuid,
  },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subject", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum JCROut {
  Messages { messages: Vec<Message> },
  Rooms { rooms: Vec<Room> },
  TODO,
  Success { message: String },
  Error { description: Vec<String> },
  PoolInvitations { invitations: Vec<PoolInvitation> },
  RoomInvitations { invitations: Vec<RoomInvitation> },
}

jcr_full!(JCROut {});
