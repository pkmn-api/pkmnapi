+++
title = "Pokédex Entries"
weight = 10
+++

| Endpoint                                                    | Description                    |
|-------------------------------------------------------------|--------------------------------|
| [GET /v1/pokedex/entries](#get-pokedex-entry-all)           | Gets a list of Pokédex entries |
| [GET /v1/pokedex/entries/:pokedex_id](#get-pokedex-entry)   | Gets a Pokédex entry           |
| [POST /v1/pokedex/entries/:pokedex_id](#post-pokedex-entry) | Updates a Pokédex entry        |

---

### GET /v1/pokedex/entries {#get-pokedex-entry-all}

Gets a list of Pokédex entries

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokedex/entries
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                      | array  |                                              |
| `data[]`                    | object |                                              |
| `data[].id`                 | string | Pokédex ID. (identical to `:pokedex_id`)     |
| `data[].type`               | string | Type of resource. Must be "pokedex_entries". |
| `data[].attributes`         | object |                                              |
| `data[].attributes.species` | string | Pokémon species.                             |
| `data[].attributes.height`  | number | Pokémon height. (inches)                     |
| `data[].attributes.weight`  | number | Pokémon weight. (pounds)                     |
| `data[].links`              | object |                                              |
| `data[].links.self`         | string | Link to current resource.                    |
| `links`                     | object |                                              |
| `links.self`                | string | Link to list resource.                       |
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
            "type": "pokemon_entries",
            "attributes": {
                "species": "SEED",
                "height": 28,
                "weight": 150
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/pokedex/entries/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokedex/entries"
    }
}
{% end %}

---

### GET /v1/pokedex/entries/:pokedex_id {#get-pokedex-entry}

Gets a Pokédex entry

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokedex/entries/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                    | object |                                              |
| `data.id`                 | string | Pokédex ID. (identical to `:pokedex_id`)     |
| `data.type`               | string | Type of resource. Must be "pokedex_entries". |
| `data.attributes`         | object |                                              |
| `data.attributes.species` | string | Pokémon species.                             |
| `data.attributes.height`  | number | Pokémon height. (inches)                     |
| `data.attributes.weight`  | number | Pokémon weight. (pounds)                     |
| `data.links`              | object |                                              |
| `data.links.self`         | string | Link to current resource.                    |
| `links`                   | object |                                              |
| `links.self`              | string | Link to current resource.                    |
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
        "type": "pokemon_entries",
        "attributes": {
            "species": "SEED",
            "height": 28,
            "weight": 150
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokedex/entries/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokedex/entries/1"
    }
}
{% end %}

---

### POST /v1/pokedex/entries/:pokedex_id {#post-pokedex-entry}

Updates a Pokédex entry

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`             | string | ✔️ | Pokédex ID.                              |
| header | `X-Patch-Description`     | string |   | Description of change.                   |
| body   | `data`                    | object | ✔️ |                                          |
| body   | `data.type`               | string | ✔️ | Type of data. Must be "pokedex_entries". |
| body   | `data.attributes`         | object | ✔️ |                                          |
| body   | `data.attributes.species` | string | ✔️ | Pokémon species.                         |
| body   | `data.attributes.height`  | string | ✔️ | Pokémon height. (inches)                 |
| body   | `data.attributes.weight`  | string | ✔️ | Pokémon weight. (pounds)                 |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokedex/entries/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR's species to LEAF
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_entries",
        "attributes": {
            "species": "LEAF",
            "height": 28,
            "weight": 150
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
