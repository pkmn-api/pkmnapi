+++
title = "TM Names"
weight = 1
+++

| Endpoint                                   | Description             |
|--------------------------------------------|-------------------------|
| [GET /v1/tms/names](#get-tm-name-all)      | Gets a list of TM names |
| [GET /v1/tms/names/:tm_id](#get-tm-name)   | Gets a TM name          |
| [POST /v1/tms/names/:tm_id](#post-tm-name) | Updates a TM name       |

---

### GET /v1/tms/names {#get-tm-name-all}

Gets a list of TM names

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/tms/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                   | array  |                                         |
| `data[]`                 | object |                                         |
| `data[].id`              | string | TM ID. (identical to `:tm_id`)          |
| `data[].type`            | string | Type of resource. Must be "tm_names".   |
| `data[].attributes`      | object |                                         |
| `data[].attributes.name` | string | TM name.                                |
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
            "type": "tm_names",
            "attributes": {
                "name": "TM01"
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/tms/names/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/tms/names"
    }
}
{% end %}

---

### GET /v1/tms/names/:tm_id {#get-tm-name}

Gets a TM name

#### Request Parameters

{% api_request_params() %}
| url | `:tm_id` | string | ✔️ | TM ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/tms/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                         |
| `data.id`              | string | TM ID. (identical to `:tm_id`)          |
| `data.type`            | string | Type of resource. Must be "tm_names".   |
| `data.attributes`      | object |                                         |
| `data.attributes.name` | string | TM name.                                |
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
        "type": "tm_names",
        "attributes": {
            "name": "TM01"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/tms/names/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/tms/names/1"
    }
}
{% end %}
