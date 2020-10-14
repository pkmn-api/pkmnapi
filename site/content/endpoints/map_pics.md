+++
title = "Map Pics"
weight = 6
+++

| Endpoint                                  | Description    |
|-------------------------------------------|----------------|
| [GET /v1/maps/pics/:map_id](#get-map-pic) | Gets a map pic |

---

### GET /v1/maps/pics/:map_id {#get-map-pic}

Gets a map pic

#### Request Parameters

{% api_request_params() %}
| url    | `:map_id` | string | ✔️ | Map ID.                                                       |
| header | `Accept`  | string |   | Type of image to return. Must be "image/png" or "image/jpeg". |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/maps/pics/0
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
Content-Disposition: attachment; filename="map-0.png"
Server: pkmnapi/0.1.0
{% end %}

**Body:**

![map-0.png](/img/response/map-0.png)
