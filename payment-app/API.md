# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Input](#input)
    - [PutAccount](#putaccount)
    - [PutCompany](#putcompany)
    - [PutIntent](#putintent)
    - [GetRequiredAccount](#getrequiredaccount)
    - [GenerateFile](#generatefile)
  - [Outputs](#outputs)
    - [File](#file)
  - [Webhook](#webhook)
    - [POST `/stripe_webooks`](#post-stripe_webooks)

## Input

### PutAccount
Inserts new, or updates existing, account on the db.

Returns generic `Success` response.

```json
{
  "subject": "put_account",
  "data": {
    "id": "string",
    "company_name": "string",
    "secret": "string",
    "picked": i32,
    "registration_date" : "yyyy-mm-dd"
  }
}
```
### PutCompany

Inserts new, or updates existing, company on the db.

Returns generic `Success` response.

```json
{
  "subject": "put_company",
  "data": {
    "company_name": "string",
    "switch_method": "string",
    "send_email": i32,
    "send_attachment": i32
  }
}
```

### PutIntent

Inserts new, or updates existing, intent on the db.

Returns generic `Success` response.

```json
{
  "subject": "put_intent",
  "data": {
    "id": "string",
    "currency": "string",
    "amount": i64,
    "email": "string",
    "description": "string",
    "name": "string",
    "account_id": "string"
  }
}
```

### GetRequiredAccount

Considering the account change algorithm returns the `account_id` to which it is worth creating an intent by company name.

Returns [AccountId](#accountid).

```json
{
  "subject": "get_required_account",
  "data": {
    "company_name": "string"
  }
}
```

### GenerateFile

Generates a PDF file for specified payment intent id.
Returns [File](#file).

```json
{
  "subject": "generate_file",
  "data": {
    "payment_intent_id": "string"
  }
}
```

## Outputs

### File

JCR wrapper over a file byte array.

```json
{
  "subject": "file",
  "data": {
    "file": [ u8 ]
  }
}
```

### AccountId

JCR wrapper over a `account_id`.

```json
{
  "subject": "account_id",
  "data": {
    "id": "string"
  }
}
```

## Webhook

### POST `/stripe_webooks`

Handles responses from Stripe.

Generates receipt and sends email if needed.

Always returns 200 http code.