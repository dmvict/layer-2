# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Objects](#objects)
    - [Message](#message)
    - [Room](#room)
    - [Topic](#topic)
    - [Pool](#pool)
    - [PoolInvitation](#poolinvitation)
    - [RoomInvitation](#roominvitation)
  - [Input](#input)
    - [FetchRoomMessages](#fetchroommessages)
    - [FetchUserRooms](#fetchuserrooms)
    - [FetchStaffRooms](#fetchstaffrooms)
    - [PutRoom](#putroom)
    - [PutTopic](#puttopic)
    - [FetchRoomWithoutStaff](#fetchroomwithoutstaff)
    - [CreatePool](#createpool)
    - [JoinRoom](#joinroom)
    - [JoinPool](#joinpool)
    - [InvitePoolToRoom](#invitepooltoroom)
    - [GetRoomInvitations](#getroominvitations)
    - [GetPoolInvitations](#getpoolinvitations)
  - [Outputs](#outputs)
    - [Messages](#messages)
    - [Rooms](#rooms)
    - [PoolInvitations](#poolinvitations)
    - [RoomInvitations](#roominvitations)
  - [WebSocket](#websocket)
    - [Connection](#connection)
    - [ClientMessage](#clientmessage)
    - [Error](#error)

## Objects

### Room

Describes a room. 
room_id is an optional parameter, specify it if you want to update the room or not if you want to create a new.
When it is used as a return parameter, it will be without user_id.

```json
{
  "room_id": "string", //optional
  "title": "string",
  "user_id": "string"
}
```

### Topic

Describes a topic.
topic_id is an optional parameter, specify it if you want to update the topic or not if you want to create a new.

```json
{
  "topic_id": "string", //optional
  "name": "string"
}
```

### Pool

Describe a pool.
When it is used as a return parameter, it will be without topic_id and initiator_id.
When it is used as a input parameter, it will be without topic_name and initiator_name.

```json
{
  "name": "string",
  "topic_id": "string",
  "initiator_id": "string",
  "topic_name": "string",
  "initiator_name": "string"
}
```

### PoolInvitation

Describes an staff’s invitation to join pool.

```json
{
  "pool_id": "string",
  "name": "string",
  "initiator_name": "string",
  "initiator_id": "string"
}
```

### RoomInvitation

Describes an staff’s invitation to join room.

```json
{
  "room_id": "string",
  "name": "string",
} 
```

## Input

### SaveMessage

Inserts new message on the db.

Input is [Message](#message).

Returns generic `Success` response.

```json
{
  "subject": "save_message",
  "data": Message
}
```

### FetchRoomMessages

Returns an [Message](#message) by it's `room_id`.

Input is `room_id`, `n` - page number and `size` - size of page.

```json
{
  "subject": "put_company",
  "data": {
    "room_id": "string",
    "n": usize,
    "size": usize
  }
}
```

### FetchStaffRooms

Returns an [Room](#room) by `staff_id`.

Input is `staff_id`.

```json
{
  "subject": "fetch_staff_rooms",
  "data": {
    "staff_id": "string"
  }
}
```

### FetchUserRooms

Returns an [Room](#room) by `user_id`.

Input is `user_id`.

```json
{
  "subject": "fetch_user_rooms",
  "data": {
    "user_id": "string"
  }
}
```

### PutRoom

Inserts new, or updates existing, room on the db.

Input is [Room](#room).

Returns generic `Success` response.

```json
{
  "subject": "put_room",
  "data": {
    "room": Room
  }
}
```

### PutTopic

Inserts new, or updates existing, topic on the db.

Input is [Topic](#topic).

Returns generic `Success` response.

```json
{
  "subject": "put_topic",
  "data": {
    "topic": Topic
  }
}
```
### CreatePool

Inserts new pool on the db.

Input is [Pool](#poll).

Returns generic `Success` response.

```json
{
  "subject": "create_pool",
  "data": {
    "pool": Pool
  }
}
```
### FetchRoomWithoutStaff

Returns an [Room](#room) where there is no staff.


```json
{
  "subject": "fetch_room_without_staff",
}
```
### JoinRoom

Update related room_invitation record.

Input is `room_id` and `self_id`(staff_id).

Returns generic `Success` response.


```json
{
  "subject": "join_room",
  "data": {
    "room_id": "string",
    "self_id": "string"
  }
}
```
### JoinPool

Update related pool_invitation record.

Input is `pool_id` and `self_id`(staff_id).

Returns generic `Success` response.


```json
{
  "subject": "join_pool",
  "data": {
    "pool_id": "string",
    "self_id": "string"
  }
}
```
### InvitePoolToRoom

Create invitation to all pool member.

Input is `pool_id` and `room_id`.

Returns generic `Success` response.


```json
{
  "subject": "invite_pool_to_room",
  "data": {
    "pool_id": "string",
    "room_id": "string"
  }
}
```
### GetPoolInvitations

Returns an unaccepted [PoolInvitation]`s(#poolinvitation) by staff_id.


```json
{
  "subject": "get_pool_invitations",
  "data": {
    "self_id": "string"
  }
}
```
### GetRoomInvitations

Returns an unaccepted [RoomInvitation]`s(#roominvitation) by staff_id.


```json
{
  "subject": "get_room_invitations",
  "data": {
    "self_id": "string"
  }
}
```

## Outputs

### Messages

JCR wrapper over [Message](#message).

```json
{
  "subject": "messages",
  "data": {
    "messages": [
      Message,
      Message,
      ...
    ]
  }
}
```

### Rooms

JCR wrapper over [Room](#room).

```json
{
  "subject": "rooms",
  "data": {
    "rooms": [
      Room,
      Room,
      ...
    ]
  }
}
```
### PoolInvitations

JCR wrapper over [PoolInvitation](#poolinvitation).

```json
{
  "subject": "pool_invitations",
  "data": {
    "pool_invitations": [
      PoolInvitation,
      PoolInvitation,
      ...
    ]
  }
}
```
### RoomInvitations

JCR wrapper over [RoomInvitation](#roominvitation).

```json
{
  "subject": "room_invitations",
  "data": {
    "room_invitations": [
      RoomInvitation,
      RoomInvitation,
      ...
    ]
  }
}
```

## WebSocket

### Connection

By this route `/ws/{user_id}/{is_staff}` you can create ws connection.
user_id - string,
is_staff - bool.
Params need to fetch room's.

### ClientMessage

Describe user message.
Use as input and output value.
User send this messages and receive this messages, if user present in related room.

```json
{
  "author_id": "string",
  "date": "yyyy-mm-dd hh:mm:ss",
  "room_id": "string",
  "message": "string",
  "is_staff": bool
}
```

### Error

Describe ws error.
Use as output value.
```json
{
    "reason": "string"
}
```
