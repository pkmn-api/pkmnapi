+++
title = "Trainer Parties"
weight = 17
+++

| Endpoint                                                       | Description                 |
|----------------------------------------------------------------|-----------------------------|
| [GET /v1/trainers/parties/:trainer_id](#get-trainer-parties)   | Gets a trainer's parties    |
| [POST /v1/trainers/parties/:trainer_id](#post-trainer-parties) | Updates a trainer's parties |

---

### GET /v1/trainers/parties/:trainer_id {#get-trainer-parties}

Gets a trainer's parties

#### Request Parameters

{% api_request_params() %}
| url | `:trainer_id` | string | ✔️ | Trainer ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/trainers/parties/34
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                                      | object |                                              |
| `data.id`                                                   | string | Trainer ID. (identical to `:trainer_id`)     |
| `data.type`                                                 | string | Type of resource. Must be "trainer_parties". |
| `data.attributes`                                           | object |                                              |
| `data.attributes.parties`                                   | array  | List of trainer parties.                     |
| `data.attributes.parties[].party`                           | array  | List of Pokémon in party.                    |
| `data.attributes.parties[].party[].level`                   | number | Level of Pokémon.                            |
| `data.attributes.parties[].party[].pokemon`                 | object | Pokémon.                                     |
| `data.attributes.parties[].party[].pokemon.id`              | string | Pokédex ID.                                  |
| `data.attributes.parties[].party[].pokemon.type`            | string | Type of resource. Must be "pokemon_names"    |
| `data.attributes.parties[].party[].pokemon.attributes`      | object |                                              |
| `data.attributes.parties[].party[].pokemon.attributes.name` | string | Pokémon name.                                |
| `data.attributes.parties[].party[].pokemon.links`           | object |                                              |
| `data.attributes.parties[].party[].pokemon.links.self`      | string | Link to Pokémon resource.                    |
| `data.links`                                                | object |                                              |
| `data.links.self`                                           | string | Link to current resource.                    |
| `links`                                                     | object |                                              |
| `links.self`                                                | string | Link to current resource.                    |
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
        "id": "34",
        "type": "trainer_parties",
        "attributes": {
        "parties": [
            {
                "party": [
                    {
                        "level": 12,
                        "pokemon": {
                            "id": "74",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "GEODUDE"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/74"
                            }
                        }
                    },
                    {
                        "level": 14,
                        "pokemon": {
                            "id": "95",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "ONIX"
                            },
                            "links": {
                                "self": "{{API_DOMAIN}}/v1/pokemon/names/95"
                            }
                        }
                    }
                ]
            }
        ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/trainers/parties/34"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/trainers/parties/34"
    }
}
{% end %}

---

### POST /v1/trainers/parties/:trainer_id {#post-trainer-parties}

Updates a trainer's parties

#### Request Parameters

{% api_request_params() %}
| url    | `:trainer_id`                                  | string | ✔️ | Trainer ID.                              |
| header | `X-Patch-Description`                          | string |   | Description of change.                   |
| body   | `data`                                         | object | ✔️ |                                          |
| body   | `data.type`                                    | string | ✔️ | Type of data. Must be "trainer_parties". |
| body   | `data.attributes`                              | object | ✔️ |                                          |
| body   | `data.attributes.parties`                      | array  | ✔️ | List of trainer parties.                 |
| body   | `data.attributes.parties[].party`              | array  | ✔️ | List of Pokémon in party.                |
| body   | `data.attributes.parties[].party[].level`      | number | ✔️ | Level of Pokémon.                        |
| body   | `data.attributes.parties[].party[].pokemon`    | object | ✔️ | Pokémon.                                 |
| body   | `data.attributes.parties[].party[].pokemon.id` | string | ✔️ | Pokédex ID.                              |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/trainers/parties/34
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BROCK's party to all GEODUDE
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "trainer_parties",
        "attributes": {
            "parties": [
                {
                    "party": [
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        }
                    ]
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
