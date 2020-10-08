+++
title = "Endpoints"
weight = 4
sort_by = "weight"
+++

### [Access Tokens](@/endpoints/access_tokens.md)

| Endpoint                                                                 | Description                |
|--------------------------------------------------------------------------|----------------------------|
| [POST /v1/access_tokens](@/endpoints/access_tokens.md#post-access-token) | Creates a new access token |

### [HMs](@/endpoints/hms.md)

| Endpoint                                          | Description  |
|---------------------------------------------------|--------------|
| [GET /v1/hms/:hm_id](@/endpoints/hms.md#get-hm)   | Gets a HM    |
| [POST /v1/hms/:hm_id](@/endpoints/hms.md#post-hm) | Updates a HM |

### [Imgs](@/endpoints/imgs.md)

| Endpoint                                                          | Description           |
|-------------------------------------------------------------------|-----------------------|
| [GET /v1/imgs/pokemon_logo](@/endpoints/imgs.md#get-pokemon-logo) | Gets the Pokémon logo |
| [GET /v1/imgs/town_map](@/endpoints/imgs.md#get-town-map)         | Gets the town map     |

### [Item Names](@/endpoints/item_names.md)

| Endpoint                                                                  | Description            |
|---------------------------------------------------------------------------|------------------------|
| [GET /v1/items/names/:item_id](@/endpoints/item_names.md#get-item-name)   | Gets an item's name    |
| [POST /v1/items/names/:item_id](@/endpoints/item_names.md#post-item-name) | Updates an item's name |

### [Map Pics](@/endpoints/map_pics.md)

| Endpoint                                                         | Description    |
|------------------------------------------------------------------|----------------|
| [GET /v1/maps/pics/:map_id](@/endpoints/map_pics.md#get-map-pic) | Gets a map pic |

### [Map Pokémon](@/endpoints/map_pokemon.md)

| Endpoint                                                                     | Description             |
|------------------------------------------------------------------------------|-------------------------|
| [GET /v1/maps/pokemon/:map_id](@/endpoints/map_pokemon.md#get-map-pokemon)   | Gets a map's Pokémon    |
| [POST /v1/maps/pokemon/:map_id](@/endpoints/map_pokemon.md#post-map-pokemon) | Updates a map's Pokémon |

### [Move Names](@/endpoints/move_names.md)

| Endpoint                                                                  | Description         |
|---------------------------------------------------------------------------|---------------------|
| [GET /v1/moves/names/:move_id](@/endpoints/move_names.md#get-move-name)   | Gets a move name    |
| [POST /v1/moves/names/:move_id](@/endpoints/move_names.md#post-move-name) | Updates a move name |

### [Pokédex Entries](@/endpoints/pokedex_entries.md)

| Endpoint                                                                                  | Description             |
|-------------------------------------------------------------------------------------------|-------------------------|
| [GET /v1/pokedex/entries/:pokedex_id](@/endpoints/pokedex_entries.md#get-pokedex-entry)   | Gets a Pokédex entry    |
| [POST /v1/pokedex/entries/:pokedex_id](@/endpoints/pokedex_entries.md#post-pokedex-entry) | Updates a Pokédex entry |

### [Pokédex Texts](@/endpoints/pokedex_texts.md)

| Endpoint                                                                             | Description            |
|--------------------------------------------------------------------------------------|------------------------|
| [GET /v1/pokedex/texts/:pokedex_id](@/endpoints/pokedex_texts.md#get-pokedex-text)   | Gets a Pokédex text    |
| [POST /v1/pokedex/texts/:pokedex_id](@/endpoints/pokedex_texts.md#post-pokedex-text) | Updates a Pokédex text |

### [Pokémon Cries](@/endpoints/pokemon_cries.md)

| Endpoint                                                                            | Description             |
|-------------------------------------------------------------------------------------|-------------------------|
| [GET /v1/pokemon/cries/:pokedex_id](@/endpoints/pokemon_cries.md#get-pokemon-cry)   | Gets a Pokémon's cry    |
| [POST /v1/pokemon/cries/:pokedex_id](@/endpoints/pokemon_cries.md#post-pokemon-cry) | Updates a Pokémon's cry |

### [Pokémon Evolutions](@/endpoints/pokemon_evolutions.md)

| Endpoint                                                                                             | Description                    |
|------------------------------------------------------------------------------------------------------|--------------------------------|
| [GET /v1/pokemon/evolutions/:pokedex_id](@/endpoints/pokemon_evolutions.md#get-pokemon-evolutions)   | Gets a Pokémon's evolutions    |
| [POST /v1/pokemon/evolutions/:pokedex_id](@/endpoints/pokemon_evolutions.md#post-pokemon-evolutions) | Updates a Pokémon's evolutions |

### [Pokémon Names](@/endpoints/pokemon_names.md)

| Endpoint                                                                             | Description              |
|--------------------------------------------------------------------------------------|--------------------------|
| [GET /v1/pokemon/names/:pokedex_id](@/endpoints/pokemon_names.md#get-pokemon-name)   | Gets a Pokémon's name    |
| [POST /v1/pokemon/names/:pokedex_id](@/endpoints/pokemon_names.md#post-pokemon-name) | Updates a Pokémon's name |

### [Pokémon Pics](@/endpoints/pokemon_pics.md)

| Endpoint                                                                          | Description             |
|-----------------------------------------------------------------------------------|-------------------------|
| [GET /v1/pokemon/pics/:pokedex_id](@/endpoints/pokemon_pics.md#get-pokemon-pic)   | Gets a Pokémon's pic    |
| [POST /v1/pokemon/pics/:pokedex_id](@/endpoints/pokemon_pics.md#post-pokemon-pic) | Updates a Pokémon's pic |

### [Pokémon Stats](@/endpoints/pokemon_stats.md)

| Endpoint                                                                              | Description               |
|---------------------------------------------------------------------------------------|---------------------------|
| [GET /v1/pokemon/stats/:pokedex_id](@/endpoints/pokemon_stats.md#get-pokemon-stats)   | Gets a Pokémon's stats    |
| [POST /v1/pokemon/stats/:pokedex_id](@/endpoints/pokemon_stats.md#post-pokemon-stats) | Updates a Pokémon's stats |

### [ROM Patches](@/endpoints/rom_patches.md)

| Endpoint                                                                         | Description                |
|----------------------------------------------------------------------------------|----------------------------|
| [GET /v1/roms/patches](@/endpoints/rom_patches.md#get-rom-patches)               | Gets a list of ROM patches |
| [GET /v1/roms/patches/:patch_id](@/endpoints/rom_patches.md#get-rom-patch)       | Gets a ROM patch           |
| [DELETE /v1/roms/patches/:patch_id](@/endpoints/rom_patches.md#delete-rom-patch) | Deletes a ROM patch        |

### [ROMs](@/endpoints/roms.md)

| Endpoint                                          | Description  |
|---------------------------------------------------|--------------|
| [POST /v1/roms](@/endpoints/roms.md#post-rom)     | Upload a ROM |
| [GET /v1/roms](@/endpoints/roms.md#get-rom)       | Get ROM      |
| [DELETE /v1/roms](@/endpoints/roms.md#delete-rom) | Delete ROM   |

### [SAV Player Names](@/endpoints/sav_player_names.md)

| Endpoint                                                                           | Description               |
|------------------------------------------------------------------------------------|---------------------------|
| [GET /v1/savs/player_names](@/endpoints/sav_player_names.md#get-sav-player-name)   | Gets saved player name    |
| [POST /v1/savs/player_names](@/endpoints/sav_player_names.md#post-sav-player-name) | Updates saved player name |

### [SAVs](@/endpoints/savs.md)

| Endpoint                                          | Description  |
|---------------------------------------------------|--------------|
| [POST /v1/savs](@/endpoints/savs.md#post-sav)     | Upload a SAV |
| [GET /v1/savs](@/endpoints/savs.md#get-sav)       | Get SAV      |
| [DELETE /v1/savs](@/endpoints/savs.md#delete-sav) | Delete SAV   |

### [Status](@/endpoints/status.md)

| Endpoint                                        | Description    |
|-------------------------------------------------|----------------|
| [GET /status](@/endpoints/status.md#get-status) | Get API status |

### [TM Moves](@/endpoints/tm_moves.md)

| Endpoint                                                          | Description         |
|-------------------------------------------------------------------|---------------------|
| [GET /v1/tms/moves/:tm_id](@/endpoints/tm_moves.md#get-tm-move)   | Gets a TM's move    |
| [POST /v1/tms/moves/:tm_id](@/endpoints/tm_moves.md#post-tm-move) | Updates a TM's move |

### [TM Prices](@/endpoints/tm_prices.md)

| Endpoint                                                             | Description          |
|----------------------------------------------------------------------|----------------------|
| [GET /v1/tms/prices/:tm_id](@/endpoints/tm_prices.md#get-tm-price)   | Gets a TM's price    |
| [POST /v1/tms/prices/:tm_id](@/endpoints/tm_prices.md#post-tm-price) | Updates a TM's price |

### [Trainer Names](@/endpoints/trainer_names.md)

| Endpoint                                                                              | Description              |
|---------------------------------------------------------------------------------------|--------------------------|
| [GET /v1/trainers/names/:trainer_id](@/endpoints/trainer_names.md#get-trainer-name)   | Gets a trainer's name    |
| [POST /v1/trainers/names/:trainer_id](@/endpoints/trainer_names.md#post-trainer-name) | Updates a trainer's name |

### [Trainer Parties](@/endpoints/trainer_parties.md)

| Endpoint                                                                                     | Description                 |
|----------------------------------------------------------------------------------------------|-----------------------------|
| [GET /v1/trainers/parties/:trainer_id](@/endpoints/trainer_parties.md#get-trainer-parties)   | Gets a trainer's parties    |
| [POST /v1/trainers/parties/:trainer_id](@/endpoints/trainer_parties.md#post-trainer-parties) | Updates a trainer's parties |

### [Trainer Pics](@/endpoints/trainer_pics.md)

| Endpoint                                                                           | Description             |
|------------------------------------------------------------------------------------|-------------------------|
| [GET /v1/trainers/pics/:trainer_id](@/endpoints/trainer_pics.md#get-trainer-pic)   | Gets a trainer's pic    |
| [POST /v1/trainers/pics/:trainer_id](@/endpoints/trainer_pics.md#post-trainer-pic) | Updates a trainer's pic |

### [Type Effects](@/endpoints/type_effects.md)

| Endpoint                                                                               | Description           |
|----------------------------------------------------------------------------------------|-----------------------|
| [GET /v1/types/effects/:type_effect_id](@/endpoints/type_effects.md#get-type-effect)   | Gets a type effect    |
| [POST /v1/types/effects/:type_effect_id](@/endpoints/type_effects.md#post-type-effect) | Updates a type effect |

### [Type Names](@/endpoints/type_names.md)

| Endpoint                                                                  | Description         |
|---------------------------------------------------------------------------|---------------------|
| [GET /v1/types/names/:type_id](@/endpoints/type_names.md#get-type-name)   | Gets a type name    |
| [POST /v1/types/names/:type_id](@/endpoints/type_names.md#post-type-name) | Updates a type name |
