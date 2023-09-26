# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Input](#input)
    - [GenerateReceiptLink](#generatereceiptlink)
  - [Outputs](#outputs)
    - [DownloadLink](#downloadlink)
  - [Routes](#routes)
    - [GET `/download/{token}`](#get-downloadtoken)

## Input

### GenerateReceiptLink

Generates receipt link for specified user and intent id and returns [DownloadLink](#downloadlink) response.

```json
{
  "subject": "generate_receipt_link",
  "data": {
    "user_id": "string",
    "intent_id": "string"
  }
}
```

## Outputs

### DownloadLink

JCR wrapper for download link.

```json
{
  "subject": "download_link",
  "data": {
    "link": "string"
  }
}
```

## Routes

### GET `/download/{token}`

Retrieves stored file and sends it to the client.

Returns 404 if token is incorrect.
