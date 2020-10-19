+++
title = "Trainer Rewards"
weight = 32
+++

| Endpoint                                                      | Description                    |
|---------------------------------------------------------------|--------------------------------|
| [GET /v1/trainers/rewards](#get-trainer-reward-all)           | Gets a list of trainer rewards |
| [GET /v1/trainers/rewards/:trainer_id](#get-trainer-reward)   | Gets a trainer's reward        |
| [POST /v1/trainers/rewards/:trainer_id](#post-trainer-reward) | Updates a trainer's reward     |

---

### GET /v1/trainers/rewards {#get-trainer-reward-all}

Gets a list of trainer rewards

#### Request Parameters

{{ api_request_params() }}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/trainers/rewards
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                     | array  |                                              |
| `data[]`                   | object |                                              |
| `data[].id`                | string | Trainer ID. (identical to `:trainer_id`)     |
| `data[].type`              | string | Type of resource. Must be "trainer_rewards". |
| `data[].attributes`        | object |                                              |
| `data[].attributes.reward` | number | Trainer reward.                              |
| `data[].links`             | object |                                              |
| `data[].links.self`        | string | Link to current resource.                    |
| `links`                    | object |                                              |
| `links.self`               | string | Link to list resource.                       |
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
    "data": [
        {
            "id": "1",
            "type": "trainer_rewards",
            "attributes": {
                "reward": 1500
            },
            "links": {
                "self": "{{API_DOMAIN}}/v1/trainers/rewards/1"
            }
        },
        ...
    ],
    "links": {
        "self": "{{API_DOMAIN}}/v1/trainers/rewards"
    }
}
{% end %}

---

### GET /v1/trainers/rewards/:trainer_id {#get-trainer-reward}

Gets a trainer's reward

#### Request Parameters

{% api_request_params() %}
| url | `:trainer_id` | string | ✔️ | Trainer ID. |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
GET /v1/trainers/rewards/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
{% end %}

**Body:**

{{ api_request() }}

#### Response Parameters

{% api_response_params() %}
| `data`                   | object |                                              |
| `data.id`                | string | Trainer ID. (identical to `:trainer_id`)     |
| `data.type`              | string | Type of resource. Must be "trainer_rewards". |
| `data.attributes`        | object |                                              |
| `data.attributes.reward` | number | Trainer reward.                              |
| `data.links`             | object |                                              |
| `data.links.self`        | string | Link to current resource.                    |
| `links`                  | object |                                              |
| `links.self`             | string | Link to current resource.                    |
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
        "type": "trainer_rewards",
        "attributes": {
            "reward": 1500
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/trainers/rewards/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/trainers/rewards/1"
    }
}
{% end %}

---

### POST /v1/trainers/rewards/:trainer_id {#post-trainer-reward}

Updates a trainer's reward

#### Request Parameters

{% api_request_params() %}
| url    | `:trainer_id`            | string | ✔️ | Trainer ID.                              |
| header | `X-Patch-Description`    | string |   | Description of change.                   |
| body   | `data`                   | object | ✔️ |                                          |
| body   | `data.type`              | string | ✔️ | Type of data. Must be "trainer_rewards". |
| body   | `data.attributes`        | object | ✔️ |                                          |
| body   | `data.attributes.reward` | number | ✔️ | Trainer reward.                          |
{% end %}

#### Example Request

**Header:**

{% api_headers() %}
POST /v1/trainers/rewards/1
Host: {{API_HOST}}
Authorization: Bearer <access_token>
Content-Type: application/json
X-Patch-Description: Update YOUNGSTER's reward
{% end %}

**Body:**

{% api_request() %}
{
    "data": {
        "type": "trainer_rewards",
        "attributes": {
            "reward": 1337
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
