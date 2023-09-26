# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Objects](#objects)
    - [PublicUserObject](#publicuserobject)
    - [PublicAddressObject](#publicaddressobject)
    - [AuthObject](#authobject)
  - [Input](#input)
    - [Register](#register)
    - [Login](#login)
    - [UpdateUser](#updateuser)
    - [GenerateUploadLink](#generateuploadlink)
  - [Outputs](#outputs)
    - [Token](#token)
    - [UploadLink](#uploadlink)

## Objects

### PublicUserObject

Describes simplified [UserObject](#userobject) for input.

```json
{
  "email": "string",
  "website": "string",
  "first_name": "string",
  "last_name": "string",
  "phone_number": "string",
  "company_id": "string"
}
```

### PublicAddressObject

Describes simplified [AddressObject](#addressobject) for input.

```json
{
  "line1": "string",
  "line2": "string",
  "city": "string",
  "zip": i32,
  "region": "string",
  "country": "string"
}
```

### AuthObject

Describes minimal information needed for user to login.

```json
{
  "email": "string",
  "website": "string",
  "password": "string"
}
```

## Input

### Register

Verifies and registers user.

Input is [PublicUserObject](#publicuserobject), [PublicAddressObject](#publicaddressobject), and [AuthObject](#authobject).

Returns [Token](#token).

```json
{
  "subject": "register",
  "data": {
    "user": PublicUserObject,
    "address": PublicAddressObject,
    "auth": AuthObject
  }
}
```

### Login

Verifies and fetches existing user.

Input is [AuthObject](#authobject).

Returns [Token](#token).

```json
{
  "subject": "login",
  "data": {
    "auth": AuthObject
  }
}
```

### UpdateUser

Updates existing user by email & website pair.

Returns generic `Success` response.

```json
{
  "subject": "update_user",
  "data": {
    "user": {
      "email": "string",
      "website": "string"
    },
    "new_user": {
      "user": PublicUserObject,
      "address": PublicAddressObject
    }
  }
}
```

### GenerateUploadLink

Generates upload link for specified user.

Returns [UploadLink](#uploadlink).

```json
{
  "subject": "generate_token",
  "data": {
    "user": PublicUserObject
  }
}
```

## Outputs

### Token

Auth token for the user.

```json
{
  "subject": "token",
  "data": {
    "token": "string"
  }
}
```

### UploadLink

JCR wrapper over link string.

```json
{
  "subject": "upload_link",
  "data": {
    "link": "string"
  }
}
```
