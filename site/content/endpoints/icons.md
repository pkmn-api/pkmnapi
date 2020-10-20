+++
title = "Icons"
weight = 1
+++

| Endpoint                            | Description  |
|-------------------------------------|--------------|
| [GET /v1/icons/:icon_id](#get-icon) | Gets an icon |

---

### GET /v1/icons/:icon_id {#get-icon}

Gets an icon

#### Request Parameters

{% api_request_params() %}
| url | `:icon_id` | string | ✔️ | Icon ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/icons/0
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_response() }}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Header:**

{% api_headers() %}
HTTP/1.1 200 OK
Content-Type: image/gif
Content-Disposition: attachment; filename="icon-0.gif"
Server: pkmnapi/0.1.0
{% end %}

**Body:**

![icon-0.gif](/img/response/icon-0.gif)
