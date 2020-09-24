+++
title = "Creating an Access Token"
weight = 1
+++

All requests sent to `pkmnapi` _must_ be authenticated using an access token.

To create an access token, send request to <code>[POST /v1/access_tokens](@/endpoints/access_tokens.md#post-access-token)</code> with a valid email address:

> ⚠️ Requests sent to <code>[POST /v1/access_tokens](@/endpoints/access_tokens.md#post-access-token)</code> are the only ones that must be _unauthenticated_

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -X POST \
    -H 'Content-Type: application/json' \
    -d '{
    "data": {
        "type": "access_tokens",
        "attributes": {
            "email_address": "your@email.com"
        }
    }
}' \
    {{API_DOMAIN}}/v1/access_tokens
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('POST', '{{API_DOMAIN}}/v1/access_tokens', true);
xhr.setRequestHeader('Content-Type', 'application/json');
xhr.send(JSON.stringify({
    data: {
        type: 'access_tokens',
        attributes: {
            email_address: 'your@email.com'
        }
    }
}));
{% end %}

An email containing your access token will be sent to the provided email address.

> 📄 To prevent the frivolous generation of access tokens, there is a rate limit of 10 minutes on the <code>[POST /v1/access_tokens](@/endpoints/access_tokens.md#post-access-token)</code> endpoint per email address

> 📄 Want more than 1 access token with the same email address? Consider using a "+" in your email: `your+access+token@email.com`, `your+second+access+token@email.com`, etc
