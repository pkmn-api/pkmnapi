# Geting an Access Token

All requests must be authenticated with an access token.

To generate one, send a `POST` request to the `/v1/access_tokens` endpoint along with your email address:

```bash
curl \
    -X POST \
    -H 'Content-Type: application/json' \
    -d '{
    "data": {
        "type": "access_tokens",
        "attributes": {
            "email_address": "email@example.com"
        }
    }
}' \
    https://api.pkmnapi.com/v1/access_tokens
```

An email will be sent to the email address provided and will include your access token.

[[Up]](../index.md) - [[Next - Using An Access Token]](../02-using-an-access-token/index.md)
