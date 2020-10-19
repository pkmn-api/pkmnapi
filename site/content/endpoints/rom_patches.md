+++
title = "ROM Patches"
weight = 22
+++

| Endpoint                                               | Description                |
|--------------------------------------------------------|----------------------------|
| [GET /v1/roms/patches](#get-rom-patches)               | Gets a list of ROM patches |
| [GET /v1/roms/patches/:patch_id](#get-rom-patch)       | Gets a ROM patch           |
| [DELETE /v1/roms/patches/:patch_id](#delete-rom-patch) | Deletes a ROM patch        |

---

### GET /v1/roms/patches {#get-rom-patches}

Gets a list of ROM patches

#### Request Parameters

{% api_request_params() %}
| query  | `checksum` | bool   | | Append checksum patch. Default: true. (Only applies to "application/patch" requests) |
| header | `Accept`   | string | | Format to return. Must be "application/json" or "application/patch".                 |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/roms/patches
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Accept: application/json
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                          | array  |                                          |
| `data[].id`                     | string | Patch ID.                                |
| `data[].type`                   | string | Type of resource. Must be "rom_patches". |
| `data[].attributes`             | object |                                          |
| `data[].attributes.description` | string | Patch description.                       |
| `data[].links`                  | object |                                          |
| `data[].links.self`             | string | Link to individual patch resource.       |
| `links`                         | object |                                          |
| `links.self`                    | string | Link to list resource.                   |
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
    "data": [
        {
            "id": "1337",
            "type": "rom_patches",
            "attributes": {
                "description": "Change X to Y"
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/roms/patches/1337"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/roms/patches"
    }
}
{% end %}

---

### GET /v1/roms/patches/:patch_id {#get-rom-patch}

Gets a ROM patch

#### Request Parameters

{% api_request_params() %}
| url | `:patch_id` | string | ✔️ | Patch ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/roms/patches/1337
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                        | array  |                                          |
| `data.id`                     | string | Patch ID. (identical to `:patch_id`)     |
| `data.type`                   | string | Type of resource. Must be "rom_patches". |
| `data.attributes`             | object |                                          |
| `data.attributes.description` | string | Patch description.                       |
| `data.links`                  | object |                                          |
| `data.links.self`             | string | Link to current resource.                |
| `links`                       | object |                                          |
| `links.self`                  | string | Link to current resource.                |
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
        "type": "rom_patches",
        "attributes": {
            "description": "Change X to Y"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/roms/patches/1337"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/roms/patches/1337"
    }
}
{% end %}

---

### DELETE /v1/roms/patches/:patch_id {#delete-rom-patch}

Deletes a ROM patch

#### Request Parameters

{% api_request_params() %}
| url    | `:patch_id` | string | ✔️ | Patch ID.         |
| header | `If-Match`  | string | ✔️ | ETag of resource. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
DELETE /v1/roms/patches/1337
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
