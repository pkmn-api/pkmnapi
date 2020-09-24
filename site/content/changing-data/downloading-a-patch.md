+++
title = "Downloading a Patch"
weight = 5
+++

In order to incorporate the previous changes into the actual ROM, you must download a patch file.

> 📄 Patch files are in <a href="http://fileformats.archiveteam.org/wiki/IPS_(binary_patch_format)" target="_blank">IPS format</a>. After downloading, you must apply the IPS patch manually.

Request a patch by sending a request to the <code>[GET /v1/roms/patches](@/endpoints/rom_patches.md#get-rom-patches)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    -H 'Accept: application/patch' \
    {{API_DOMAIN}}/v1/roms/patches
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/roms/patches', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.setRequestHeader('Accept', 'application/patch');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}
