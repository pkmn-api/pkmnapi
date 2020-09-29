+++
title = "TMs"
weight = 15
+++

| Endpoint                        | Description  |
|---------------------------------|--------------|
| [GET /v1/tms/:tm_id](#get-tm)   | Gets a TM    |
| [POST /v1/tms/:tm_id](#post-tm) | Updates a TM |

---

### GET /v1/tms/:tm_id {#get-tm}

Gets a TM

#### Request Parameters

{% api_request_params() %}
| url | `:tm_id` | string | ✔️ | TM ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/tms/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                 | object |                                         |
| `data.id`                              | string | TM ID. (identical to `:tm_id`)          |
| `data.type`                            | string | Type of resource. Must be "tms".        |
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
        "type": "tms",
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
            "self": "{{API_DOMAIN}}/v1/tms/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/tms/1"
    }
}
{% end %}

---

### POST /v1/tms/:tm_id {#post-tm}

Updates a TM

#### Request Parameters

{% api_request_params() %}
| url    | `:tm_id`                  | string | ✔️ | TM ID.                       |
| header | `X-Patch-Description`     | string |   | Description of change.       |
| body   | `data`                    | object | ✔️ |                              |
| body   | `data.type`               | string | ✔️ | Type of data. Must be "tms". |
| body   | `data.attributes`         | object | ✔️ |                              |
| body   | `data.attributes.move`    | object | ✔️ |                              |
| body   | `data.attributes.move.id` | string | ✔️ | Move ID.                     |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/tms/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update MEGA PUNCH to TELEPORT
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "tms",
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