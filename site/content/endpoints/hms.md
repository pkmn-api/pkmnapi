+++
title = "HMs"
weight = 2
+++

| Endpoint                        | Description  |
|---------------------------------|--------------|
| [GET /v1/hms/:hm_id](#get-hm)   | Gets a HM    |
| [POST /v1/hms/:hm_id](#post-hm) | Updates a HM |

---

### GET /v1/hms/:hm_id {#get-hm}

Gets a HM

#### Request Parameters

{% api_request_params() %}
| url | `:hm_id` | string | ✔️ | HM ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/hms/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                 | object |                                         |
| `data.id`                              | string | HM ID. (identical to `:hm_id`)          |
| `data.type`                            | string | Type of resource. Must be "hms".        |
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
        "type": "hms",
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
            "self": "{{API_DOMAIN}}/v1/hms/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/hms/1"
    }
}
{% end %}

---

### POST /v1/hms/:hm_id {#post-hm}

Updates a HM

#### Request Parameters

{% api_request_params() %}
| url    | `:hm_id`                  | string | ✔️ | HM ID.                       |
| header | `X-Patch-Description`     | string |   | Description of change.       |
| body   | `data`                    | object | ✔️ |                              |
| body   | `data.type`               | string | ✔️ | Type of data. Must be "hms". |
| body   | `data.attributes`         | object | ✔️ |                              |
| body   | `data.attributes.move`    | object | ✔️ |                              |
| body   | `data.attributes.move.id` | string | ✔️ | Move ID.                     |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/hms/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update CUT to TELEPORT
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "hms",
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
