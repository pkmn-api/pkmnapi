+++
title = "Player Names"
weight = 10
+++

| Endpoint                                    | Description                  |
|---------------------------------------------|------------------------------|
| [GET /v1/player_names](#get-player-names)   | Gets default player names    |
| [POST /v1/player_names](#post-player-names) | Updates default player names |

---

### GET /v1/player_names {#get-player-names}

Gets default player names

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/player_names
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                     | object |                                           |
| `data.id`                  | string | ID. Must be "0".                          |
| `data.type`                | string | Type of resource. Must be "player_names". |
| `data.attributes`          | object |                                           |
| `data.attributes.player`   | array  | Player names.                             |
| `data.attributes.player[]` | string | Player name.                              |
| `data.attributes.rival`    | array  | Rival names.                              |
| `data.attributes.rival[]`  | string | Rival name.                               |
| `data.links`               | object |                                           |
| `data.links.self`          | string | Link to current resource.                 |
| `links`                    | object |                                           |
| `links.self`               | string | Link to current resource.                 |
{% end %}

#### Example Response

**Headers:**

{% api_headers() %}
HTTP/1.1 200 OK
Content-Type: application/json
Server: pkmnapi/0.1.0
{% end %}

**Body:**

{% api_response() %}
{
    "data": {
        "id": "0",
        "type": "player_names",
        "attributes": {
            "player": [
                "RED",
                "ASH",
                "JACK"
            ],
            "rival": [
                "BLUE",
                "GARY",
                "JOHN"
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/player_names"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/player_names"
    }
}
{% end %}

---

### POST /v1/player_names {#post-player-names}

Updates default player names

#### Request Parameters

{% api_request_params() %}
| header | `X-Patch-Description`      | string |   | Description of change.                |
| body   | `data`                     | object | ✔️ |                                       |
| body   | `data.type`                | string | ✔️ | Type of data. Must be "player_names". |
| body   | `data.attributes`          | object | ✔️ |                                       |
| body   | `data.attributes.player`   | array  | ✔️ | Player names.                         |
| body   | `data.attributes.player[]` | string | ✔️ | Player name.                          |
| body   | `data.attributes.rival`    | array  | ✔️ | Rival names.                          |
| body   | `data.attributes.rival[]`  | string | ✔️ | Rival name.                           |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/player_names
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update default player names
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "player_names",
        "attributes": {
            "player": [
                "BED",
                "ASK",
                "JILL"
            ],
            "rival": [
                "TRUE",
                "MARY",
                "JANE"
            ]
        }
    }
}
{% end %}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Headers:**

{% api_headers() %}
HTTP/1.1 202 Accepted
Content-Type: application/json
Server: pkmnapi/0.1.0
{% end %}

**Body:**

{% api_response() %}
{}
{% end %}
