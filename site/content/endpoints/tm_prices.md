+++
title = "TM Prices"
weight = 22
+++

| Endpoint                                     | Description          |
|----------------------------------------------|----------------------|
| [GET /v1/tms/prices/:tm_id](#get-tm-price)   | Gets a TM's price    |
| [POST /v1/tms/prices/:tm_id](#post-tm-price) | Updates a TM's price |

---

### GET /v1/tms/prices/:tm_id {#get-tm-price}

Gets a TM's price

#### Request Parameters

{% api_request_params() %}
| url | `:tm_id` | string | ✔️ | TM ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/tms/prices/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                  | object |                                        |
| `data.id`               | string | TM ID. (identical to `:tm_id`)         |
| `data.type`             | string | Type of resource. Must be "tm_prices". |
| `data.attributes`       | object |                                        |
| `data.attributes.price` | number | TM price.                              |
| `data.links`            | object |                                        |
| `data.links.self`       | string | Link to current resource.              |
| `links`                 | object |                                        |
| `links.self`            | string | Link to current resource.              |
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
        "type": "tm_prices",
        "attributes": {
            "price": 3000
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/tms/prices/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/tms/prices/1"
    }
}
{% end %}

---

### POST /v1/tms/prices/:tm_id {#post-tm-price}

Updates a TM's price

#### Request Parameters

{% api_request_params() %}
| url    | `:tm_id`                | string | ✔️ | TM ID.                             |
| header | `X-Patch-Description`   | string |   | Description of change.             |
| body   | `data`                  | object | ✔️ |                                    |
| body   | `data.type`             | string | ✔️ | Type of data. Must be "tm_prices". |
| body   | `data.attributes`       | object | ✔️ |                                    |
| body   | `data.attributes.price` | number | ✔️ | TM price.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/tms/prices/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update TM01 price
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "tm_prices",
        "attributes": {
            "price": 9000
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
