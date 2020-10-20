+++
title = "Mart Items"
weight = 1
+++

| Endpoint                                         | Description               |
|--------------------------------------------------|---------------------------|
| [GET /v1/mart/items](#get-mart-items-all)        | Gets a list of mart items |
| [GET /v1/mart/items/:mart_id](#get-mart-items)   | Gets a mart's items       |
| [POST /v1/mart/items/:mart_id](#post-mart-items) | Updates a mart's items    |

---

### GET /v1/mart/items {#get-mart-items-all}

Gets a list of mart items

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/mart/items
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                                           | array  |                                                        |
| `data[]`                                                         | object |                                                        |
| `data[].id`                                                      | string | Item ID. (identical to `:mart_id`)                     |
| `data[].type`                                                    | string | Type of resource. Must be "mart_items".                |
| `data[].attributes`                                              | object |                                                        |
| `data[].attributes.mart_items`                                   | array  |                                                        |
| `data[].attributes.mart_items[]`                                 | object |                                                        |
| `data[].attributes.mart_items[].id`                              | number | ID of mart item.                                       |
| `data[].attributes.mart_items[].type`                            | string | Type of mart item. Must be "item_names" or "tm_moves". |
| `data[].attributes.mart_items[].attributes`                      | object |                                                        |
| `data[].attributes.mart_items[].attributes.name`                 | string | Item name. (If type is "item_names")                   |
| `data[].attributes.mart_items[].attributes.move`                 | string | Move. (If type is "tm_moves")                          |
| `data[].attributes.mart_items[].attributes.move.id`              | string | ID of move.                                            |
| `data[].attributes.mart_items[].attributes.move.type`            | string | Type of move. Must be "move_names".                    |
| `data[].attributes.mart_items[].attributes.move.attributes`      | object |                                                        |
| `data[].attributes.mart_items[].attributes.move.attributes.name` | string | Move name.                                             |
| `data[].attributes.mart_items[].attributes.move.links`           | object |                                                        |
| `data[].attributes.mart_items[].attributes.move.links.self`      | string | Link to move resource.                                 |
| `data[].attributes.mart_items[].links`                           | object |                                                        |
| `data[].attributes.mart_items[].self`                            | string | Link of mart item resource.                            |
| `data[].links`                                                   | object |                                                        |
| `data[].links.self`                                              | string | Link to current resource.                              |
| `links`                                                          | object |                                                        |
| `links.self`                                                     | string | Link to list resource.                                 |
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
            "id": "0",
            "type": "mart_items",
            "attributes": {
                "mart_items": [
                    {
                        "id": "4",
                        "type": "item_names",
                        "attributes": {
                            "name": "POKé BALL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/4"
                        }
                    },
                    {
                        "id": "11",
                        "type": "item_names",
                        "attributes": {
                            "name": "ANTIDOTE"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/11"
                        }
                    },
                    {
                        "id": "15",
                        "type": "item_names",
                        "attributes": {
                            "name": "PARLYZ HEAL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/15"
                        }
                    },
                    {
                        "id": "12",
                        "type": "item_names",
                        "attributes": {
                            "name": "BURN HEAL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/12"
                        }
                    }
                ]
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/mart/items/0"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/mart/items"
    }
}
{% end %}

---

### GET /v1/mart/items/:mart_id {#get-mart-items}

Gets a mart's items

#### Request Parameters

{% api_request_params() %}
| url | `:mart_id` | string | ✔️ | Mart ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/mart/items/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                                         | object |                                                        |
| `data.id`                                                      | string | Item ID. (identical to `:mart_id`)                     |
| `data.type`                                                    | string | Type of resource. Must be "mart_items".                |
| `data.attributes`                                              | object |                                                        |
| `data.attributes.mart_items`                                   | array  |                                                        |
| `data.attributes.mart_items[]`                                 | object |                                                        |
| `data.attributes.mart_items[].id`                              | number | ID of mart item.                                       |
| `data.attributes.mart_items[].type`                            | string | Type of mart item. Must be "item_names" or "tm_moves". |
| `data.attributes.mart_items[].attributes`                      | object |                                                        |
| `data.attributes.mart_items[].attributes.name`                 | string | Item name. (If type is "item_names")                   |
| `data.attributes.mart_items[].attributes.move`                 | string | Move. (If type is "tm_moves")                          |
| `data.attributes.mart_items[].attributes.move.id`              | string | ID of move.                                            |
| `data.attributes.mart_items[].attributes.move.type`            | string | Type of move. Must be "move_names".                    |
| `data.attributes.mart_items[].attributes.move.attributes`      | object |                                                        |
| `data.attributes.mart_items[].attributes.move.attributes.name` | string | Move name.                                             |
| `data.attributes.mart_items[].attributes.move.links`           | object |                                                        |
| `data.attributes.mart_items[].attributes.move.links.self`      | string | Link to move resource.                                 |
| `data.links`                                                   | object |                                                        |
| `data.links.self`                                              | string | Link to current resource.                              |
| `links`                                                        | object |                                                        |
| `links.self`                                                   | string | Link to current resource.                              |
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
        "type": "mart_items",
        "attributes": {
            "mart_items": [
                {
                    "id": "4",
                    "type": "item_names",
                    "attributes": {
                        "name": "POKé BALL"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/items/names/4"
                    }
                },
                {
                    "id": "11",
                    "type": "item_names",
                    "attributes": {
                        "name": "ANTIDOTE"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/items/names/11"
                    }
                },
                {
                    "id": "15",
                    "type": "item_names",
                    "attributes": {
                        "name": "PARLYZ HEAL"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/items/names/15"
                    }
                },
                {
                    "id": "12",
                    "type": "item_names",
                    "attributes": {
                        "name": "BURN HEAL"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/items/names/12"
                    }
                }
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/mart/items/0"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/mart/items/0"
    }
}
{% end %}

---

### POST /v1/mart/items/:mart_id {#post-mart-items}

Updates a mart's items

#### Request Parameters

{% api_request_params() %}
| url    | `:mart_id`             | string | ✔️ | Mart ID.                            |
| header | `X-Patch-Description`  | string |   | Description of change.              |
| body   | `data`                 | object | ✔️ |                                     |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "mart_items". |
| body   | `data.attributes`      | object | ✔️ |                                     |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/mart/items/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update Viridian's Pokémart items
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "mart_items",
        "attributes": {
            "mart_items": [
                {
                    "id": "1",
                    "type": "item_names"
                },
                {
                    "id": "2",
                    "type": "item_names"
                },
                {
                    "id": "3",
                    "type": "item_names"
                },
                {
                    "id": "4",
                    "type": "item_names"
                }
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
