+++
title = "Pokémon Learnsets"
weight = 15
+++

| Endpoint                                                         | Description                      |
|------------------------------------------------------------------|----------------------------------|
| [GET /v1/pokemon/learnsets](#get-pokemon-learnset-all)           | Gets a list of Pokémon learnsets |
| [GET /v1/pokemon/learnsets/:pokedex_id](#get-pokemon-learnset)   | Gets a Pokémon's learnset        |
| [POST /v1/pokemon/learnsets/:pokedex_id](#post-pokemon-learnset) | Updates a Pokémon's learnset     |

---

### GET /v1/pokemon/learnsets {#get-pokemon-learnset-all}

Gets a list of Pokémon learnsets

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/learnsets
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                            | array  |                                                 |
| `data[]`                                          | object |                                                 |
| `data[].id`                                       | string | Pokédex ID. (identical to `:pokedex_id`)        |
| `data[].type`                                     | string | Type of resource. Must be "pokemon_learnsets".  |
| `data[].attributes`                               | object |                                                 |
| `data[].attributes.learnset`                      | array  | Pokémon learnset.                               |
| `data[].attributes.learnset[]`                    | object |                                                 |
| `data[].attributes.learnset.level`                | number | Learn level.                                    |
| `data[].attributes.learnset.move`                 | object |                                                 |
| `data[].attributes.learnset.move.id`              | string | Move ID.                                        |
| `data[].attributes.learnset.move.type`            | string | Type of move resource. Must be "move_names".    |
| `data[].attributes.learnset.move.attributes`      | object |                                                 |
| `data[].attributes.learnset.move.attributes.name` | string | Move name.                                      |
| `data[].attributes.learnset.move.links`           | object |                                                 |
| `data[].attributes.learnset.move.links.self`      | string | Link to move resource.                          |
| `data[].links`                                    | object |                                                 |
| `data[].links.self`                               | string | Link to current resource.                       |
| `links`                                           | object |                                                 |
| `links.self`                                      | string | Link to list resource.                          |
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
            "type": "pokemon_learnsets",
            "attributes": {
                "learnset": [
                    {
                        "level": 7,
                        "move": {
                            "id": "73",
                            "type": "move_names",
                            "attributes": {
                                "name": "LEECH SEED"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/73"
                            }
                        }
                    },
                    {
                        "level": 13,
                        "move": {
                            "id": "22",
                            "type": "move_names",
                            "attributes": {
                                "name": "VINE WHIP"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/22"
                            }
                        }
                    },
                    {
                        "level": 20,
                        "move": {
                            "id": "77",
                            "type": "move_names",
                            "attributes": {
                                "name": "POISONPOWDER"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/77"
                            }
                        }
                    },
                    {
                        "level": 27,
                        "move": {
                            "id": "75",
                            "type": "move_names",
                            "attributes": {
                                "name": "RAZOR LEAF"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/75"
                            }
                        }
                    },
                    {
                        "level": 34,
                        "move": {
                            "id": "74",
                            "type": "move_names",
                            "attributes": {
                                "name": "GROWTH"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/74"
                            }
                        }
                    },
                    {
                        "level": 41,
                        "move": {
                            "id": "79",
                            "type": "move_names",
                            "attributes": {
                                "name": "SLEEP POWDER"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/79"
                            }
                        }
                    },
                    {
                        "level": 48,
                        "move": {
                            "id": "76",
                            "type": "move_names",
                            "attributes": {
                                "name": "SOLARBEAM"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/76"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/pokemon/learnsets/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/learnsets"
    }
}
{% end %}

---

### GET /v1/pokemon/learnsets/:pokedex_id {#get-pokemon-learnset}

Gets a Pokémon's learnset

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/learnsets/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                          | object |                                                 |
| `data.id`                                       | string | Pokédex ID. (identical to `:pokedex_id`)        |
| `data.type`                                     | string | Type of resource. Must be "pokemon_learnsets".  |
| `data.attributes`                               | object |                                                 |
| `data.attributes.learnset`                      | array  | Pokémon learnset.                               |
| `data.attributes.learnset[]`                    | object |                                                 |
| `data.attributes.learnset.level`                | number | Learn level.                                    |
| `data.attributes.learnset.move`                 | object |                                                 |
| `data.attributes.learnset.move.id`              | string | Move ID.                                        |
| `data.attributes.learnset.move.type`            | string | Type of move resource. Must be "move_names".    |
| `data.attributes.learnset.move.attributes`      | object |                                                 |
| `data.attributes.learnset.move.attributes.name` | string | Move name.                                      |
| `data.attributes.learnset.move.links`           | object |                                                 |
| `data.attributes.learnset.move.links.self`      | string | Link to move resource.                          |
| `data.links`                                    | object |                                                 |
| `data.links.self`                               | string | Link to current resource.                       |
| `links`                                         | object |                                                 |
| `links.self`                                    | string | Link to current resource.                       |
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
        "type": "pokemon_learnsets",
        "attributes": {
            "learnset": [
                {
                    "level": 7,
                    "move": {
                        "id": "73",
                        "type": "move_names",
                        "attributes": {
                            "name": "LEECH SEED"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/73"
                        }
                    }
                },
                {
                    "level": 13,
                    "move": {
                        "id": "22",
                        "type": "move_names",
                        "attributes": {
                            "name": "VINE WHIP"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/22"
                        }
                    }
                },
                {
                    "level": 20,
                    "move": {
                        "id": "77",
                        "type": "move_names",
                        "attributes": {
                            "name": "POISONPOWDER"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/77"
                        }
                    }
                },
                {
                    "level": 27,
                    "move": {
                        "id": "75",
                        "type": "move_names",
                        "attributes": {
                            "name": "RAZOR LEAF"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/75"
                        }
                    }
                },
                {
                    "level": 34,
                    "move": {
                        "id": "74",
                        "type": "move_names",
                        "attributes": {
                            "name": "GROWTH"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/74"
                        }
                    }
                },
                {
                    "level": 41,
                    "move": {
                        "id": "79",
                        "type": "move_names",
                        "attributes": {
                            "name": "SLEEP POWDER"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/79"
                        }
                    }
                },
                {
                    "level": 48,
                    "move": {
                        "id": "76",
                        "type": "move_names",
                        "attributes": {
                            "name": "SOLARBEAM"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/moves/names/76"
                        }
                    }
                }
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/learnsets/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/learnsets/1"
    }
}
{% end %}

---

### POST /v1/pokemon/learnsets/:pokedex_id {#post-pokemon-learnset}

Updates a Pokémon's learnsets

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`                        | string | ✔️ | Pokédex ID.                                |
| header | `X-Patch-Description`                | string |   | Description of change.                     |
| body   | `data`                               | object | ✔️ |                                            |
| body   | `data.type`                          | string | ✔️ | Type of data. Must be "pokemon_learnsets". |
| body   | `data.attributes`                    | object | ✔️ |                                            |
| body   | `data.attributes.learnset`           | array  | ✔️ | Pokémon learnset.                          |
| body   | `data.attributes.learnset[]`         | object | ✔️ |                                            |
| body   | `data.attributes.learnset[].level`   | number | ✔️ | Learn level.                               |
| body   | `data.attributes.learnset[].move`    | object | ✔️ |                                            |
| body   | `data.attributes.learnset[].move.id` | string | ✔️ | Move ID.                                   |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/learnsets/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR's learnset
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_learnsets",
        "attributes": {
            "learnset": [
                {
                    "level": 5,
                    "move": {
                        "id": "1"
                    }
                },
                {
                    "level": 6,
                    "move": {
                        "id": "2"
                    }
                },
                {
                    "level": 7,
                    "move": {
                        "id": "3"
                    }
                },
                {
                    "level": 8,
                    "move": {
                        "id": "4"
                    }
                },
                {
                    "level": 9,
                    "move": {
                        "id": "5"
                    }
                },
                {
                    "level": 10,
                    "move": {
                        "id": "6"
                    }
                },
                {
                    "level": 11,
                    "move": {
                        "id": "7"
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
