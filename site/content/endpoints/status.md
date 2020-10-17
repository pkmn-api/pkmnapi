+++
title = "Status"
weight = 25
+++

| Endpoint                   | Description    |
|----------------------------|----------------|
| [GET /status](#get-status) | Get API status |

---

### GET /status {#get-status}

Get API status

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /status
Host: {{API_HOST}}
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{{ api_response_params() }}

#### Example Response

**Headers:**

{% api_headers() %}
HTTP/1.1 200 OK
Content-Type: text/plain
Server: pkmnapi/0.1.0
{% end %}

**Body:**

{% api_response(lang="text") %}
OK
{% end %}
