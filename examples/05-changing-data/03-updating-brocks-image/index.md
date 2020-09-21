## Updating BROCK's Image

Remembering that BROCK's trainer ID from the previous step is 34, we can verify that the image matches as well by sending a request to `/v1/trainer/pics/:trainer_id`:

```bash
curl \
    -H 'Authorization: Bearer <access_token>' \
    -J -O \
    https://api.pkmnapi.com/v1/trainer/pics/34
```

This should download a PNG named "ROCK .png" (because we changed BROCK's name earlier)

Now let's replace that image with one of our own, an image of a rock:

![Rock](../../img/rock.png)

We have sized this image to be the same size as BROCK's (56x56). The image is also 2-bit grayscale and contains only the allowed color palette:

* Black (#000000)
* Dark Grey (#555555)
* Light Grey (#AAAAAA)
* White (#FFFFFF)

To replace a trainer's pic, send a `POST` request to the `/v1/trainer/pics/:trainer_id` endpoint:

```bash
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H 'X-Patch-Description: Make BROCK look like a rock' \
    -H 'Content-Type: image/png' \
    --data-binary @img/rock.png \
    https://api.pkmnapi.com/v1/trainer/pics/34
```

Should be a success. Let's check again to make sure:

```bash
curl \
    -H 'Authorization: Bearer <access_token>' \
    -J -O \
    https://api.pkmnapi.com/v1/trainer/pics/34
```

This, too, rocks!

Behold, the final result:

![Brock Rock](../../img/brock-rock.png)

[[Up]](../index.md) - [[Prev - Updating BROCK's Name]](../02-updating-brocks-name/index.md) - [[Next - Updating BROCK's Pokémon]](../04-updating-brocks-pokemon/index.md)
