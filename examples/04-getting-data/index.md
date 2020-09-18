# Getting Data

Once a ROM has been uploaded, the data is now queryable.

To get a Pokémon's name, send a request to the `/v1/pokemon/names/:pokedex_id` endpoint:

```bash
curl \
    -H 'Authorization: Bearer <access_token>' \
    https://api.pkmnapi.com/v1/pokemon/names/1
```

Here is the result:

```json
{
    "data": {
        "id": "1",
        "type": "pokemon_names",
        "attributes": {
            "name": "BULBASAUR"
        },
        "links": {
            "self": "https://api.pkmnapi.com/v1/pokemon/names/1"
        }
    },
    "links": {
        "self": "https://api.pkmnapi.com/v1/pokemon/names/1"
    }
}
```

[[Up]](../index.md) - [[Prev - Uploading A ROM]](../03-uploading-a-rom/index.md) - [[Next - Changing Data]](../05-changing-data/index.md)
