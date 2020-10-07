+++
title = "Type Effects"
weight = 24
+++

| Endpoint                                                    | Description           |
|-------------------------------------------------------------|-----------------------|
| [GET /v1/types/effects/:type_effect_id](#get-type-effect)   | Gets a type effect    |
| [POST /v1/types/effects/:type_effect_id](#post-type-effect) | Updates a type effect |

---

### GET /v1/types/effects/:type_effect_id {#get-type-effect}

Gets a type effect

#### Request Parameters

{% api_request_params() %}
| url | `:type_effect_id` | string | ✔️ | Type effect ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/types/effects/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                           | object |                                                  |
| `data.id`                                        | string | Type effect ID. (identical to `:type_effect_id`) |
| `data.type`                                      | string | Type of resource. Must be "type_effects".        |
| `data.attributes`                                | object |                                                  |
| `data.attributes.attacking_type`                 | object | Attacking type.                                  |
| `data.attributes.attacking_type.id`              | string | Type ID.                                         |
| `data.attributes.attacking_type.type`            | string | Type of resource. Must be "type_names".          |
| `data.attributes.attacking_type.attributes`      | object |                                                  |
| `data.attributes.attacking_type.attributes.name` | string | Type name.                                       |
| `data.attributes.attacking_type.links`           | object |                                                  |
| `data.attributes.attacking_type.links.self`      | string | Link to type resource.                           |
| `data.attributes.defending_type`                 | object | Defending type.                                  |
| `data.attributes.defending_type.id`              | string | Type ID.                                         |
| `data.attributes.defending_type.type`            | string | Type of resource. Must be "type_names".          |
| `data.attributes.defending_type.attributes`      | object |                                                  |
| `data.attributes.defending_type.attributes.name` | string | Type name.                                       |
| `data.attributes.defending_type.links`           | object |                                                  |
| `data.attributes.defending_type.links.self`      | string | Link to type resource.                           |
| `data.attributes.multiplier`                     | number | Damage multiplier.                               |
| `data.links`                                     | object |                                                  |
| `data.links.self`                                | string | Link to current resource.                        |
| `links`                                          | object |                                                  |
| `links.self`                                     | string | Link to current resource.                        |
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
        "type": "type_effects",
        "attributes": {
            "attacking_type": {
                "id": "21",
                "type": "type_names",
                "attributes": {
                    "name": "WATER"
                },
                "links": {
                    "self": "{{API_DOMAIN}}/v1/types/names/21"
                }
            },
            "defending_type": {
                "id": "20",
                "type": "type_names",
                "attributes": {
                    "name": "FIRE"
                },
                "links": {
                    "self": "{{API_DOMAIN}}/v1/types/names/20"
                }
            },
            "multiplier": 2
        },
        "links": {
        "self": "{{API_DOMAIN}}/v1/types/effects/0"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/types/effects/0"
    }
}
{% end %}

---

### POST /v1/types/effects/:type_effect_id {#post-type-effect}

Updates a type effect

#### Request Parameters

{% api_request_params() %}
| url    | `:type_effect_id`                   | string | ✔️ | Type effect ID.                       |
| header | `X-Patch-Description`               | string |   | Description of change.                |
| body   | `data`                              | object | ✔️ |                                       |
| body   | `data.type`                         | string | ✔️ | Type of data. Must be "type_effects". |
| body   | `data.attributes`                   | object | ✔️ |                                       |
| body   | `data.attributes.attacking_type`    | object | ✔️ | Attacking type.                       |
| body   | `data.attributes.attacking_type.id` | string | ✔️ | Type ID.                              |
| body   | `data.attributes.defending_type`    | object | ✔️ | Defending type.                       |
| body   | `data.attributes.defending_type.id` | string | ✔️ | Type ID.                              |
| body   | `data.attributes.multiplier`        | number | ✔️ | Damage multiplier.                    |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/types/effects/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update WATER vs. FIRE type effect
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "type_effects",
        "attributes": {
            "attacking_type": {
                "id": "21"
            },
            "defending_type": {
                "id": "20"
            },
            "multiplier": 0.0
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
