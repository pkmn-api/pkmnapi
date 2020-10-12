+++
title = "Pokémon Pics"
weight = 14
+++

| Endpoint                                               | Description             |
|--------------------------------------------------------|-------------------------|
| [GET /v1/pokemon/pics/:pokedex_id](#get-pokemon-pic)   | Gets a Pokémon's pic    |
| [POST /v1/pokemon/pics/:pokedex_id](#post-pokemon-pic) | Updates a Pokémon's pic |

---

### GET /v1/pokemon/pics/:pokedex_id {#get-pokemon-pic}

Gets a Pokémon's pic

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id` | string | ✔️ | Pokédex ID.                                                   |
| query  | `face`        | string |   | Face to return. Must be "front" or "back".                    |
| query  | `mirror`      | bool   |   | Enable horizontal mirroring. Must be "true" or "false".       |
| header | `Accept`      | string |   | Type of image to return. Must be "image/png" or "image/jpeg". |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/pokemon/pics/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Accept: image/png
{% end %}

**Body:**

{{ api_response() }}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Header:**

{% api_headers() %}
HTTP/1.1 200 OK
Content-Type: image/png
Content-Disposition: attachment; filename="BULBASAUR.png"
Server: pkmnapi/0.1.0
{% end %}

**Body:**

![BULBASAUR.png](/img/response/BULBASAUR.png)

---

### POST /v1/pokemon/pics/:pokedex_id {#post-pokemon-pic}

Updates a Pokémon's pic

#### Request Parameters

{% api_request_params() %}
| url    | `:pokedex_id`         | string | ✔️ | Pokédex ID.                                                   |
| query  | `face`                | string |   | Face to upload. Must be "front" or "back".                    |
| query  | `method`              | number |   | Encoding method. Must be 1, 2, or 3.                          |
| query  | `prinary`             | number |   | Primary buffer. Must be 0 or 1.                               |
| header | `Content-Type`        | string |   | Type of image to upload. Must be "image/png" or "image/jpeg". |
| header | `X-Patch-Description` | string |   | Description of change.                                        |
| body   | `<raw>`               | binary | ✔️ | Pokémon pic.                                                  |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/pokemon/pics/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: image/png
X-Patch-Description: Update BULBASAUR's pic
{% end %}

**Body:**

![leaf.png](/img/request/leaf.png)

> ⚠️ Images must be 8-bit (or fewer) grayscale and the dimensions must be multiples of 8 pixels

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
