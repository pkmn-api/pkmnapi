+++
title = "Pokémon Machines"
weight = 13
+++

| Endpoint                                                        | Description                  |
|-----------------------------------------------------------------|------------------------------|
| [GET /v1/pokemon/machines/:pokedex_id](#get-pokemon-machines)   | Gets a Pokémon's machines    |
| [POST /v1/pokemon/machines/:pokedex_id](#post-pokemon-machines) | Updates a Pokémon's machines |

---

### GET /v1/pokemon/machines/:pokedex_id {#get-pokemon-machines}

Gets a Pokémon's machines

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/machines/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                                       | object |                                                 |
| `data.id`                                                    | string | Pokédex ID. (identical to `:pokedex_id`)        |
| `data.type`                                                  | string | Type of resource. Must be "pokemon_machines".   |
| `data.attributes`                                            | object |                                                 |
| `data.attributes.machines`                                   | array  | Pokémon machines.                               |
| `data.attributes.machines[]`                                 | object | Pokémon machine.                                |
| `data.attributes.machines[].id`                              | string | Machine ID.                                     |
| `data.attributes.machines[].type`                            | string | Machine type. Must be "tm_moves" or "hm_moves". |
| `data.attributes.machines[].attributes`                      | object |                                                 |
| `data.attributes.machines[].attributes.move`                 | object |                                                 |
| `data.attributes.machines[].attributes.move.id`              | string | Move ID.                                        |
| `data.attributes.machines[].attributes.move.type`            | string | Type of move resource. Must be "move names".    |
| `data.attributes.machines[].attributes.move.attributes`      | object |                                                 |
| `data.attributes.machines[].attributes.move.attributes.name` | string | Move name.                                      |
| `data.attributes.machines[].attributes.move.links`           | object |                                                 |
| `data.attributes.machines[].attributes.move.links.self`      | string | Link to move resource.                          |
| `data.attributes.machines[].links`                           | object |                                                 |
| `data.attributes.machines[].links.self`                      | string | Link to machine resource.                       |
| `data.links`                                                 | object |                                                 |
| `data.links.self`                                            | string | Link to current resource.                       |
| `links`                                                      | object |                                                 |
| `links.self`                                                 | string | Link to current resource.                       |
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
        "type": "pokemon_machines",
        "attributes": {
            "machines": [
                {
                    "id": "3",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "14",
                            "type": "move_names",
                            "attributes": {
                                "name": "SWORDS DANCE"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/14"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/3"
                    }
                },
                {
                    "id": "6",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "92",
                            "type": "move_names",
                            "attributes": {
                                "name": "TOXIC"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/92"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/6"
                    }
                },
                {
                    "id": "8",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "34",
                            "type": "move_names",
                            "attributes": {
                                "name": "BODY SLAM"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/34"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/8"
                    }
                },
                {
                    "id": "9",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "36",
                            "type": "move_names",
                            "attributes": {
                                "name": "TAKE DOWN"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/36"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/9"
                    }
                },
                {
                    "id": "10",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "38",
                            "type": "move_names",
                            "attributes": {
                                "name": "DOUBLE-EDGE"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/38"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/10"
                    }
                },
                {
                    "id": "20",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "99",
                            "type": "move_names",
                            "attributes": {
                                "name": "RAGE"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/99"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/20"
                    }
                },
                {
                    "id": "21",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "72",
                            "type": "move_names",
                            "attributes": {
                                "name": "MEGA DRAIN"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/72"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/21"
                    }
                },
                {
                    "id": "22",
                    "type": "tm_moves",
                    "attributes": {
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
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/22"
                    }
                },
                {
                    "id": "31",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "102",
                            "type": "move_names",
                            "attributes": {
                                "name": "MIMIC"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/102"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/31"
                    }
                },
                {
                    "id": "32",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "104",
                            "type": "move_names",
                            "attributes": {
                                "name": "DOUBLE TEAM"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/104"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/32"
                    }
                },
                {
                    "id": "33",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "115",
                            "type": "move_names",
                            "attributes": {
                                "name": "REFLECT"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/115"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/33"
                    }
                },
                {
                    "id": "34",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "117",
                            "type": "move_names",
                            "attributes": {
                                "name": "BIDE"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/117"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/34"
                    }
                },
                {
                    "id": "44",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "156",
                            "type": "move_names",
                            "attributes": {
                                "name": "REST"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/156"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/44"
                    }
                },
                {
                    "id": "50",
                    "type": "tm_moves",
                    "attributes": {
                        "move": {
                            "id": "164",
                            "type": "move_names",
                            "attributes": {
                                "name": "SUBSTITUTE"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/164"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/tms/moves/50"
                    }
                },
                {
                    "id": "1",
                    "type": "hm_moves",
                    "attributes": {
                        "move": {
                            "id": "15",
                            "type": "move_names",
                            "attributes": {
                                "name": "CUT"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/moves/names/15"
                            }
                        }
                    },
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/hms/moves/1"
                    }
                }
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/machines/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/machines/1"
    }
}
{% end %}

---

### POST /v1/pokemon/machines/:pokedex_id {#post-pokemon-machines}

Updates a Pokémon's machines

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`                     | string | ✔️ | Pokédex ID.                                     |
| header | `X-Patch-Description`             | string |   | Description of change.                          |
| body   | `data`                            | object | ✔️ |                                                 |
| body   | `data.type`                       | string | ✔️ | Type of data. Must be "pokemon_machines".       |
| body   | `data.attributes`                 | object | ✔️ |                                                 |
| body   | `data.attributes.machines`        | array  | ✔️ | Pokémon machines.                               |
| body   | `data.attributes.machines[]`      | object | ✔️ | Pokémon machine.                                |
| body   | `data.attributes.machines[].id`   | string | ✔️ | Machine ID.                                     |
| body   | `data.attributes.machines[].type` | string | ✔️ | Machine type. Must be "tm_moves" or "hm_moves". |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/machines/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR to only be able to CUT
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_machines",
        "attributes": {
            "machines": [
                {
                    "id": "1",
                    "type": "hm_moves"
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
