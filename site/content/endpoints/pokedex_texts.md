+++
title = "Pokédex Texts"
weight = 11
+++

| Endpoint                                                   | Description            |
|------------------------------------------------------------|------------------------|
| [GET /v1/pokedex/texts/:pokedex_id](#get-pokedex-text)   | Gets a Pokédex text    |
| [POST /v1/pokedex/texts/:pokedex_id](#post-pokedex-text) | Updates a Pokédex text |

---

### GET /v1/pokedex/texts/:pokedex_id {#get-pokedex-text}

Gets a Pokédex text

#### Request Parameters

{% api_request_params() %}
| url | `:pokedex_id` | string | ✔️ | Pokédex ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokedex/texts/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                 | object |                                            |
| `data.id`              | string | Pokédex ID. (identical to `:pokedex_id`)   |
| `data.type`            | string | Type of resource. Must be "pokedex_texts". |
| `data.attributes`      | object |                                            |
| `data.attributes.text` | string | Pokédex text.                              |
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
        "type": "pokedex_texts",
        "attributes": {
            "text": "A strange seed was\nplanted on its\nback at birth.¶The plant sprouts\nand grows with\nthis #MON"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokedex/texts/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokedex/texts/1"
    }
}
{% end %}

---

### POST /v1/pokedex/texts/:pokedex_id {#post-pokedex-text}

Updates a Pokédex text

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`          | string | ✔️ | Pokédex ID.                            |
| header | `X-Patch-Description`  | string |   | Description of change.                 |
| body   | `data`                 | object | ✔️ |                                        |
| body   | `data.type`            | string | ✔️ | Type of data. Must be "pokedex_texts". |
| body   | `data.attributes`      | object | ✔️ |                                        |
| body   | `data.attributes.text` | string | ✔️ | Pokédex text.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokedex/texts/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update BULBASAUR's species to LEAF
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "pokedex_texts",
        "attributes": {
            "text": "When in disgrace\nwith fortune\nand men's eyes,¶I all alone beweep\nmy outcast\nstate"
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
