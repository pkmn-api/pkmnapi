+++
title = "Pokémon Stats"
weight = 19
+++

| Endpoint                                                  | Description                  |
|-----------------------------------------------------------|------------------------------|
| [GET /v1/pokemon/stats](#get-pokemon-stats-all)           | Gets a list of Pokémon stats |
| [GET /v1/pokemon/stats/:pokedex_id](#get-pokemon-stats)   | Gets a Pokémon's stats       |
| [POST /v1/pokemon/stats/:pokedex_id](#post-pokemon-stats) | Updates a Pokémon's stats    |

---

### GET /v1/pokemon/stats {#get-pokemon-stats-all}

Gets a list of Pokémon stats

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/stats
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                      | array  |                                            |
| `data[]`                                    | object |                                            |
| `data[].id`                                 | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data[].type`                               | string | Type of resource. Must be "pokemon_stats". |
| `data[].attributes`                         | object |                                            |
| `data[].attributes.base_hp`                 | number | Base HP.                                   |
| `data[].attributes.base_attack`             | number | Base attack.                               |
| `data[].attributes.base_defence`            | number | Base defence.                              |
| `data[].attributes.base_speed`              | number | Base speed.                                |
| `data[].attributes.base_special`            | number | Base special.                              |
| `data[].attributes.types`                   | array  |                                            |
| `data[].attributes.types[].id`              | string | Type ID.                                   |
| `data[].attributes.types[].type`            | string | Type of resource. Must be "type_names".    |
| `data[].attributes.types[].attributes`      | object |                                            |
| `data[].attributes.types[].attributes.name` | string | Type name.                                 |
| `data[].attributes.types[].links`           | object |                                            |
| `data[].attributes.types[].links.self`      | string | Link to type resource.                     |
| `data[].attributes.catch_rate`              | number | Catch rate.                                |
| `data[].attributes.base_exp_yield`          | number | Base experience yield.                     |
| `data[].links`                              | object |                                            |
| `data[].links.self`                         | string | Link to current resource.                  |
| `links`                                     | object |                                            |
| `links.self`                                | string | Link to list resource.                     |
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
            "type": "pokemon_stats",
            "attributes": {
                "base_hp": 45,
                "base_attack": 49,
                "base_defence": 49,
                "base_speed": 45,
                "base_special": 65,
                "types": [
                    {
                        "id": "22",
                        "type": "type_names",
                        "attributes": {
                            "name": "GRASS"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/types/names/22"
                        }
                    },
                    {
                        "id": "3",
                        "type": "type_names",
                        "attributes": {
                            "name": "POISON"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/types/names/3"
                        }
                    }
                ],
                "catch_rate": 45,
                "base_exp_yield": 64
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/pokemon/stats/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/stats"
    }
}
{% end %}

---

### GET /v1/pokemon/stats/:pokedex_id {#get-pokemon-stats}

Gets a Pokémon's stats

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/stats/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                    | object |                                            |
| `data.id`                                 | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data.type`                               | string | Type of resource. Must be "pokemon_stats". |
| `data.attributes`                         | object |                                            |
| `data.attributes.base_hp`                 | number | Base HP.                                   |
| `data.attributes.base_attack`             | number | Base attack.                               |
| `data.attributes.base_defence`            | number | Base defence.                              |
| `data.attributes.base_speed`              | number | Base speed.                                |
| `data.attributes.base_special`            | number | Base special.                              |
| `data.attributes.types`                   | array  |                                            |
| `data.attributes.types[].id`              | string | Type ID.                                   |
| `data.attributes.types[].type`            | string | Type of resource. Must be "type_names".    |
| `data.attributes.types[].attributes`      | object |                                            |
| `data.attributes.types[].attributes.name` | string | Type name.                                 |
| `data.attributes.types[].links`           | object |                                            |
| `data.attributes.types[].links.self`      | string | Link to type resource.                     |
| `data.attributes.catch_rate`              | number | Catch rate.                                |
| `data.attributes.base_exp_yield`          | number | Base experience yield.                     |
| `data.links`                              | object |                                            |
| `data.links.self`                         | string | Link to current resource.                  |
| `links`                                   | object |                                            |
| `links.self`                              | string | Link to current resource.                  |
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
        "type": "pokemon_stats",
        "attributes": {
            "base_hp": 45,
            "base_attack": 49,
            "base_defence": 49,
            "base_speed": 45,
            "base_special": 65,
            "types": [
                {
                    "id": "22",
                    "type": "type_names",
                    "attributes": {
                        "name": "GRASS"
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/types/names/22"
                    }
                },
                {
                    "id": "3",
                    "type": "type_names",
                    "attributes": {
                        "name": "POISON"
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/types/names/3"
                    }
                }
            ],
            "catch_rate": 45,
            "base_exp_yield": 64
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/stats/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/stats/1"
    }
}
{% end %}

---

### POST /v1/pokemon/stats/:pokedex_id {#post-pokemon-stats}

Updates a Pokémon's stats

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`                             | string | ✔️ | Pokédex ID.                            |
| header | `X-Patch-Description`                     | string |   | Description of change.                 |
| body   | `data`                                    | object | ✔️ |                                        |
| body   | `data.type`                               | string | ✔️ | Type of data. Must be "pokemon_stats". |
| body   | `data.attributes`                         | object | ✔️ |                                        |
| body   | `data.attributes.base_hp`                 | number | ✔️ | Base HP.                               |
| body   | `data.attributes.base_attack`             | number | ✔️ | Base attack.                           |
| body   | `data.attributes.base_defence`            | number | ✔️ | Base defence.                          |
| body   | `data.attributes.base_speed`              | number | ✔️ | Base speed.                            |
| body   | `data.attributes.base_special`            | number | ✔️ | Base special.                          |
| body   | `data.attributes.types`                   | array  | ✔️ |                                        |
| body   | `data.attributes.types[].id`              | string | ✔️ | Type ID.                               |
| body   | `data.attributes.catch_rate`              | number | ✔️ | Catch rate.                            |
| body   | `data.attributes.base_exp_yield`          | number | ✔️ | Base experience yield.                 |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/stats/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR's stats
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_stats",
        "attributes": {
            "base_hp": 42,
            "base_attack": 42,
            "base_defence": 42,
            "base_speed": 42,
            "base_special": 42,
            "types": [
                {
                    "id": "20"
                },
                {
                    "id": "20"
                }
            ],
            "catch_rate": 42,
            "base_exp_yield": 42
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
