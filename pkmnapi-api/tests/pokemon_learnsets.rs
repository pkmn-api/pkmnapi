use pkmnapi_api::responses::pokemon_learnsets::PokemonLearnsetResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_pokemon_learnset_all_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/learnsets")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<PokemonLearnsetResponseAll>("../secrets/data/json/get_pokemon_learnset_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_learnset_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/learnsets/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_learnsets",
            "attributes": {
                "learnset": [
                    {
                        "level": 7,
                        "move": {
                            "id": "73",
                            "type": "move_names",
                            "attributes": {
                                "name": "LEECH SEED"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/73"
                            }
                        }
                    },
                    {
                        "level": 13,
                        "move": {
                            "id": "22",
                            "type": "move_names",
                            "attributes": {
                                "name": "VINE WHIP"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/22"
                            }
                        }
                    },
                    {
                        "level": 20,
                        "move": {
                            "id": "77",
                            "type": "move_names",
                            "attributes": {
                                "name": "POISONPOWDER"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/77"
                            }
                        }
                    },
                    {
                        "level": 27,
                        "move": {
                            "id": "75",
                            "type": "move_names",
                            "attributes": {
                                "name": "RAZOR LEAF"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/75"
                            }
                        }
                    },
                    {
                        "level": 34,
                        "move": {
                            "id": "74",
                            "type": "move_names",
                            "attributes": {
                                "name": "GROWTH"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/74"
                            }
                        }
                    },
                    {
                        "level": 41,
                        "move": {
                            "id": "79",
                            "type": "move_names",
                            "attributes": {
                                "name": "SLEEP POWDER"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/79"
                            }
                        }
                    },
                    {
                        "level": 48,
                        "move": {
                            "id": "76",
                            "type": "move_names",
                            "attributes": {
                                "name": "SOLARBEAM"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/76"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/learnsets/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/learnsets/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_learnset_401, (client) {
    let request = client.get("/v1/pokemon/learnsets/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_pokemon_learnset_404, (client, access_token) {
    let request = client
        .get("/v1/pokemon/learnsets/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid Pokédex ID: 200"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::NotFound);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_learnset_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_learnsets",
            "attributes": {
                "learnset": [
                    {
                        "level": 5,
                        "move": {
                            "id": "1"
                        }
                    },
                    {
                        "level": 6,
                        "move": {
                            "id": "2"
                        }
                    },
                    {
                        "level": 7,
                        "move": {
                            "id": "3"
                        }
                    },
                    {
                        "level": 8,
                        "move": {
                            "id": "4"
                        }
                    },
                    {
                        "level": 9,
                        "move": {
                            "id": "5"
                        }
                    },
                    {
                        "level": 10,
                        "move": {
                            "id": "6"
                        }
                    },
                    {
                        "level": 11,
                        "move": {
                            "id": "7"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/learnsets/1")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({});

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Accepted);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ]).unwrap();

    let request = client
        .get("/v1/pokemon/learnsets/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_learnsets",
            "attributes": {
                "learnset": [
                    {
                        "level": 5,
                        "move": {
                            "id": "1",
                            "type": "move_names",
                            "attributes": {
                                "name": "POUND"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/1"
                            }
                        }
                    },
                    {
                        "level": 6,
                        "move": {
                            "id": "2",
                            "type": "move_names",
                            "attributes": {
                                "name": "KARATE CHOP"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/2"
                            }
                        }
                    },
                    {
                        "level": 7,
                        "move": {
                            "id": "3",
                            "type": "move_names",
                            "attributes": {
                                "name": "DOUBLESLAP"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/3"
                            }
                        }
                    },
                    {
                        "level": 8,
                        "move": {
                            "id": "4",
                            "type": "move_names",
                            "attributes": {
                                "name": "COMET PUNCH"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/4"
                            }
                        }
                    },
                    {
                        "level": 9,
                        "move": {
                            "id": "5",
                            "type": "move_names",
                            "attributes": {
                                "name": "MEGA PUNCH"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/5"
                            }
                        }
                    },
                    {
                        "level": 10,
                        "move": {
                            "id": "6",
                            "type": "move_names",
                            "attributes": {
                                "name": "PAY DAY"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/6"
                            }
                        }
                    },
                    {
                        "level": 11,
                        "move": {
                            "id": "7",
                            "type": "move_names",
                            "attributes": {
                                "name": "FIRE PUNCH"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/7"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/learnsets/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/learnsets/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_learnset_401, (client) {
    let request_body = json!({
        "data": {
            "type": "pokemon_learnsets",
            "attributes": {
                "learnset": [
                    {
                        "level": 5,
                        "move": {
                            "id": "1"
                        }
                    },
                    {
                        "level": 6,
                        "move": {
                            "id": "2"
                        }
                    },
                    {
                        "level": 7,
                        "move": {
                            "id": "3"
                        }
                    },
                    {
                        "level": 8,
                        "move": {
                            "id": "4"
                        }
                    },
                    {
                        "level": 9,
                        "move": {
                            "id": "5"
                        }
                    },
                    {
                        "level": 10,
                        "move": {
                            "id": "6"
                        }
                    },
                    {
                        "level": 11,
                        "move": {
                            "id": "7"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/learnsets/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_pokemon_learnset_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_learnsets",
            "attributes": {
                "learnset": [
                    {
                        "level": 5,
                        "move": {
                            "id": "1"
                        }
                    },
                    {
                        "level": 6,
                        "move": {
                            "id": "2"
                        }
                    },
                    {
                        "level": 7,
                        "move": {
                            "id": "3"
                        }
                    },
                    {
                        "level": 8,
                        "move": {
                            "id": "4"
                        }
                    },
                    {
                        "level": 9,
                        "move": {
                            "id": "5"
                        }
                    },
                    {
                        "level": 10,
                        "move": {
                            "id": "6"
                        }
                    },
                    {
                        "level": 11,
                        "move": {
                            "id": "7"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/learnsets/200")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid Pokédex ID: 200"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::NotFound);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});
