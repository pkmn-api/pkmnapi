+++
title = "HM Names"
weight = 1
+++

| Endpoint                                   | Description             |
|--------------------------------------------|-------------------------|
| [GET /v1/hms/names](#get-hm-name-all)      | Gets a list of HM names |
| [GET /v1/hms/names/:hm_id](#get-hm-name)   | Gets a HM name          |
| [POST /v1/hms/names/:hm_id](#post-hm-name) | Updates a HM name       |

---

### GET /v1/hms/names {#get-hm-name-all}

Gets a list of HM names

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/hms/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                   | array  |                                         |
| `data[]`                 | object |                                         |
| `data[].id`              | string | HM ID. (identical to `:hm_id`)          |
| `data[].type`            | string | Type of resource. Must be "hm_names".   |
| `data[].attributes`      | object |                                         |
| `data[].attributes.name` | string | HM name.                                |
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
            "type": "hm_names",
            "attributes": {
                "name": "HM01"
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/hms/names/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/hms/names"
    }
}
{% end %}

---

### GET /v1/hms/names/:hm_id {#get-hm-name}

Gets a HM name

#### Request Parameters

{% api_request_params() %}
| url | `:hm_id` | string | ✔️ | HM ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/hms/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                         |
| `data.id`              | string | HM ID. (identical to `:hm_id`)          |
| `data.type`            | string | Type of resource. Must be "hm_names".   |
| `data.attributes`      | object |                                         |
| `data.attributes.name` | string | HM name.                                |
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
        "type": "hm_names",
        "attributes": {
            "name": "HM01"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/hms/names/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/hms/names/1"
    }
}
{% end %}
