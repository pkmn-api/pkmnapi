+++
title = "Updating BROCK's Pokémon"
weight = 4
+++

Now we'll update BROCK's Pokémon or, more aptly, his party of Pokémon.

First, let's inspect BROCK's current party by making a request to the <code>[GET /v1/trainers/parties/:trainer_id](@/endpoints/trainer_parties.md#get-trainer-parties)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    {{API_DOMAIN}}/v1/trainers/parties/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/trainers/parties/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}

Yielding:

{% api_response() %}
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
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/74"
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
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/95"
                                }
                            }
                        }
                    ]
                }
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/trainers/parties/34"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/trainers/parties/34"
    }
}
{% end %}

> 📄 Trainers may have more than one party for different encounters (because trainers are re-used in different locations). For example, the YOUNGSTER trainer has 13 parties (some of which are unused). Gym leaders (with the exception of Giovanni) only have 1 party because they are only battled once.

Now let's fill his party with a bunch of level 5 Geodude!

The game stores parties in 1 of 2 ways:

1) All Pokémon are the same level: all that has to be stored is the level of all Pokémon and their Pokédex IDs
2) All Pokémon are not the same level: each Pokémon is stored with their level and their Pokédex ID

Because the first way takes up less space, we can actually fill BROCK's party with **4** Geodude of the same level, where before he only had room for a Geodude and Onix of different levels.

Let's do so by sending a request to the <code>[POST /v1/trainers/parties/:trainer_id](@/endpoints/trainer_parties.md#post-trainer-parties)</code> endpoint:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
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
    {{API_DOMAIN}}/v1/trainers/parties/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('POST', '{{API_DOMAIN}}/v1/trainers/parties/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.setRequestHeader('X-Patch-Description', 'Nerf BROCK\'s party');
xhr.setRequestHeader('Content-Type', 'application/json');
xhr.send(JSON.stringify({
    data: {
        type: 'trainer_parties',
        attributes: {
            parties: [
                {
                    party: [
                        {
                            level: 5,
                            pokemon: {
                                id: '74'
                            }
                        },
                        {
                            level: 5,
                            pokemon: {
                                id: '74'
                            }
                        },
                        {
                            level: 5,
                            pokemon: {
                                id: '74'
                            }
                        },
                        {
                            level: 5,
                            pokemon: {
                                id: '74'
                            }
                        }
                    ]
                }
            ]
        }
    }
}));
{% end %}

{{ api_snippet_end() }}

As always, we should verify that our change stuck:

{{ api_snippet_start(langs=["bash", "js"]) }}

{% api_request(lang="bash") %}
curl \
    -H 'Authorization: Bearer <access_token>' \
    {{API_DOMAIN}}/v1/trainers/parties/34
{% end %}

{% api_request(lang="js", hidden=true) %}
const xhr = new XMLHttpRequest();

xhr.addEventListener('load', () => {
    console.log(xhr.response);
});

xhr.open('GET', '{{API_DOMAIN}}/v1/trainers/parties/34', true);
xhr.setRequestHeader('Authorization', 'Bearer <access_token>');
xhr.send(null);
{% end %}

{{ api_snippet_end() }}

Yielding:

{% api_response() %}
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
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/74"
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
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/74"
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
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/74"
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
                                    "self": "{{API_DOMAIN}}/v1/pokemon/names/74"
                                }
                            }
                        }
                    ]
                }
            ]
        },
        "links": {
            "self": "{{API_DOMAIN}}/v1/trainers/parties/34"
        }
    },
    "links": {
        "self": "{{API_DOMAIN}}/v1/trainers/parties/34"
    }
}
{% end %}

That should make (B)ROCK an easier challenge!

![Brock Nerf](/img/screenshot/brock-nerf.png)
