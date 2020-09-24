+++
title = "Updating BROCK's Image"
weight = 3
+++

Remembering that BROCK's trainer ID from the previous step is 34, we can verify that the image matches as well by sending a request to the <code>[GET /v1/trainers/pics/:trainer_id](@/endpoints/trainer_pics.md#get-trainer-pic)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    -J -O \
    {{API_DOMAIN}}/v1/trainers/pics/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/trainers/pics/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}

This should download a PNG named "ROCK .png" (because we changed BROCK's name earlier)

Now let's replace that image with one of our own, an image of a rock:

![Rock](/img/request/rock.png)

We have sized this image to be the same size as BROCK's (56x56). The image is also 2-bit grayscale and contains only the allowed color palette:

* Black (#000000)
* Dark Grey (#555555)
* Light Grey (#AAAAAA)
* White (#FFFFFF)

To replace a trainer's pic, send a request to the <code>[POST /v1/trainers/pics/:trainer_id](@/endpoints/trainer_pics.md#post-trainer-pic)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H 'X-Patch-Description: Make BROCK look like a rock' \
    -H 'Content-Type: image/png' \
    --data-binary @path/to/rock.png \
    {{API_DOMAIN}}/v1/trainers/pics/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/trainers/pics/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.setRequestHeader('X-Patch-Description', 'Make BROCK look like a rock');
xhr.setRequestHeader('Content-Type', 'image/png');
xhr.send(png_data);
{% end %}

{{ api_snippet_end() }}

Should be a success. Let's check again to make sure:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    -J -O \
    {{API_DOMAIN}}/v1/trainers/pics/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/trainers/pics/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}

This, too, rocks!

Behold, the final result:

![Brock Rock](/img/screenshot/brock-rock.png)
