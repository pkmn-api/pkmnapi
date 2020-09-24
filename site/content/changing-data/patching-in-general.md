+++
title = "Patching in General"
weight = 1
+++

Every change to the ROM generates a patch that contains information for how the source ROM should be updated to reflect the change.

Multiple patches may update the same area of the ROM with different data. As such, every change to the data can be identified by adding a description to the request via the `X-Patch-Description` header:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H 'X-Patch-Description: Brief description of data updated' \
    -H 'Content-Type: application/json' \
    ...
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('POST', '...', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.setRequestHeader('X-Patch-Description', 'Brief description of data updated');
xhr.setRequestHeader('Content-Type', 'application/json');
{% end %}

{{ api_snippet_end() }}
