+++
title = "Imgs"
weight = 4
+++

| Endpoint                                         | Description              |
|--------------------------------------------------|--------------------------|
| [GET /v1/imgs/pokemon_logo](#get-pokemon-logo)   | Gets the Pokémon logo    |
| [POST /v1/imgs/pokemon_logo](#post-pokemon-logo) | Updates the Pokémon logo |
| [GET /v1/imgs/town_map](#get-town-map)           | Gets the town map        |

---

### GET /v1/imgs/pokemon_logo {#get-pokemon-logo}

Gets the Pokémon logo

#### Request Parameters

{% api_request_params() %}
| header | `Accept` | string | | Type of image to return. Must be "image/png" or "image/jpeg". |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/imgs/pokemon_logo
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
Content-Disposition: attachment; filename="pokemon_logo.png"
Server: pkmnapi/0.1.0
{% end %}

**Body:**

![pokemon_logo.png](/img/response/pokemon_logo.png)

---

### POST /v1/imgs/pokemon_logo {#post-pokemon-logo}

Updates the Pokémon logo

#### Request Parameters

{% api_request_params() %}
| header | `Content-Type`        | string |   | Type of image to upload. Must be "image/png" or "image/jpeg". |
| header | `X-Patch-Description` | string |   | Description of change.                                        |
| body   | `<raw>`               | binary | ✔️ | Pokémon logo.                                                 |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/imgs/pokemon_logo
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: image/png
X-Patch-Description: Update Pokémon to Digimon
{% end %}

**Body:**

![leaf.png](/img/request/digimon_logo.png)

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

---

### GET /v1/imgs/town_map {#get-town-map}

Gets the town map

#### Request Parameters

{% api_request_params() %}
| header | `Accept` | string | | Type of image to return. Must be "image/png" or "image/jpeg". |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/imgs/town_map
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
Content-Disposition: attachment; filename="town_map.png"
Server: pkmnapi/0.1.0
{% end %}

**Body:**

![towm_map.png](/img/response/town_map.png)
