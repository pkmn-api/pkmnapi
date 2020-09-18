# Uploading a ROM

In order to use a Pokémon Gen 1 ROM as your data source, one must be provided.

Once a ROM is provided, it *CANNOT* be downloaded again. That would be tantamount to ROM distribution and would very much incur the wrath of Nintendo and/or Game Freak.

Instead, patches (AKA _just_ the changes to the ROM) may be downloaded and, since you have proven that you have the original ROM, you may patch your ROM manually. See the section on [Downloading A Patch](./06-downloading-a-patch.md) for more information.

To upload a ROM, send a `POST` request to the `/v1/roms` endpoint:

```bash
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    --data-binary @rom.gb \
    https://api.pkmnapi.com/v1/roms
```

Only one ROM may be uploaded at a time, so repeating the previous request will result in an error:

```json
{
    "data": "error_roms_rom_exists",
    "type": "errors",
    "attributes": {
        "message": "ROM already exists
    }
}
```

To delete an existing ROM, send a `DELETE` request to the `/v1/roms` endopint:

```bash
curl \
    -X DELETE \
    -H 'Authorization: Bearer <access_token>' \
    https://api.pkmnapi.com/v1/roms
```

[[Up]](../index.md) - [[Prev - Using An Access Token]](../02-using-an-access-token/index.md) - [[Next - Getting Data]](../04-getting-data/index.md)
