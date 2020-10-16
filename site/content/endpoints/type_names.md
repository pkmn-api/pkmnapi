+++
title = "Type Names"
weight = 32
+++

| Endpoint                                         | Description               |
|--------------------------------------------------|---------------------------|
| [GET /v1/types/names](#get-type-name-all)        | Gets a list of type names |
| [GET /v1/types/names/:type_id](#get-type-name)   | Gets a type name          |
| [POST /v1/types/names/:type_id](#post-type-name) | Updates a type name       |

---

### GET /v1/types/names {#get-type-name-all}

Gets a list of type names

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/types/names
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                   | array  |                                         |
| `data[]`                 | object |                                         |
| `data[].id`              | string | Type ID. (identical to `:type_id`)      |
| `data[].type`            | string | Type of resource. Must be "type_names". |
| `data[].attributes`      | object |                                         |
| `data[].attributes.name` | string | Type name.                              |
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
            "id": "0",
            "type": "type_names",
            "attributes": {
                "name": "NORMAL"
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/types/names/0"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/types/names"
    }
}
{% end %}

---

### GET /v1/types/names/:type_id {#get-type-name}

Gets a type name

#### Request Parameters

{% api_request_params() %}
| url | `:type_id` | string | ✔️ | Type ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/types/names/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                         |
| `data.id`              | string | Type ID. (identical to `:type_id`)      |
| `data.type`            | string | Type of resource. Must be "type_names". |
| `data.attributes`      | object |                                         |
| `data.attributes.name` | string | Type name.                              |
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
        "id": "0",
        "type": "type_names",
        "attributes": {
            "name": "NORMAL"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/types/names/0"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/types/names/0"
    }
}
{% end %}

---

### POST /v1/types/names/:type_id {#post-type-name}

Updates a type

#### Request Parameters

{% api_request_params() %}
| url    | `:type_id`             | string | ✔️ | Type ID.                            |
| header | `X-Patch-Description`  | string |   | Description of change.              |
| body   | `data`                 | object | ✔️ |                                     |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "type_names". |
| body   | `data.attributes`      | object | ✔️ |                                     |
| body   | `data.attributes.name` | string | ✔️ | Type name.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/types/names/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update NORMAL type
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "type_names",
        "attributes": {
            "name": "BORING"
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
