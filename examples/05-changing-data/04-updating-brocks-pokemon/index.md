## Updating BROCK's Pokémon

Now we'll update BROCK's Pokémon or, more aptly, his party of Pokémon.

First, let's inspect BROCK's current party by making a request to the `/v1/trainer/parties/:trainer_id` endpoint:

```bash
curl \
    -H 'Authorization: Bearer <access_token>' \
    https://api.pkmnapi.com/v1/trainer/parties/34
```

Yielding:

```json
{
    "data": {
        "id": "34",
        "type": "trainer_parties",
        "attributes": {
            "parties": [
                {
                    "party": [
                        {
                            "level": 12,
                            "pokemon": {
                                "id": "74",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "GEODUDE"
                                },
                                "links": {
                                    "self": "https://api.pkmnapi.com/v1/pokemon/names/74"
                                }
                            }
                        },
                        {
                            "level": 14,
                            "pokemon": {
                                "id": "95",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "ONIX"
                                },
                                "links": {
                                    "self": "https://api.pkmnapi.com/v1/pokemon/names/95"
                                }
                            }
                        }
                    ]
                }
            ]
        },
        "links": {
            "self": "https://api.pkmnapi.com/v1/trainer/parties/34"
        }
    },
    "links": {
        "self": "https://api.pkmnapi.com/v1/trainer/parties/34"
    }
}
```

> Note: Trainers may have more than one party for different encounters (because trainers are re-used in different locations). For example, the YOUNGSTER trainer has 13 parties (some of which are unused). Gym leaders (with the exception of Giovanni) only have 1 party because they are only battled once.

Now let's fill his party with a bunch of level 5 Geodude!

The game stores parties in 1 of 2 ways:

1) All Pokémon are the same level: all that has to be stored is the level of all Pokémon and their Pokédex IDs
2) All Pokémon are not the same level: each Pokémon is stored with their level and their Pokédex ID

Because the first way takes up less space, we can actually fill BROCK's party with **4** Geodude of the same level, where before he only had room for a Geodude and Onix of different levels.

Let's do so by sending a `POST` request to the `/v1/trainer/parties/:trainer_id` endpoint:

```bash
curl \
    -X POST \
    -H 'Authorization: Bearer <access_token>' \
    -H "X-Patch-Description: Nerf BROCK's party" \
    -H 'Content-Type: application/json' \
    -d '{
    "data": {
        "type": "trainer_parties",
        "attributes": {
            "parties": [
                {
                    "party": [
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74"
                            }
                        }
                    ]
                }
            ]
        }
    }
}' \
    https://api.pkmnapi.com/v1/trainer/parties/34
```

As always, we should verify that our change stuck:

```bash
curl \
    -H 'Authorization: Bearer <access_token>' \
    https://api.pkmnapi.com/v1/trainer/parties/34
```

Yielding:

```json
{
    "data": {
        "id": "34",
        "type": "trainer_parties",
        "attributes": {
            "parties": [
                {
                    "party": [
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "GEODUDE"
                                },
                                "links": {
                                    "self": "https://api.pkmnapi.com/v1/pokemon/names/74"
                                }
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "GEODUDE"
                                },
                                "links": {
                                    "self": "https://api.pkmnapi.com/v1/pokemon/names/74"
                                }
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "GEODUDE"
                                },
                                "links": {
                                    "self": "https://api.pkmnapi.com/v1/pokemon/names/74"
                                }
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "74",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "GEODUDE"
                                },
                                "links": {
                                    "self": "https://api.pkmnapi.com/v1/pokemon/names/74"
                                }
                            }
                        }
                    ]
                }
            ]
        },
        "links": {
            "self": "https://api.pkmnapi.com/v1/trainer/parties/34"
        }
    },
    "links": {
        "self": "https://api.pkmnapi.com/v1/trainer/parties/34"
    }
}
```

That should make (B)ROCK an easier challenge!

![Brock Nerf](../../img/brock-nerf.png)

[[Up]](../index.md) - [[Prev - Updating BROCK's Image]](../03-updating-brocks-image/index.md)
