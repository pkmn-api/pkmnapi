+++
title = "Access Tokens"
weight = 1
+++

| Endpoint                                                   | Description                       |
|------------------------------------------------------------|-----------------------------------|
| [POST /v1/access_tokens](#post-access-token)               | Creates a new access token        |
| [POST /v1/access_tokens/delete](#post-access-token-delete) | Requests an access token deletion |
| [DELETE /v1/access_tokens](#delete-access-token)           | Deletes an access token           |

---

### POST /v1/access_tokens {#post-access-token}

Creates a new access token

#### Request Parameters

{% api_request_params() %}
| body | `data`                          | object | ✔️ |                                        |
| body | `data.type`                     | string | ✔️ | Type of data. Must be "access_tokens". |
| body | `data.attributes`               | object | ✔️ |                                        |
| body | `data.attributes.email_address` | string | ✔️ | Email address to send access token to. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/access_tokens
Host: {{API_HOST}}
Content-Type: application/json
{% end %}

**Body:**

{% api_response() %}
{
    "data": {
        "type": "access_tokens",
        "attributes": {
            "email_address": "your@email.com"
        }
    }
}
{% end %}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Header:**

{% api_headers() %}
HTTP/1.1 201 Created
Content-Type: application/json
Location: {{API_DOMAIN}}/v1/access_tokens
Server: pkmnapi/0.1.0
{% end %}

**Body:**

{% api_response() %}
{}
{% end %}

---

### POST /v1/access_tokens/delete {#post-access-token-delete}

Requests an access token deletion

#### Request Parameters

{% api_request_params() %}
| body | `data`                          | object | ✔️ |                                                            |
| body | `data.type`                     | string | ✔️ | Type of data. Must be "access_tokens".                     |
| body | `data.attributes`               | object | ✔️ |                                                            |
| body | `data.attributes.email_address` | string | ✔️ | Email address to send access token delete confirmation to. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/access_tokens/delete
Host: {{API_HOST}}
Content-Type: application/json
{% end %}

**Body:**

{% api_response() %}
{
    "data": {
        "type": "access_tokens",
        "attributes": {
            "email_address": "your@email.com"
        }
    }
}
{% end %}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Header:**

{% api_headers() %}
HTTP/1.1 201 Created
Content-Type: application/json
Location: {{API_DOMAIN}}/v1/access_tokens
Server: pkmnapi/0.1.0
{% end %}

**Body:**

{% api_response() %}
{}
{% end %}

---

### DELETE /v1/access_tokens {#delete-access-token}

Deletes an access token

#### Request Parameters

{% api_request_params() %}
| body | `data`                          | object | ✔️ |                                           |
| body | `data.type`                     | string | ✔️ | Type of data. Must be "access_tokens".    |
| body | `data.attributes`               | object | ✔️ |                                           |
| body | `data.attributes.code`          | string | ✔️ | Code sent to the user's email address.    |
| body | `data.attributes.email_address` | string | ✔️ | Email address to delete access token for. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
DELETE /v1/access_tokens
Host: {{API_HOST}}
Content-Type: application/json
{% end %}

**Body:**

{% api_response() %}
{
    "data": {
        "type": "access_tokens",
        "attributes": {
            "code": "1337",
            "email_address": "your@email.com"
        }
    }
}
{% end %}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Header:**

{% api_headers() %}
HTTP/1.1 204 No Content
Content-Type: application/json
Server: pkmnapi/0.1.0
{% end %}

**Body:**

{% api_response() %}
{}
{% end %}
