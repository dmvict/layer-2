# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Input](#input)
    - [GenerateToken](#generatetoken)
  - [Outputs](#outputs)
    - [UploadLink](#uploadlink)
  - [Routes](#routes)
    - [POST `/upload/{token}`](#post-uploadtoken)

## Input

### GenerateToken

Generates upload token for specified user.

Returns [UploadLink](#uploadlink).

```json
{
  "subject": "generate_token",
  "data": {
    "user_id": "string"
  }
}
```

## Outputs

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

## Routes

### POST `/upload/{token}`

Route for sending form multipart-encoded files.

Returns generic `Success` response.
