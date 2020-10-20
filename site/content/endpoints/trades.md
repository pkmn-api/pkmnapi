+++
title = "Trades"
weight = 29
+++

| Endpoint                                 | Description           |
|------------------------------------------|-----------------------|
| [GET /v1/trades](#get-trade-all)         | Gets a list of trades |
| [GET /v1/trades/:trade_id](#get-trade)   | Gets a trade          |
| [POST /v1/trades/:trade_id](#post-trade) | Updates a trade       |

---

### GET /v1/trades {#get-trade-all}

Gets a list of trades

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/trades
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                   | array  |                                                    |
| `data[]`                                 | object |                                                    |
| `data[].id`                              | string | Trade ID. (identical to `:trade_id`)               |
| `data[].type`                            | string | Type of resource. Must be "trades".                |
| `data[].attributes`                      | object |                                                    |
| `data[].attributes.give`                 | object | Pokémon to give.                                   |
| `data[].attributes.give.id`              | string | Pokédex ID.                                        |
| `data[].attributes.give.type`            | string | Type of Pokémon resource. Must be "pokemon_names". |
| `data[].attributes.give.attributes`      | object |                                                    |
| `data[].attributes.give.attributes.name` | string | Pokémon name.                                      |
| `data[].attributes.give.links`           | object |                                                    |
| `data[].attributes.give.links.self`      | string | Link to Pokémon resource.                          |
| `data[].attributes.get`                  | object | Pokémon to get.                                    |
| `data[].attributes.get.id`               | string | Pokédex ID.                                        |
| `data[].attributes.get.type`             | string | Type of Pokémon resource. Must be "pokemon_names". |
| `data[].attributes.get.attributes`       | object |                                                    |
| `data[].attributes.get.attributes.name`  | string | Pokémon name.                                      |
| `data[].attributes.get.links`            | object |                                                    |
| `data[].attributes.get.links.self`       | string | Link to Pokémon resource.                          |
| `data[].attributes.nickname`             | string | Pokémon nickname.                                  |
| `data[].links`                           | object |                                                    |
| `data[].links.self`                      | string | Link to current resource.                          |
| `links`                                  | object |                                                    |
| `links.self`                             | string | Link to list resource.                             |
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
            "type": "trades",
            "attributes": {
                "give": {
                    "id": "33",
                    "type": "pokemon_names",
                    "attributes": {
                        "name": "NIDORINO"
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/pokemon/names/33"
                    }
                },
                "get": {
                    "id": "30",
                    "type": "pokemon_names",
                    "attributes": {
                        "name": "NIDORINA"
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/pokemon/names/30"
                    }
                },
                "nickname": "TERRY"
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/trades/0"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/trades"
    }
}
{% end %}

---

### GET /v1/trades/:trade_id {#get-trade}

Gets a trade

#### Request Parameters

{% api_request_params() %}
| url | `:trade_id` | string | ✔️ | Trade ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/trades/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                 | object |                                                    |
| `data.id`                              | string | Trade ID. (identical to `:trade_id`)               |
| `data.type`                            | string | Type of resource. Must be "trades".                |
| `data.attributes`                      | object |                                                    |
| `data.attributes.give`                 | object | Pokémon to give.                                   |
| `data.attributes.give.id`              | string | Pokédex ID.                                        |
| `data.attributes.give.type`            | string | Type of Pokémon resource. Must be "pokemon_names". |
| `data.attributes.give.attributes`      | object |                                                    |
| `data.attributes.give.attributes.name` | string | Pokémon name.                                      |
| `data.attributes.give.links`           | object |                                                    |
| `data.attributes.give.links.self`      | string | Link to Pokémon resource.                          |
| `data.attributes.get`                  | object | Pokémon to give.                                   |
| `data.attributes.get.id`               | string | Pokédex ID.                                        |
| `data.attributes.get.type`             | string | Type of Pokémon resource. Must be "pokemon_names". |
| `data.attributes.get.attributes`       | object |                                                    |
| `data.attributes.get.attributes.name`  | string | Pokémon name.                                      |
| `data.attributes.get.links`            | object |                                                    |
| `data.attributes.get.links.self`       | string | Link to Pokémon resource.                          |
| `data.attributes.nickname`             | string | Pokémon nickname.                                  |
| `data.links`                           | object |                                                    |
| `data.links.self`                      | string | Link to current resource.                          |
| `links`                                | object |                                                    |
| `links.self`                           | string | Link to current resource.                          |
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
        "type": "trades",
        "attributes": {
            "give": {
                "id": "33",
                "type": "pokemon_names",
                "attributes": {
                    "name": "NIDORINO"
                },
                "links": {
                    "self": "{{API_DOMAIN}}/v1/pokemon/names/33"
                }
            },
            "get": {
                "id": "30",
                "type": "pokemon_names",
                "attributes": {
                    "name": "NIDORINA"
                },
                "links": {
                    "self": "{{API_DOMAIN}}/v1/pokemon/names/30"
                }
            },
            "nickname": "TERRY"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/trades/0"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/trades/0"
    }
}
{% end %}

---

### POST /v1/trades/:trade_id {#post-trade}

Updates a trade

#### Request Parameters

{% api_request_params() %}
| url    | `:trade_id`                | string | ✔️ | Trade ID.                       |
| header | `X-Patch-Description`      | string |   | Description of change.          |
| body   | `data`                     | object | ✔️ |                                 |
| body   | `data.type`                | string | ✔️ | Type of data. Must be "trades". |
| body   | `data.attributes`          | object | ✔️ |                                 |
| body   | `data.attributes.give`     | object | ✔️ | Pokémon to give.                |
| body   | `data.attributes.give.id`  | string | ✔️ | Pokédex ID.                     |
| body   | `data.attributes.get`      | object | ✔️ | Pokémon to get.                 |
| body   | `data.attributes.get.id`   | string | ✔️ | Pokédex ID.                     |
| body   | `data.attributes.nickname` | string | ✔️ | Pokémon nickname.               |

{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/trades/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR to LEAFY-BOI
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "trades",
        "attributes": {
            "give": {
                "id": "3"
            },
            "get": {
                "id": "6"
            },
            "nickname": "CHARCHAR"
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
