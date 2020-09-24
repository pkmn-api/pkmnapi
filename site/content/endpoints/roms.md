+++
title = "ROMs"
weight = 11
+++

| Endpoint                       | Description  |
|--------------------------------|--------------|
| [POST /v1/roms](#post-rom)     | Upload a ROM |
| [GET /v1/roms](#get-rom)       | Get ROM      |
| [DELETE /v1/roms](#delete-rom) | Delete ROM   |

---

### POST /v1/roms {#post-rom}

Upload a ROM

> ⚠️ Only 1 ROM may be uploaded at a time. After a successful upload, subsequent uploads will fail unless the ROM is deleted.

#### Request Parameters

{% api_request_params() %}
| body | `<raw>` | binary | ✔️ | Game Boy ROM. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/roms
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request(raw=true) }}

#### Response Parameters

{% api_response_params() %}
| `data`                  | object |                                                |
| `data.id`               | string | ROM ID.                                        |
| `data.type`             | string | Type of resource. Must be "roms".              |
| `data.attributes`       | object |                                                |
| `data.attributes.name`  | string | Name in ROM header.                            |
| `data.attributes.hash`  | string | ROM hash. (MD5)                                |
| `data.attributes.valid` | bool   | True if valid Pokémon Gen 1 ROM, false if not. |
| `data.links`            | object |                                                |
| `data.links.self`       | string | Link to current resource.                      |
| `links`                 | object |                                                |
| `links.self`            | string | Link to current resource.                      |
{% end %}

#### Example Response

**Headers:**

{% api_headers() %}
HTTP/1.1 201 Created
Content-Type: application/json
Server: pkmnapi/0.1.0
ETag: w/"abcdef0123456789abcdef0123456789"
{% end %}

**Body:**

{% api_response() %}
{
    "data": {
        "id": "1337",
        "type": "roms",
        "attributes": {
            "name": "POKEMON RED",
            "hash": "3d45c1ee9abd5738df46d2bdda8b57dc",
            "valid": true
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/roms/1337"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/roms/1337"
    }
}
{% end %}

---

### GET /v1/roms {#get-rom}

Get ROM

> ⚠️ There is no way to download a ROM that has been uploaded. See the [Disclaimer](@/disclaimer/_index.md) for the reasons why.

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/roms
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                  | object |                                                |
| `data.id`               | string | ROM ID.                                        |
| `data.type`             | string | Type of resource. Must be "roms".              |
| `data.attributes`       | object |                                                |
| `data.attributes.name`  | string | Name in ROM header.                            |
| `data.attributes.hash`  | string | ROM hash. (MD5)                                |
| `data.attributes.valid` | bool   | True if valid Pokémon Gen 1 ROM, false if not. |
| `data.links`            | object |                                                |
| `data.links.self`       | string | Link to current resource.                      |
| `links`                 | object |                                                |
| `links.self`            | string | Link to current resource.                      |
{% end %}

#### Example Response

**Headers:**

{% api_headers() %}
HTTP/1.1 200 OK
Content-Type: application/json
Server: pkmnapi/0.1.0
ETag: w/"abcdef0123456789abcdef0123456789"
{% end %}

**Body:**

{% api_response() %}
{
    "data": {
        "id": "1337",
        "type": "roms",
        "attributes": {
            "name": "POKEMON RED",
            "hash": "3d45c1ee9abd5738df46d2bdda8b57dc",
            "valid": true
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/roms/1337"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/roms/1337"
    }
}
{% end %}

---

### DELETE /v1/roms {#delete-rom}

Deletes ROM

#### Request Parameters

{% api_request_params() %}
| header | `If-Match` | string | ✔️ | ETag of resource. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
DELETE /v1/roms
Host: {{API_HOST}}
Authorization: Bearer <access_token>
If-Match: w/"abcdef0123456789abcdef0123456789"
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Headers:**

{% api_headers() %}
HTTP/1.1 204 No Content
Content-Type: application/json
Server: pkmnapi/0.1.0
{% end %}

**Body:**

{{ api_response() }}
