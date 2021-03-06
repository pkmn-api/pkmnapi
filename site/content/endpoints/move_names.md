+++
title = "Move Names"
weight = 1
+++

| Endpoint                                         | Description               |
|--------------------------------------------------|---------------------------|
| [GET /v1/moves/names](#get-move-name-all)        | Gets a list of move names |
| [GET /v1/moves/names/:move_id](#get-move-name)   | Gets a move name          |
| [POST /v1/moves/names/:move_id](#post-move-name) | Updates a move name       |

---

### GET /v1/moves/names {#get-move-name-all}

Gets a list of move names

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/moves/names
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                   | array  |                                         |
| `data[]`                 | object |                                         |
| `data[].id`              | string | Move ID. (identical to `:move_id`)      |
| `data[].type`            | string | Type of resource. Must be "move_names". |
| `data[].attributes`      | object |                                         |
| `data[].attributes.name` | string | Move name.                              |
| `data[].links`           | object |                                         |
| `data[].links.self`      | string | Link to current resource.               |
| `links`                  | object |                                         |
| `links.self`             | string | Link to list resource.                  |
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
            "type": "move_names",
            "attributes": {
                "name": "POUND"
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/moves/names/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/moves/names"
    }
}
{% end %}

---

### GET /v1/moves/names/:move_id {#get-move-name}

Gets a move name

#### Request Parameters

{% api_request_params() %}
| url | `:move_id` | string | ✔️ | Move ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/moves/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                         |
| `data.id`              | string | Move ID. (identical to `:move_id`)      |
| `data.type`            | string | Type of resource. Must be "move_names". |
| `data.attributes`      | object |                                         |
| `data.attributes.name` | string | Move name.                              |
| `data.links`           | object |                                         |
| `data.links.self`      | string | Link to current resource.               |
| `links`                | object |                                         |
| `links.self`           | string | Link to current resource.               |
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
        "type": "move_names",
        "attributes": {
            "name": "POUND"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/moves/names/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/moves/names/1"
    }
}
{% end %}

---

### POST /v1/moves/names/:move_id {#post-move-name}

Updates a move name

#### Request Parameters

{% api_request_params() %}
| url    | `:move_id`             | string | ✔️ | Move ID.                            |
| header | `X-Patch-Description`  | string |   | Description of change.              |
| body   | `data`                 | object | ✔️ |                                     |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "move_names". |
| body   | `data.attributes`      | object | ✔️ |                                     |
| body   | `data.attributes.name` | string | ✔️ | Move name.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/moves/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update POUND move name
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "move_names",
        "attributes": {
            "name": "SMACK"
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
