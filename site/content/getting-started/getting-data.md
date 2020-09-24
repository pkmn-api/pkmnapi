+++
title = "Getting Data"
weight = 4
+++

Once a ROM has been uploaded, the data is now queryable.

To get a Pokémon's name, send a request to the <code>[GET /v1/pokemon/names/:pokedex_id](@/endpoints/pokemon_names.md#get-pokemon-name)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    {{API_DOMAIN}}/v1/pokemon/names/1
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/pokemon/names/1', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}

Here is the result:

{% api_response() %}
{
    "data": {
        "id": "1",
        "type": "pokemon_names",
        "attributes": {
            "name": "BULBASAUR"
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/pokemon/names/1"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/pokemon/names/1"
    }
}
{% end %}
