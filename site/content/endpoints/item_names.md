+++
title = "Item Names"
weight = 5
+++

| Endpoint                                         | Description               |
|--------------------------------------------------|---------------------------|
| [GET /v1/items/names](#get-item-name-all)        | Gets a list of item names |
| [GET /v1/items/names/:item_id](#get-item-name)   | Gets an item's name       |
| [POST /v1/items/names/:item_id](#post-item-name) | Updates an item's name    |

---

### GET /v1/items/names {#get-item-name-all}

Gets a list of item name

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/items/names
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                   | array  |                                         |
| `data[]`                 | object |                                         |
| `data[].id`              | string | Item ID. (identical to `:item_id`)      |
| `data[].type`            | string | Type of resource. Must be "item_names". |
| `data[].attributes`      | object |                                         |
| `data[].attributes.name` | string | Item name.                              |
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
            "type": "item_names",
            "attributes": {
                "name": "MASTER BALL"
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/items/names/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/items/names"
    }
}
{% end %}

---

### GET /v1/items/names/:item_id {#get-item-name}

Gets an item's name

#### Request Parameters

{% api_request_params() %}
| url | `:item_id` | string | ✔️ | Item ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/items/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                         |
| `data.id`              | string | Item ID. (identical to `:item_id`)      |
| `data.type`            | string | Type of resource. Must be "item_names". |
| `data.attributes`      | object |                                         |
| `data.attributes.name` | string | Item name.                              |
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
        "type": "item_names",
        "attributes": {
            "name": "MASTER BALL"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/items/names/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/items/names/1"
    }
}
{% end %}

---

### POST /v1/items/names/:item_id {#post-item-name}

Updates an item's name

#### Request Parameters

{% api_request_params() %}
| url    | `:item_id`             | string | ✔️ | Item ID.                            |
| header | `X-Patch-Description`  | string |   | Description of change.              |
| body   | `data`                 | object | ✔️ |                                     |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "item_names". |
| body   | `data.attributes`      | object | ✔️ |                                     |
| body   | `data.attributes.name` | string | ✔️ | Item name.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/items/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update MASTER BALL to CHEATERBALL
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "item_names",
        "attributes": {
            "name": "CHEATERBALL"
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
