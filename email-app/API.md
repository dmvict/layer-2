# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Objects](#objects)
    - [ReceiptObject](#receiptobject)
  - [Input](#input)
    - [SendReceipt](#sendreceipt)

## Objects

### ReceiptObject

Describes receipt to send email.

```json
{
  "amount": i64,
  "currency": "string",
  "email": "string",
  "description": "string",
  "name": "string"
}
```

## Input

### SendReceipt

Sends an email to the address specified in a [ReceiptObject](#receiptobject).
Returns generic `Success` response.

```json
{
  "subject": "send_receipt",
  "data": {
    "receipt": ReceiptObject
  }
}
```
