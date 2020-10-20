+++
title = "Move Stats"
weight = 1
+++

| Endpoint                                          | Description               |
|---------------------------------------------------|---------------------------|
| [GET /v1/moves/stats](#get-move-stats-all)        | Gets a list of move stats |
| [GET /v1/moves/stats/:move_id](#get-move-stats)   | Gets a move's stats       |
| [POST /v1/moves/stats/:move_id](#post-move-stats) | Updates a move's stats    |

---

### GET /v1/moves/stats {#get-move-stats-all}

Gets a list of move stats

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/moves/stats
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                      | array  |                                         |
| `data[]`                                    | object |                                         |
| `data[].id`                                 | string | Move ID. (identical to `:move_id`)      |
| `data[].type`                               | string | Type of resource. Must be "move_stats". |
| `data[].attributes`                         | object |                                         |
| `data[].attributes.effect`                  | number | Effect ID.                              |
| `data[].attributes.power`                   | number | Power level.                            |
| `data[].attributes.type`                    | array  |                                         |
| `data[].attributes.type.id`                 | string | Type ID.                                |
| `data[].attributes.type.type`               | string | Type of resource. Must be "type_names". |
| `data[].attributes.type.attributes`         | object |                                         |
| `data[].attributes.type.attributes.name`    | string | Type name.                              |
| `data[].attributes.type.links`              | object |                                         |
| `data[].attributes.type.links.self`         | string | Link to type resource.                  |
| `data[].attributes.accuracy`                | number | Move accuracy.                          |
| `data[].attributes.pp`                      | number | Power points.                           |
| `data[].links`                              | object |                                         |
| `data[].links.self`                         | string | Link to current resource.               |
| `links`                                     | object |                                         |
| `links.self`                                | string | Link to list resource.                  |
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
            "type": "move_stats",
            "attributes": {
                "effect": 0,
                "power": 40,
                "type": {
                    "id": "0",
                    "type": "type_names",
                    "attributes": {
                        "name": "NORMAL"
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/types/names/0"
                    }
                },
                "accuracy": 1.0,
                "pp": 35
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/moves/stats/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/moves/stats"
    }
}
{% end %}

---

### GET /v1/moves/stats/:move_id {#get-move-stats}

Gets a move's stats

#### Request Parameters

{% api_request_params() %}
| url | `:move_id` | string | ✔️ | Move ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/moves/stats/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                    | object |                                         |
| `data.id`                                 | string | Move ID. (identical to `:move_id`)      |
| `data.type`                               | string | Type of resource. Must be "move_stats". |
| `data.attributes`                         | object |                                         |
| `data.attributes.effect`                  | number | Effect ID.                              |
| `data.attributes.power`                   | number | Power level.                            |
| `data.attributes.type`                    | array  |                                         |
| `data.attributes.type.id`                 | string | Type ID.                                |
| `data.attributes.type.type`               | string | Type of resource. Must be "type_names". |
| `data.attributes.type.attributes`         | object |                                         |
| `data.attributes.type.attributes.name`    | string | Type name.                              |
| `data.attributes.type.links`              | object |                                         |
| `data.attributes.type.links.self`         | string | Link to type resource.                  |
| `data.attributes.accuracy`                | number | Move accuracy.                          |
| `data.attributes.pp`                      | number | Power points.                           |
| `data.links`                              | object |                                         |
| `data.links.self`                         | string | Link to current resource.               |
| `links`                                   | object |                                         |
| `links.self`                              | string | Link to current resource.               |
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
        "type": "move_stats",
        "attributes": {
            "effect": 0,
            "power": 40,
            "type": {
                "id": "0",
                "type": "type_names",
                "attributes": {
                    "name": "NORMAL"
                },
                "links": {
                    "self": "{{API_DOMAIN}}/v1/types/names/0"
                }
            },
            "accuracy": 1.0,
            "pp": 35
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/moves/stats/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/moves/stats/1"
    }
}
{% end %}

---

### POST /v1/moves/stats/:move_id {#post-move-stats}

Updates a move's stats

#### Request Parameters

{% api_request_params() %}
| url    | `:move_id`                       | string | ✔️ | Move ID.                            |
| header | `X-Patch-Description`            | string |   | Description of change.              |
| body   | `data`                           | object | ✔️ |                                     |
| body   | `data.type`                      | string | ✔️ | Type of data. Must be "move_stats". |
| body   | `data.attributes`                | object | ✔️ |                                     |
| body   | `data.attributes.effect`         | number | ✔️ | Effect ID.                          |
| body   | `data.attributes.power`          | number | ✔️ | Power level.                        |
| body   | `data.attributes.type`           | array  | ✔️ |                                     |
| body   | `data.attributes.type.id`        | string | ✔️ | Type ID.                            |
| body   | `data.attributes.accuracy`       | number | ✔️ | Move accuracy.                      |
| body   | `data.attributes.pp`             | number | ✔️ | Power points.                       |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/moves/stats/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR's stats
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "move_stats",
        "attributes": {
            "effect": 0,
            "power": 20,
            "type": {
                "id": "1"
            },
            "accuracy": 0.1337,
            "pp": 42
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
