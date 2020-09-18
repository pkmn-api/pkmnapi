## Updating BROCK's Name

To update a trainer's name, we must first find out the ID of the trainer. This can be done by accessing the `/v1/trainer/names/:trainer_id` endpoint:

```bash
curl \
    -H 'Authorization: Bearer <access_token>' \
    https://api.pkmnapi.com/v1/trainer/names/34
```

Now we know that BROCK has a trainer ID of 34.

To update BROCK's name, we must send a `POST` request to the `/v1/trainer/names/:trainer_id` endpoint with the `:trainer_id` of 34:

```bash
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
    https://api.pkmnapi.com/v1/trainer/names/34
```

You should receive the following error:

```json
{
    "data": {
        "id": "error_trainer_names",
        "type": "errors",
        "attributes": {
            "message": "Trainer name length mismatch: should be exactly 5 characters, found 4"
        }
    }
}
```

Because of how some data is stored, the updated data must be either 1) exactly the same length, or 2) no bigger than the data it is replacing.

To circumvent this constraint, let's instead update BROCK's name to "ROCK " (with a trailing space) to get the exact same length:

```bash
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H 'X-Patch-Description: Change BROCK to ROCK' \
    -H 'Content-Type: application/json' \
    -d '{
    "data": {
        "type": "trainer_names",
        "attributes": {
            "name": "ROCK "
        }
    }
}' \
    https://api.pkmnapi.com/v1/trainer/names/34
```

Success!

Let's see if our change is reflected by requesting BROCK's (now ROCK's) name again:

```bash
curl \
    -H 'Authorization: Bearer <access_token>' \
    https://api.pkmnapi.com/v1/trainer/names/34
```

Should return:

```json
{
    "data": {
        "id": "34",
        "type": "trainer_names",
        "attributes": {
            "name": "ROCK "
        },
        "links": {
            "self": "https://api.pkmnapi.com/v1/trainer/names/34"
        }
    },
    "links": {
        "self": "https://api.pkmnapi.com/v1/trainer/names/34"
    }
}
```

That rocks!

[[Up]](../index.md) - [[Prev - Patching In General]](../01-patching-in-general/index.md) - [[Next - Updating BROCK's Image]](../03-updating-brocks-image/index.md)
