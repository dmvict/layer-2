# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Objects](#objects)
  - [Input](#input)
    - [GetProgress](#getprogress)
  - [Outputs](#outputs)
    - [OrderProgress](#orderprogress)

## Objects

## Input

### GetProgress

Returns [OrderProgress](#orderprogress) for specified order.

```json
{
  "subject": "get_progress",
  "data": {
    "order_id": "string"
  }
}
```

## Outputs

### OrderProgress

JCR wrapper over progress string.

```json
{
  "subject": "order_progress",
  "data": {
    "progress": "string"
  }
}
```
