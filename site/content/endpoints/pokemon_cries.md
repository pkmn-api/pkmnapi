+++
title = "Pokémon Cries"
weight = 13
+++

| Endpoint                                                | Description                  |
|---------------------------------------------------------|------------------------------|
| [GET /v1/pokemon/cries](#get-pokemon-cry-all)           | Gets a list of Pokémon cries |
| [GET /v1/pokemon/cries/:pokedex_id](#get-pokemon-cry)   | Gets a Pokémon's cry         |
| [POST /v1/pokemon/cries/:pokedex_id](#post-pokemon-cry) | Updates a Pokémon's cry      |

---

### GET /v1/pokemon/cries {#get-pokemon-cry-all}

Gets a list of Pokémon cries

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/cries
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                     | array  |                                            |
| `data[]`                   | object |                                            |
| `data[].id`                | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data[].type`              | string | Type of resource. Must be "pokemon_cries". |
| `data[].attributes`        | object |                                            |
| `data[].attributes.base`   | number | Cry base.                                  |
| `data[].attributes.pitch`  | number | Cry pitch.                                 |
| `data[].attributes.length` | number | Cry length.                                |
| `data[].links`             | object |                                            |
| `data[].links.self`        | string | Link to current resource.                  |
| `links`                    | object |                                            |
| `links.self`               | string | Link to list resource.                     |
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
            "type": "pokemon_cries",
            "attributes": {
                "base": 15,
                "pitch": 128,
                "length": 1
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/pokemon/cries/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/cries"
    }
}
{% end %}

---

### GET /v1/pokemon/cries/:pokedex_id {#get-pokemon-cry}

Gets a Pokémon's cry

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id` | string | ✔️ | Pokédex ID.                                                          |
| header | `Accept`      | string | ✔️ | Type of result to return. Must be "application/json" or "audio/wav". |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/cries/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Accept: audio/wav
{% end %}

OR

{% api_headers() %}
GET /v1/pokemon/cries/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Accept: application/json
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                   | object |                                            |
| `data.id`                | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data.type`              | string | Type of resource. Must be "pokemon_cries". |
| `data.attributes`        | object |                                            |
| `data.attributes.base`   | number | Cry base.                                  |
| `data.attributes.pitch`  | number | Cry pitch.                                 |
| `data.attributes.length` | number | Cry length.                                |
| `data.links`             | object |                                            |
| `data.links.self`        | string | Link to current resource.                  |
| `links`                  | object |                                            |
| `links.self`             | string | Link to current resource.                  |
{% end %}

#### Example Response

**Headers:**

{% api_headers() %}
HTTP/1.1 200 OK
Content-Type: audio/wav
Server: pkmnapi/0.1.0
{% end %}

OR

{% api_headers() %}
HTTP/1.1 200 OK
Content-Type: application/json
Server: pkmnapi/0.1.0
{% end %}

**Body:**

<audio src="/audio/BULBASAUR.wav" controls></audio>

OR

{% api_response() %}
{
    "data": {
        "id": "1",
        "type": "pokemon_cries",
        "attributes": {
            "base": 15,
            "pitch": 128,
            "length": 1
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/cries/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/cries/1"
    }
}
{% end %}

---

### POST /v1/pokemon/cries/:pokedex_id {#post-pokemon-cry}

Updates a Pokémon's cry

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`            | string | ✔️ | Pokédex ID.                            |
| header | `X-Patch-Description`    | string |   | Description of change.                 |
| body   | `data`                   | object | ✔️ |                                        |
| body   | `data.type`              | string | ✔️ | Type of data. Must be "pokemon_cries". |
| body   | `data.attributes`        | object | ✔️ |                                        |
| body   | `data.attributes.base`   | number | ✔️ | Cry base.                              |
| body   | `data.attributes.pitch`  | number | ✔️ | Cry pitch.                             |
| body   | `data.attributes.length` | number | ✔️ | Cry length.                            |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/cries/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR's cry
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_cries",
        "attributes": {
            "base": 13,
            "pitch": 128,
            "length": 10
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
