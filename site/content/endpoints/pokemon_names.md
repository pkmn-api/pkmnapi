+++
title = "Pokémon Names"
weight = 13
+++

| Endpoint                                                 | Description              |
|----------------------------------------------------------|--------------------------|
| [GET /v1/pokemon/names/:pokedex_id](#get-pokemon-name)   | Gets a Pokémon's name    |
| [POST /v1/pokemon/names/:pokedex_id](#post-pokemon-name) | Updates a Pokémon's name |

---

### GET /v1/pokemon/names/:pokedex_id {#get-pokemon-name}

Gets a Pokémon's name

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                            |
| `data.id`              | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data.type`            | string | Type of resource. Must be "pokemon_names". |
| `data.attributes`      | object |                                            |
| `data.attributes.name` | string | Pokémon name.                              |
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
        "type": "pokemon_names",
        "attributes": {
            "name": "BULBASAUR"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/names/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/names/1"
    }
}
{% end %}

---

### POST /v1/pokemon/names/:pokedex_id {#post-pokemon-name}

Updates a Pokémon's name

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`          | string | ✔️ | Pokédex ID.                            |
| header | `X-Patch-Description`  | string |   | Description of change.                 |
| body   | `data`                 | object | ✔️ |                                        |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "pokemon_names". |
| body   | `data.attributes`      | object | ✔️ |                                        |
| body   | `data.attributes.name` | string | ✔️ | Pokémon name.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/names/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR to LEAFY-BOI
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_names",
        "attributes": {
            "name": "LEAFY-BOI"
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
