+++
title = "Uploading a ROM"
weight = 3
+++

In order to use a Pokémon Gen 1 ROM as your data source, one must be provided.

`pkmnapi` has been tested with Pokémon Red and Pokémon Blue. Other versions may not work as expected.

Once a ROM is provided, it *CANNOT* be downloaded again. That would be tantamount to ROM distribution and would very much incur the wrath of Nintendo and/or Game Freak.

Instead, patches (AKA _just_ the changes to the ROM) may be downloaded and, since you have proven that you have the original ROM, you should be able patch your ROM manually. See the section on [Downloading A Patch](@/changing-data/downloading-a-patch.md) for more information.

To upload a ROM, send a request to the <code>[POST /v1/roms](@/endpoints/roms.md#post-rom)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    --data-binary @path/to/rom.gb \
    {{API_DOMAIN}}/v1/roms
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('POST', '{{API_DOMAIN}}/v1/roms', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(rom_data);
{% end %}

{{ api_snippet_end() }}

Only one ROM may be uploaded at a time, so repeating the previous request will result in an error:

{% api_response() %}
{
    "data": "error_roms_rom_exists",
    "type": "errors",
    "attributes": {
        "message": "ROM already exists
    }
}
{% end %}

To delete an existing ROM, send a request to the <code>[DELETE /v1/roms](@/endpoints/roms.md#delete-rom)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -X DELETE \
    -H 'Authorization: Bearer <access_token>' \
    -H 'If-Match: w/"abcdef0123456789abcdef0123456789"' \
    {{API_DOMAIN}}/v1/roms
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('DELETE', '{{API_DOMAIN}}/v1/roms');
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.setRequestHeader('If-Match', 'w/"abcdef0123456789abcdef0123456789"');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}
