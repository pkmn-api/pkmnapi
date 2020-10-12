+++
title = "Pokémon Evolutions"
weight = 12
+++

| Endpoint                                                            | Description                    |
|---------------------------------------------------------------------|--------------------------------|
| [GET /v1/pokemon/evolutions/:pokedex_id](#get-pokemon-evolutions)   | Gets a Pokémon's evolutions    |
| [POST /v1/pokemon/evolutions/:pokedex_id](#post-pokemon-evolutions) | Updates a Pokémon's evolutions |

---

### GET /v1/pokemon/evolutions/:pokedex_id {#get-pokemon-evolutions}

Gets a Pokémon's evolutions

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/evolutions/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                                                 | object |                                                                               |
| `data.id`                                              | string | Pokédex ID. (identical to `:pokedex_id`)                                      |
| `data.type`                                            | string | Type of resource. Must be "pokemon_evolutions".                               |
| `data.attributes`                                      | object |                                                                               |
| `data.attributes.evolutions`                           | array  |                                                                               |
| `data.attributes.evolutions[].evolution_type`          | string | Pokémon evolution type. (must be "level", "item", or "trade")                 |
| `data.attributes.evolutions[].level`                   | number | Level at which Pokémon evolves. (only present if `evolution_type` is "level") |
| `data.attributes.evolutions[].item`                    | number | Item that evolves the Pokémon. (only present if `evolution_type` is "item")   |
| `data.attributes.evolutions[].item.id`                 | string | Item ID.                                                                      |
| `data.attributes.evolutions[].item.type`               | string | Type of resource. Must be "item_names".                                       |
| `data.attributes.evolutions[].item.attributes`         | object |                                                                               |
| `data.attributes.evolutions[].item.attributes.name`    | string | Item name.                                                                    |
| `data.attributes.evolutions[].item.links`              | object |                                                                               |
| `data.attributes.evolutions[].item.links.self`         | string | Link to item resource.                                                        |
| `data.attributes.evolutions[].pokemon`                 | object | Pokémon.                                                                      |
| `data.attributes.evolutions[].pokemon.id`              | string | Pokédex ID.                                                                   |
| `data.attributes.evolutions[].pokemon.type`            | string | Type of resource. Must be "pokemon_names".                                    |
| `data.attributes.evolutions[].pokemon.attributes`      | object |                                                                               |
| `data.attributes.evolutions[].pokemon.attributes.name` | string | Pokémon name.                                                                 |
| `data.attributes.evolutions[].pokemon.links`           | object |                                                                               |
| `data.attributes.evolutions[].pokemon.links.self`      | string | Link to Pokémon resource.                                                     |
| `data.links`                                           | object |                                                                               |
| `data.links.self`                                      | string | Link to current resource.                                                     |
| `links`                                                | object |                                                                               |
| `links.self`                                           | string | Link to current resource.                                                     |
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
        "type": "pokemon_evolutions",
        "attributes": {
            "evolutions": [
                {
                    "evolution_type": "level",
                    "level": 16,
                    "pokemon": {
                        "id": "2",
                        "type": "pokemon_names",
                        "attributes": {
                            "name": "IVYSAUR"
                        },
                        "links": {
                            "self": "{{API_DOMAIN}}/v1/pokemon/names/1"
                        }
                    }
                }
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/evolutions/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/evolutions/1"
    }
}
{% end %}

---

### POST /v1/pokemon/evolutions/:pokedex_id {#post-pokemon-evolutions}

Updates a Pokémon's evolutions

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`                                 | string | ✔️                                  | Pokédex ID.                                                   |
| header | `X-Patch-Description`                         | string |                                    | Description of change.                                        |
| body   | `data`                                        | object | ✔️                                  |                                                               |
| body   | `data.type`                                   | string | ✔️                                  | Type of data. Must be "pokemon_evolutions".                   |
| body   | `data.attributes`                             | object | ✔️                                  |                                                               |
| body   | `data.attributes.evolutions`                  | array  | ✔️                                  |                                                               |
| body   | `data.attributes.evolutions[].evolution_type` | string | ✔️                                  | Pokémon evolution type. (must be "level", "item", or "trade") |
| body   | `data.attributes.evolutions[].level`          | number | ✔️ (if `evolution_type` is "level") | Level at which Pokémon evolves.                               |
| body   | `data.attributes.evolutions[].item`           | number | ✔️ (if `evolution_type` is "item")  | Item that evolves the Pokémon.                                |
| body   | `data.attributes.evolutions[].item.id`        | string | ✔️ (if `evolution_type` is "item")  | Item ID.                                                      |
| body   | `data.attributes.evolutions[].pokemon`        | object | ✔️                                  | Pokémon.                                                      |
| body   | `data.attributes.evolutions[].pokemon.id`     | string | ✔️                                  | Pokédex ID.                                                   |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/evolutions/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR to evolve into PIKACHU
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_evolutions",
        "attributes": {
            "evolutions": [
                {
                    "evolution_type": "level",
                    "level": 16,
                    "pokemon": {
                        "id": "25"
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
