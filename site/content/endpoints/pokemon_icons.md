+++
title = "Pokémon Icons"
weight = 15
+++

| Endpoint                                                 | Description                  |
|----------------------------------------------------------|------------------------------|
| [GET /v1/pokemon/icons](#get-pokemon-icon-all)           | Gets a list of Pokémon icons |
| [GET /v1/pokemon/icons/:pokedex_id](#get-pokemon-icon)   | Gets a Pokémon's icon        |
| [POST /v1/pokemon/icons/:pokedex_id](#post-pokemon-icon) | Updates a Pokémon's icon     |

---

### GET /v1/pokemon/icons {#get-pokemon-icon-all}

Gets a list of Pokémon icons

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/icons
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                              | array  |                                            |
| `data[]`                            | object |                                            |
| `data[].id`                         | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data[].type`                       | string | Type of resource. Must be "pokemon_icons". |
| `data[].attributes`                 | object |                                            |
| `data[].attributes.icon`            | object |                                            |
| `data[].attributes.icon.id`         | string | Icon ID.                                   |
| `data[].attributes.icon.type`       | string | Type of resource. Must be "icons".         |
| `data[].attributes.icon.attributes` | object |                                            |
| `data[].attributes.icon.links`      | object |                                            |
| `data[].attributes.icon.links.self` | string | Link to icon resource.                     |
| `data[].links`                      | object |                                            |
| `data[].links.self`                 | string | Link to current resource.                  |
| `links`                             | object |                                            |
| `links.self`                        | string | Link to list resource.                     |
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
            "type": "pokemon_icons",
            "attributes": {
                "icon": {
                    "id": "7",
                    "type": "icons",
                    "attributes": {},
                    "links": {
                        "self": "{{API_DOMAIN}}/v1/icons/7"
                    }
                }
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/pokemon/icons/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/icons"
    }
}
{% end %}

---

### GET /v1/pokemon/icons/:pokedex_id {#get-pokemon-icon}

Gets a Pokémon's icon

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/icons/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                            | object |                                            |
| `data.id`                         | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data.type`                       | string | Type of resource. Must be "pokemon_icons". |
| `data.attributes`                 | object |                                            |
| `data.attributes.icon`            | object |                                            |
| `data.attributes.icon.id`         | string | Icon ID.                                   |
| `data.attributes.icon.type`       | string | Type of resource. Must be "icons".         |
| `data.attributes.icon.attributes` | object |                                            |
| `data.attributes.icon.links`      | object |                                            |
| `data.attributes.icon.links.self` | string | Link to icon resource.                     |
| `data.links`                      | object |                                            |
| `data.links.self`                 | string | Link to current resource.                  |
| `links`                           | object |                                            |
| `links.self`                      | string | Link to current resource.                  |
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
        "type": "pokemon_icons",
        "attributes": {
            "icon": {
                "id": "7",
                "type": "icons",
                "attributes": {},
                "links": {
                    "self": "{{API_DOMAIN}}/v1/icons/7"
                }
            }
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/icons/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/icons/1"
    }
}
{% end %}

---

### POST /v1/pokemon/icons/:pokedex_id {#post-pokemon-icon}

Updates a Pokémon's icon

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`             | string | ✔️ | Pokédex ID.                            |
| header | `X-Patch-Description`     | string |   | Description of change.                 |
| body   | `data`                    | object | ✔️ |                                        |
| body   | `data.type`               | string | ✔️ | Type of data. Must be "pokemon_icons". |
| body   | `data.attributes`         | object | ✔️ |                                        |
| body   | `data.attributes.icon`    | object | ✔️ |                                        |
| body   | `data.attributes.icon.id` | string | ✔️ | Icon ID.                               |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/icons/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR to snake icon
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokemon_icons",
        "attributes": {
            "icon": {
                "id": "8"
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
