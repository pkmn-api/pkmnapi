+++
title = "Map Pokémon"
weight = 7
+++

| Endpoint                                           | Description                |
|----------------------------------------------------|----------------------------|
| [GET /v1/maps/pokemon](#get-map-pokemon-all)       | Gets a list of map Pokémon |
| [GET /v1/maps/pokemon/:map_id](#get-map-pokemon)   | Gets a map's Pokémon       |
| [POST /v1/maps/pokemon/:map_id](#post-map-pokemon) | Updates a map's Pokémon    |

---

### GET /v1/maps/pokemon {#get-map-pokemon-all}

Gets a list of map Pokémon

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/maps/pokemon
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                                            | array  |                                            |
| `data[]`                                                          | object |                                            |
| `data[].id`                                                       | string | Map ID. (identical to `:map_id`)           |
| `data[].type`                                                     | string | Type of resource. Must be "map_pokemon".   |
| `data[].attributes`                                               | object |                                            |
| `data[].attributes.grass`                                         | object | Grass Pokémon.                             |
| `data[].attributes.grass.encounter_rate`                          | number | Encounter rate.                            |
| `data[].attributes.grass.pokemon`                                 | array  |                                            |
| `data[].attributes.grass.pokemon[].level`                         | number | Pokémon level.                             |
| `data[].attributes.grass.pokemon[].level.pokemon`                 | object | Pokémon.                                   |
| `data[].attributes.grass.pokemon[].level.pokemon.id`              | string | Pokémon ID.                                |
| `data[].attributes.grass.pokemon[].level.pokemon.type`            | string | Type of resource. Must be "pokemon_names". |
| `data[].attributes.grass.pokemon[].level.pokemon.attributes`      | object |                                            |
| `data[].attributes.grass.pokemon[].level.pokemon.attributes.name` | string | Pokémon name.                              |
| `data[].attributes.grass.pokemon[].level.pokemon.links`           | object |                                            |
| `data[].attributes.grass.pokemon[].level.pokemon.links.self`      | string | Link to Pokémon resource.                  |
| `data[].attributes.water`                                         | object | Water Pokémon.                             |
| `data[].attributes.water.encounter_rate`                          | number | Encounter rate.                            |
| `data[].attributes.water.pokemon`                                 | array  |                                            |
| `data[].attributes.water.pokemon[].level`                         | number | Pokémon level.                             |
| `data[].attributes.water.pokemon[].level.pokemon`                 | object | Pokémon.                                   |
| `data[].attributes.water.pokemon[].level.pokemon.id`              | string | Pokémon ID.                                |
| `data[].attributes.water.pokemon[].level.pokemon.type`            | string | Type of resource. Must be "pokemon_names". |
| `data[].attributes.water.pokemon[].level.pokemon.attributes`      | object |                                            |
| `data[].attributes.water.pokemon[].level.pokemon.attributes.name` | string | Pokémon name.                              |
| `data[].attributes.water.pokemon[].level.pokemon.links`           | object |                                            |
| `data[].attributes.water.pokemon[].level.pokemon.links.self`      | string | Link to Pokémon resource.                  |
| `data[].links`                                                    | object |                                            |
| `data[].links.self`                                               | string | Link to current resource.                  |
| `links`                                                           | object |                                            |
| `links.self`                                                      | string | Link to list resource.                     |
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
            "id": "12",
            "type": "map_pokemon",
            "attributes": {
                "grass": {
                    "encounter_rate": 25,
                    "pokemon": [
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 2,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 2,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 4,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 4,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                                }
                            }
                        }
                    ]
                },
                "water": {
                    "encounter_rate": 0,
                    "pokemon": []
                }
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/maps/pokemon/12"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/maps/pokemon"
    }
}
{% end %}

---

### GET /v1/maps/pokemon/:map_id {#get-map-pokemon}

Gets a map's Pokémon

#### Request Parameters

{% api_request_params() %}
| url | `:map_id` | string | ✔️ | Map ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/maps/pokemon/12
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                                          | object |                                            |
| `data.id`                                                       | string | Map ID. (identical to `:map_id`)           |
| `data.type`                                                     | string | Type of resource. Must be "map_pokemon".   |
| `data.attributes`                                               | object |                                            |
| `data.attributes.grass`                                         | object | Grass Pokémon.                             |
| `data.attributes.grass.encounter_rate`                          | number | Encounter rate.                            |
| `data.attributes.grass.pokemon`                                 | array  |                                            |
| `data.attributes.grass.pokemon[].level`                         | number | Pokémon level.                             |
| `data.attributes.grass.pokemon[].level.pokemon`                 | object | Pokémon.                                   |
| `data.attributes.grass.pokemon[].level.pokemon.id`              | string | Pokémon ID.                                |
| `data.attributes.grass.pokemon[].level.pokemon.type`            | string | Type of resource. Must be "pokemon_names". |
| `data.attributes.grass.pokemon[].level.pokemon.attributes`      | object |                                            |
| `data.attributes.grass.pokemon[].level.pokemon.attributes.name` | string | Pokémon name.                              |
| `data.attributes.grass.pokemon[].level.pokemon.links`           | object |                                            |
| `data.attributes.grass.pokemon[].level.pokemon.links.self`      | string | Link to Pokémon resource.                  |
| `data.attributes.water`                                         | object | Water Pokémon.                             |
| `data.attributes.water.encounter_rate`                          | number | Encounter rate.                            |
| `data.attributes.water.pokemon`                                 | array  |                                            |
| `data.attributes.water.pokemon[].level`                         | number | Pokémon level.                             |
| `data.attributes.water.pokemon[].level.pokemon`                 | object | Pokémon.                                   |
| `data.attributes.water.pokemon[].level.pokemon.id`              | string | Pokémon ID.                                |
| `data.attributes.water.pokemon[].level.pokemon.type`            | string | Type of resource. Must be "pokemon_names". |
| `data.attributes.water.pokemon[].level.pokemon.attributes`      | object |                                            |
| `data.attributes.water.pokemon[].level.pokemon.attributes.name` | string | Pokémon name.                              |
| `data.attributes.water.pokemon[].level.pokemon.links`           | object |                                            |
| `data.attributes.water.pokemon[].level.pokemon.links.self`      | string | Link to Pokémon resource.                  |
| `data.links`                                                    | object |                                            |
| `data.links.self`                                               | string | Link to current resource.                  |
| `links`                                                         | object |                                            |
| `links.self`                                                    | string | Link to current resource.                  |
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
        "id": "12",
        "type": "map_pokemon",
        "attributes": {
            "grass": {
                "encounter_rate": 25,
                "pokemon": [
                    {
                        "level": 3,
                        "pokemon": {
                            "id": "16",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "PIDGEY"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                            }
                        }
                    },
                    {
                        "level": 3,
                        "pokemon": {
                            "id": "19",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "RATTATA"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                            }
                        }
                    },
                    {
                        "level": 3,
                        "pokemon": {
                            "id": "19",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "RATTATA"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                            }
                        }
                    },
                    {
                        "level": 2,
                        "pokemon": {
                            "id": "19",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "RATTATA"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                            }
                        }
                    },
                    {
                        "level": 2,
                        "pokemon": {
                            "id": "16",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "PIDGEY"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                            }
                        }
                    },
                    {
                        "level": 3,
                        "pokemon": {
                            "id": "16",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "PIDGEY"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                            }
                        }
                    },
                    {
                        "level": 3,
                        "pokemon": {
                            "id": "16",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "PIDGEY"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                            }
                        }
                    },
                    {
                        "level": 4,
                        "pokemon": {
                            "id": "19",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "RATTATA"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/19"
                            }
                        }
                    },
                    {
                        "level": 4,
                        "pokemon": {
                            "id": "16",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "PIDGEY"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                            }
                        }
                    },
                    {
                        "level": 5,
                        "pokemon": {
                            "id": "16",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "PIDGEY"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/16"
                            }
                        }
                    }
                ]
            },
            "water": {
                "encounter_rate": 0,
                "pokemon": []
            }
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/maps/pokemon/12"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/maps/pokemon/12"
    }
}
{% end %}

---

### POST /v1/maps/pokemon/:map_id {#post-map-pokemon}

Updates a map's Pokémon

#### Request Parameters

{% api_request_params() %}
| url    | `:map_id`                                    | string | ✔️ | Pokédex ID.                          |
| header | `X-Patch-Description`                        | string |   | Description of change.               |
| body   | `data`                                       | object | ✔️ |                                      |
| body   | `data.type`                                  | string | ✔️ | Type of data. Must be "map_pokemon". |
| body   | `data.attributes`                            | object | ✔️ |                                      |
| body   | `data.attributes.grass`                      | object | ✔️ | Grass Pokémon.                       |
| body   | `data.attributes.grass.encounter_rate`       | number | ✔️ | Encounter rate.                      |
| body   | `data.attributes.grass.pokemon`              | object | ✔️ |                                      |
| body   | `data.attributes.grass.pokemon[].level`      | number | ✔️ | Pokémon level.                       |
| body   | `data.attributes.grass.pokemon[].pokemon`    | object | ✔️ | Pokémon.                             |
| body   | `data.attributes.grass.pokemon[].pokemon.id` | string | ✔️ | Pokémon ID.                          |
| body   | `data.attributes.water`                      | object | ✔️ | Water Pokémon.                       |
| body   | `data.attributes.water.encounter_rate`       | number | ✔️ | Encounter rate.                      |
| body   | `data.attributes.water.pokemon`              | object | ✔️ |                                      |
| body   | `data.attributes.water.pokemon[].level`      | number | ✔️ | Pokémon level.                       |
| body   | `data.attributes.water.pokemon[].pokemon`    | object | ✔️ | Pokémon.                             |
| body   | `data.attributes.water.pokemon[].pokemon.id` | string | ✔️ | Pokémon ID.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/maps/pokemon/12
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update Route 1 Pokémon to all PIKACHU
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "map_pokemon",
        "attributes": {
            "grass": {
                "encounter_rate": 25,
                "pokemon": [
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    },
                    {
                        "level": 10,
                        "pokemon": {
                            "id": "25"
                        }
                    }
                ]
            },
            "water": {
                "encounter_rate": 0,
                "pokemon": []
            }
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
