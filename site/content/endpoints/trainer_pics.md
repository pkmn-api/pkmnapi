+++
title = "Trainer Pics"
weight = 28
+++

| Endpoint                                               | Description             |
|--------------------------------------------------------|-------------------------|
| [GET /v1/trainers/pics/:trainer_id](#get-trainer-pic)  | Gets a trainer's pic    |
| [GET /v1/trainers/pics/:trainer_id](#post-trainer-pic) | Updates a trainer's pic |

---

### GET /v1/trainers/pics/:trainer_id {#get-trainer-pic}

Gets a trainer's pic

#### Request Parameters

{% api_request_params() %}
| url    | `:trainer_id` | string | ✔️ | Trainer ID.                                                   |
| query  | `mirror`      | bool   |   | Enable horizontal mirroring. Must be "true" or "false".       |
| header | `Accept`      | string |   | Type of image to return. Must be "image/png" or "image/jpeg". |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/trainers/pics/34
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
Content-Disposition: attachment; filename="BROCK.png"
Server: pkmnapi/0.1.0
{% end %}

**Body:**

![BROCK.png](/img/response/BROCK.png)

---

### POST /v1/trainers/pics/:trainer_id {#post-trainer-pic}

Updates a trainer's pic

#### Request Parameters

{% api_request_params() %}
| url    | `:trainer_id`         | string | ✔️ | Trainer ID.                                                   |
| query  | `method`              | number |   | Encoding method. Must be 1, 2, or 3.                          |
| query  | `prinary`             | number |   | Primary buffer. Must be 0 or 1.                               |
| header | `Content-Type`        | string |   | Type of image to upload. Must be "image/png" or "image/jpeg". |
| header | `X-Patch-Description` | string |   | Description of change.                                        |
| body   | `<raw>`               | binary | ✔️ | Trainer pic.                                                  |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/trainers/pics/34
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: image/png
X-Patch-Description: Update BROCK's pic
{% end %}

**Body:**

![rock.png](/img/request/rock.png)

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
