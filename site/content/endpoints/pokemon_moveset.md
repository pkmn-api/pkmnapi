+++
title = "Pokémon Movesets"
weight = 17
+++

| Endpoint                                                         | Description                      |
|------------------------------------------------------------------|----------------------------------|
| [GET /v1/pokemon/movesets](#get-pokemon-moveset-all)           | Gets a list of Pokémon movesets |
| [GET /v1/pokemon/movesets/:pokedex_id](#get-pokemon-moveset)   | Gets a Pokémon's moveset        |
| [POST /v1/pokemon/movesets/:pokedex_id](#post-pokemon-moveset) | Updates a Pokémon's moveset     |

---

### GET /v1/pokemon/movesets {#get-pokemon-moveset-all}

Gets a list of Pokémon movesets

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/movesets
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                           | array  |                                               |
| `data[]`                                         | object |                                               |
| `data[].id`                                      | string | Pokédex ID. (identical to `:pokedex_id`)      |
| `data[].type`                                    | string | Type of resource. Must be "pokemon_movesets". |
| `data[].attributes`                              | object |                                               |
| `data[].attributes.moveset`                      | array  | Pokémon moveset.                              |
| `data[].attributes.moveset[]`                    | object |                                               |
| `data[].attributes.moveset.move`                 | object |                                               |
| `data[].attributes.moveset.move.id`              | string | Move ID.                                      |
| `data[].attributes.moveset.move.type`            | string | Type of move resource. Must be "move_names".  |
| `data[].attributes.moveset.move.attributes`      | object |                                               |
| `data[].attributes.moveset.move.attributes.name` | string | Move name.                                    |
| `data[].attributes.moveset.move.links`           | object |                                               |
| `data[].attributes.moveset.move.links.self`      | string | Link to move resource.                        |
| `data[].links`                                   | object |                                               |
| `data[].links.self`                              | string | Link to current resource.                     |
| `links`                                          | object |                                               |
| `links.self`                                     | string | Link to list resource.                        |
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
            "type": "pokemon_movesets",
            "attributes": {
                "moveset": [
                    {
                        "move": {
                            "id": "33",
                            "type": "move_names",
                            "attributes": {
                                "name": "TACKLE"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/33"
                            }
                        }
                    },
                    {
                        "move": {
                            "id": "45",
                            "type": "move_names",
                            "attributes": {
                                "name": "GROWL"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/45"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/pokemon/movesets/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/movesets"
    }
}
{% end %}

---

### GET /v1/pokemon/movesets/:pokedex_id {#get-pokemon-moveset}

Gets a Pokémon's moveset

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/movesets/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                         | object |                                               |
| `data.id`                                      | string | Pokédex ID. (identical to `:pokedex_id`)      |
| `data.type`                                    | string | Type of resource. Must be "pokemon_movesets". |
| `data.attributes`                              | object |                                               |
| `data.attributes.moveset`                      | array  | Pokémon moveset.                              |
| `data.attributes.moveset[]`                    | object |                                               |
| `data.attributes.moveset.move`                 | object |                                               |
| `data.attributes.moveset.move.id`              | string | Move ID.                                      |
| `data.attributes.moveset.move.type`            | string | Type of move resource. Must be "move_names".  |
| `data.attributes.moveset.move.attributes`      | object |                                               |
| `data.attributes.moveset.move.attributes.name` | string | Move name.                                    |
| `data.attributes.moveset.move.links`           | object |                                               |
| `data.attributes.moveset.move.links.self`      | string | Link to move resource.                        |
| `data.links`                                   | object |                                               |
| `data.links.self`                              | string | Link to current resource.                     |
| `links`                                        | object |                                               |
| `links.self`                                   | string | Link to current resource.                     |
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
        "type": "pokemon_movesets",
        "attributes": {
            "moveset": [
                {
                    "move": {
                        "id": "33",
                        "type": "move_names",
                        "attributes": {
                            "name": "TACKLE"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/33"
                        }
                    }
                },
                {
                    "move": {
                        "id": "45",
                        "type": "move_names",
                        "attributes": {
                            "name": "GROWL"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/45"
                        }
                    }
                }
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/movesets/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/movesets/1"
    }
}
{% end %}

---

### POST /v1/pokemon/movesets/:pokedex_id {#post-pokemon-moveset}

Updates a Pokémon's movesets

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`                       | string | ✔️ | Pokédex ID.                               |
| header | `X-Patch-Description`               | string |   | Description of change.                    |
| body   | `data`                              | object | ✔️ |                                           |
| body   | `data.type`                         | string | ✔️ | Type of data. Must be "pokemon_movesets". |
| body   | `data.attributes`                   | object | ✔️ |                                           |
| body   | `data.attributes.moveset`           | array  | ✔️ | Pokémon moveset.                          |
| body   | `data.attributes.moveset[]`         | object | ✔️ |                                           |
| body   | `data.attributes.moveset[].move`    | object | ✔️ |                                           |
| body   | `data.attributes.moveset[].move.id` | string | ✔️ | Move ID.                                  |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/movesets/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR's moveset
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_movesets",
        "attributes": {
            "moveset": [
                {
                    "move": {
                        "id": "1"
                    }
                },
                {
                    "move": {
                        "id": "2"
                    }
                },
                {
                    "move": {
                        "id": "3"
                    }
                },
                {
                    "move": {
                        "id": "4"
                    }
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
