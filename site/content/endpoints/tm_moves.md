+++
title = "TM Moves"
weight = 25
+++

| Endpoint                                   | Description             |
|--------------------------------------------|-------------------------|
| [GET /v1/tms/moves](#get-tm-move-all)      | Gets a list of TM moves |
| [GET /v1/tms/moves/:tm_id](#get-tm-move)   | Gets a TM's move        |
| [POST /v1/tms/moves/:tm_id](#post-tm-move) | Updates a TM's move     |

---

### GET /v1/tms/moves {#get-tm-move-all}

Gets a list of TM moves

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/tms/moves
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                   | array  |                                         |
| `data[]`                                 | object |                                         |
| `data[].id`                              | string | TM ID. (identical to `:tm_id`)          |
| `data[].type`                            | string | Type of resource. Must be "tm_moves".   |
| `data[].attributes`                      | object |                                         |
| `data[].attributes.move`                 | object |                                         |
| `data[].attributes.move.id`              | string | Move ID.                                |
| `data[].attributes.move.type`            | string | Type of resource. Must be "move_names". |
| `data[].attributes.move.attributes`      | object |                                         |
| `data[].attributes.move.attributes.name` | string | Move name.                              |
| `data[].attributes.move.links`           | object |                                         |
| `data[].attributes.move.links.self`      | string | Link to move resource.                  |
| `data[].links`                           | object |                                         |
| `data[].links.self`                      | string | Link to current resource.               |
| `links`                                  | object |                                         |
| `links.self`                             | string | Link to list resource.                  |
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
    "data": [
        {
            "id": "1",
            "type": "tm_moves",
            "attributes": {
                "move": {
                    "id": "5",
                    "type": "move_names",
                    "attributes": {
                        "name": "MEGA PUNCH"
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/moves/names/5"
                    }
                }
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/tms/moves/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/tms/moves"
    }
}
{% end %}

---

### GET /v1/tms/moves/:tm_id {#get-tm-move}

Gets a TM's move

#### Request Parameters

{% api_request_params() %}
| url | `:tm_id` | string | ✔️ | TM ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/tms/moves/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                 | object |                                         |
| `data.id`                              | string | TM ID. (identical to `:tm_id`)          |
| `data.type`                            | string | Type of resource. Must be "tm_moves".   |
| `data.attributes`                      | object |                                         |
| `data.attributes.move`                 | object |                                         |
| `data.attributes.move.id`              | string | Move ID.                                |
| `data.attributes.move.type`            | string | Type of resource. Must be "move_names". |
| `data.attributes.move.attributes`      | object |                                         |
| `data.attributes.move.attributes.name` | string | Move name.                              |
| `data.attributes.move.links`           | object |                                         |
| `data.attributes.move.links.self`      | string | Link to move resource.                  |
| `data.links`                           | object |                                         |
| `data.links.self`                      | string | Link to current resource.               |
| `links`                                | object |                                         |
| `links.self`                           | string | Link to current resource.               |
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
        "id": "1",
        "type": "tm_moves",
        "attributes": {
            "move": {
                "id": "5",
                "type": "move_names",
                "attributes": {
                    "name": "MEGA PUNCH"
                },
                "links": {
                    "self": "{{API_DOMAIN}}/v1/moves/names/5"
                }
            }
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/tms/moves/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/tms/moves/1"
    }
}
{% end %}

---

### POST /v1/tms/moves/:tm_id {#post-tm-move}

Updates a TM's move

#### Request Parameters

{% api_request_params() %}
| url    | `:tm_id`                  | string | ✔️ | TM ID.                            |
| header | `X-Patch-Description`     | string |   | Description of change.            |
| body   | `data`                    | object | ✔️ |                                   |
| body   | `data.type`               | string | ✔️ | Type of data. Must be "tm_moves". |
| body   | `data.attributes`         | object | ✔️ |                                   |
| body   | `data.attributes.move`    | object | ✔️ |                                   |
| body   | `data.attributes.move.id` | string | ✔️ | Move ID.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/tms/moves/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update MEGA PUNCH to TELEPORT
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "tm_moves",
        "attributes": {
            "move": {
                "id": "100"
            }
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
