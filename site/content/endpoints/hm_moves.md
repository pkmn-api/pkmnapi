+++
title = "HM Moves"
weight = 1
+++

| Endpoint                                   | Description             |
|--------------------------------------------|-------------------------|
| [GET /v1/hms/moves](#get-hm-move-all)      | Gets a list of HM moves |
| [GET /v1/hms/moves/:hm_id](#get-hm-move)   | Gets a HM move          |
| [POST /v1/hms/moves/:hm_id](#post-hm-move) | Updates a HM move       |

---

### GET /v1/hms/moves {#get-hm-move-all}

Gets a list of HM moves

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/hms/moves/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                   | array  |                                         |
| `data[]`                                 | object |                                         |
| `data[].id`                              | string | HM ID. (identical to `:hm_id`)          |
| `data[].type`                            | string | Type of resource. Must be "hm_moves".   |
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
            "type": "hm_moves",
            "attributes": {
                "move": {
                    "id": "15",
                    "type": "move_names",
                    "attributes": {
                        "name": "CUT"
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/moves/names/15"
                    }
                }
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/hms/moves/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/hms/moves"
    }
}
{% end %}

---

### GET /v1/hms/moves/:hm_id {#get-hm-move}

Gets a HM move

#### Request Parameters

{% api_request_params() %}
| url | `:hm_id` | string | ✔️ | HM ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/hms/moves/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                 | object |                                         |
| `data.id`                              | string | HM ID. (identical to `:hm_id`)          |
| `data.type`                            | string | Type of resource. Must be "hm_moves".   |
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
        "type": "hm_moves",
        "attributes": {
            "move": {
                "id": "15",
                "type": "move_names",
                "attributes": {
                    "name": "CUT"
                },
                "links": {
                    "self": "{{API_DOMAIN}}/v1/moves/names/15"
                }
            }
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/hms/moves/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/hms/moves/1"
    }
}
{% end %}

---

### POST /v1/hms/moves/:hm_id {#post-hm-move}

Updates a HM move

#### Request Parameters

{% api_request_params() %}
| url    | `:hm_id`                  | string | ✔️ | HM ID.                            |
| header | `X-Patch-Description`     | string |   | Description of change.            |
| body   | `data`                    | object | ✔️ |                                   |
| body   | `data.type`               | string | ✔️ | Type of data. Must be "hm_moves". |
| body   | `data.attributes`         | object | ✔️ |                                   |
| body   | `data.attributes.move`    | object | ✔️ |                                   |
| body   | `data.attributes.move.id` | string | ✔️ | Move ID.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/hms/moves/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update CUT to TELEPORT
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "hm_moves",
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
