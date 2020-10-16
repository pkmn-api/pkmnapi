+++
title = "Updating BROCK's Name"
weight = 2
+++

To update a trainer's name, we must first find out the ID of the trainer. This can be done by accessing the <code>[GET /v1/trainers/names/:trainer_id](@/endpoints/trainer_names.md#get-trainer-name)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    {{API_DOMAIN}}/v1/trainers/names/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/trainers/names/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}

Now we know that BROCK has a trainer ID of 34.

To update BROCK's name, we must send a request to the <code>[POST /v1/trainers/names/:trainer_id](@/endpoints/trainer_names.md#post-trainer-name)</code> endpoint with the `:trainer_id` of 34:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H 'X-Patch-Description: Change BROCK to ROCK' \
    -H 'Content-Type: application/json' \
    -d '{
    "data": {
        "type": "trainer_names",
        "attributes": {
            "name": "ROCK"
        }
    }
}' \
    {{API_DOMAIN}}/v1/trainers/names/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('POST', '{{API_DOMAIN}}/v1/trainers/names/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.setRequestHeader('X-Patch-Description', 'Change BROCK to ROCK');
xhr.setRequestHeader('Content-Type', 'application/json');
xhr.send(JSON.stringify({
    data: {
        type: 'trainer_names',
        attributes: {
            name: 'ROCK'
        }
    }
}));
{% end %}

{{ api_snippet_end() }}

You should receive the following error:

{% api_response() %}
{
    "data": {
        "id": "error_not_found",
        "type": "errors",
        "attributes": {
            "message": "Trainer name length mismatch: should be exactly 5 characters, found 4"
        }
    }
}
{% end %}

Because of how some data is stored, the updated data must be either 1) exactly the same length, or 2) no bigger than the data it is replacing.

To circumvent this constraint, let's instead update BROCK's name to "ROCK " (with a trailing space) to get the exact same length:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H 'X-Patch-Description: Change BROCK to ROCK' \
    -H 'Content-Type: application/json' \
    -d '{
    "data": {
        "type": "trainer_names",
        "attributes": {
            "name": "ROCK"
        }
    }
}' \
    {{API_DOMAIN}}/v1/trainers/names/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('POST', '{{API_DOMAIN}}/v1/trainers/names/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.setRequestHeader('X-Patch-Description', 'Change BROCK to ROCK');
xhr.setRequestHeader('Content-Type', 'application/json');
xhr.send(JSON.stringify({
    data: {
        type: 'trainer_names',
        attributes: {
            name: 'ROCK '
        }
    }
}));
{% end %}

Success!

Let's see if our change is reflected by requesting BROCK's (now ROCK's) name again:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    {{API_DOMAIN}}/v1/trainers/names/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/trainers/names/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}

Should return:

{% api_response() %}
{
    "data": {
        "id": "34",
        "type": "trainer_names",
        "attributes": {
            "name": "ROCK "
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/trainers/names/34"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/trainers/names/34"
    }
}
{% end %}

That rocks!
