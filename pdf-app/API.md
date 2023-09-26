# JCR API

Description of various functions for this service and their return objects.

- [JCR API](#jcr-api)
  - [Input](#input)
    - [Generate](#generate)

## Input

### Generate

Generates a PDF file from specified template name and context.

Uses cached template if needed.
If there's no template in cache attempts to read from db.

Returns raw bytes of generated file.

```json
{
  "subject": "generate",
  "data": {
    "template": "string",
    "context": {
      "string": value,
      ...
    },
    "cached": bool
  }
}
```
