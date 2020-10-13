+++
title = "SAVs"
weight = 20
+++

| Endpoint                       | Description  |
|--------------------------------|--------------|
| [POST /v1/savs](#post-sav)     | Upload a SAV |
| [GET /v1/savs](#get-sav)       | Get SAV      |
| [DELETE /v1/savs](#delete-sav) | Delete SAV   |

---

### POST /v1/savs {#post-sav}

Upload a SAV

> ⚠️ Only 1 SAV may be uploaded at a time. After a successful upload, subsequent uploads will fail unless the SAV is deleted.

#### Request Parameters

{% api_request_params() %}
| body | `<raw>` | binary | ✔️ | Game Boy SAV. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/savs
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request(raw=true) }}

#### Response Parameters

{% api_response_params() %}
| `data`            | object |                                                |
| `data.id`         | string | SAV ID.                                        |
| `data.type`       | string | Type of resource. Must be "savs".              |
| `data.attributes` | object |                                                |
| `data.links`      | object |                                                |
| `data.links.self` | string | Link to current resource.                      |
| `links`           | object |                                                |
| `links.self`      | string | Link to current resource.                      |
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
        "type": "savs",
        "attributes": {},
        "links": {
            "self": "{{API_DOMAIN}}/v1/savs/1337"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/savs/1337"
    }
}
{% end %}

---

### GET /v1/savs {#get-sav}

Get SAV

> ⚠️ There is no way to download a SAV that has been uploaded.

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/savs
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                  | object |                                                |
| `data.id`               | string | SAV ID.                                        |
| `data.type`             | string | Type of resource. Must be "savs".              |
| `data.attributes`       | object |                                                |
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
        "type": "savs",
        "attributes": {},
        "links": {
            "self": "{{API_DOMAIN}}/v1/savs/1337"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/savs/1337"
    }
}
{% end %}

---

### DELETE /v1/savs {#delete-sav}

Deletes SAV

#### Request Parameters

{% api_request_params() %}
| header | `If-Match` | string | ✔️ | ETag of resource. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
DELETE /v1/savs
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
