+++
title = "Trainer Names"
weight = 26
+++

| Endpoint                                                  | Description              |
|-----------------------------------------------------------|--------------------------|
| [GET /v1/trainers/names/:trainer_id](#get-trainer-name)   | Gets a trainer's name    |
| [POST /v1/trainers/names/:trainer_id](#post-trainer-name) | Updates a trainer's name |

---

### GET /v1/trainers/names/:trainer_id {#get-trainer-name}

Gets a trainer's name

#### Request Parameters

{% api_request_params() %}
| url | `:trainer_id` | string | ✔️ | Trainer ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/trainers/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                            |
| `data.id`              | string | Trainer ID. (identical to `:trainer_id`)   |
| `data.type`            | string | Type of resource. Must be "trainer_names". |
| `data.attributes`      | object |                                            |
| `data.attributes.name` | string | Trainer name.                              |
| `data.links`           | object |                                            |
| `data.links.self`      | string | Link to current resource.                  |
| `links`                | object |                                            |
| `links.self`           | string | Link to current resource.                  |
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
        "type": "trainer_names",
        "attributes": {
            "name": "YOUNGSTER"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/trainers/names/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/trainers/names/1"
    }
}
{% end %}

---

### POST /v1/trainers/names/:trainer_id {#post-trainer-name}

Updates a trainer's name

#### Request Parameters

{% api_request_params() %}
| url    | `:trainer_id`          | string | ✔️ | Trainer ID.                            |
| header | `X-Patch-Description`  | string |   | Description of change.                 |
| body   | `data`                 | object | ✔️ |                                        |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "trainer_names". |
| body   | `data.attributes`      | object | ✔️ |                                        |
| body   | `data.attributes.name` | string | ✔️ | Trainer name.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/trainers/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update YOUNGSTER to OLD-TIMER
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "trainer_names",
        "attributes": {
            "name": "OLD-TIMER"
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
