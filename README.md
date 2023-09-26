# SHOP Project

- [SHOP Project](#shop-project)
  - [Common](#common)
  - [Services Layer](#services-layer)
    - [Services](#services)
    - [Requirements](#requirements)
  - [API](#api)

## Common

- Build a 3 layer project structure, where client would have open internet access to 
interface layer applications, internal services layer and databases layer to be closed to client access.
- Use jwt enc JCR (new) api for inter app communication.
- Provide unified non enc JCR api to JS client at interface level.
- Apps at Interface should be pluggable ... somehow
- Services at level 2 are already standalone

## Services Layer

### Services

- UserApp (central control)
- UploadApp
- DownloadApp
- EmailApp
- PaymentsAPP
- ProductProcessor
- AdminApp 

### Requirements

- UserApp to change public_id X user_id and call relevant services (obviouslyI'm worried about security)
- UserApp to interact with UserDb + AuthDB
- UserApp to provide refresh and access tokens and https only cookies and refresh them at every http call
- UserApp to request signed file urls
- UserApp to update User profile data, save old in a vector
- Links: activate_user_link, forgot_my_password_link, we_didn't_recognize_your_device_link ... store tokens with subject then 
services will be able to act according to subject
- change country not allowed (tax cheating etc.. long story)
- DownloadApp upon user req will generate files using given data, and download link with limited time. ie 
upon completed purchase order data will be saved at user-db , when user req pdf file will be generated and link provided for user to download from dashboard.
When payment completed data saved at user-db, when user req to download receipt from dashboard, pdf will be generated and link provided.
- Later I'll provide design templates, for dev we will use any free templates
- EmailApp : we will use postmark api, I will need to store there some templates , when we send emails we only provide relevant data and tell which template ton be used, that one will be very easy.
- PaymentsAPP: Stripe
- Stripe offers JS only solution to make payment online, thats all on frontend. How do we know what the h** is going on? 
We will build webhooks(rest apis) stripe will notify our service. Initially we will simulate their responses: pending accepted denied completed ok etc.
- we can provide that feedback to frontend upon req
- when our interface api is partially ready, just enough to let Js developer to work, then we can receive real test signal from stripe.
- When payment accepted ok, then then we flag order as paid and notify ProductProcessor
- ProductProcessor : at the moment it will simulate progress object : random string + rand % rand sleep
we will provide that feedback to frontend upon request
when 100% then we flag order completed and store order -> past orders with unique order_id
- uploaded file names to be replaced with unique file ids, unique file ids for generated files
unique ids for email templates
- incoming emails : ... TBD (about admin , costumer care, contact, info)
- AdminApp: UserApp + additional functions (yes separate app... TBD)

## API

The general structure for all requests and responses is a JCR object.

JCR object only has a function name that will be executed on the target server - field `subject`.

And a `data` field which represents arguments to the function.

Both input and output are JCR objects serialized to JSON.
```json
{
  "subject": string,
  "data": object
}
```

Common JCR responses:

`Success`
```json
{
  "subject": "success",
  "data": {
    "message": "All good"
  }
}
```
`Error`
```json
{
  "subject": "error",
  "data": {
    "description": [
      "Something failed",
      "Stack trace",
      ...
    ]
  }
}
```

Each service has their own input/output JCR objects:
- [download-app](download-app/API.md)
- [email-app](email-app/API.md)
- [payment-app](payment-app/API.md)
- [pdf-app](pdf-app/API.md)
- [product-processor-app](product-processor-app/API.md)
- [upload-app](upload-app/API.md)
- [user-app](user-app/API.md)
