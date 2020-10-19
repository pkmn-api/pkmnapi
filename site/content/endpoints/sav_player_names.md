+++
title = "SAV Player Names"
weight = 24
+++

| Endpoint                                            | Description               |
|-----------------------------------------------------|---------------------------|
| [GET /v1/savs/player_names](#get-sav-player-name)   | Gets saved player name    |
| [POST /v1/savs/player_names](#post-sav-player-name) | Updates saved player name |

---

### GET /v1/savs/player_names {#get-sav-player-name}

Gets saved player name

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/savs/player_names
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                               |
| `data.id`              | string | Player ID.                                    |
| `data.type`            | string | Type of resource. Must be "sav_player_names". |
| `data.attributes`      | object |                                               |
| `data.attributes.name` | string | Saved player name.                            |
| `data.links`           | object |                                               |
| `data.links.self`      | string | Link to current resource.                     |
| `links`                | object |                                               |
| `links.self`           | string | Link to current resource.                     |
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
        "id": "1337",
        "type": "sav_player_names",
        "attributes": {
            "name": "RED"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/savs/player_names"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/savs/player_names"
    }
}
{% end %}

---

### POST /v1/savs/player_names {#post-sav-player-name}

Updates saved player name

#### Request Parameters

{% api_request_params() %}
| header | `X-Patch-Description`  | string |   | Description of change.                    |
| body   | `data`                 | object | ✔️ |                                           |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "sav_player_names". |
| body   | `data.attributes`      | object | ✔️ |                                           |
| body   | `data.attributes.name` | string | ✔️ | Saved player name.                        |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/savs/player_names
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update RED to ASH
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "sav_player_names",
        "attributes": {
            "name": "ASH"
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
