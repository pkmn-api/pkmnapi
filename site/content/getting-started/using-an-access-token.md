+++
title = "Using an Access Token"
weight = 2
+++

All requests must be authenticated with an access token.

After receiving an access token in your email, you must include the token in every request.

`pkmnapi` uses Bearer authentication, so the following header should be set with every request:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    ...
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.setRequestHeader('Authorization', 'Bearer <access_token>');

...
{% end %}

{{ api_snippet_end() }}
