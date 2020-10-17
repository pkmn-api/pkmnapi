use pkmnapi_api::responses::pokemon_movesets::PokemonMovesetResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_pokemon_moveset_all_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/movesets")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<PokemonMovesetResponseAll>("../secrets/data/json/get_pokemon_moveset_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_moveset_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/movesets/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_movesets",
            "attributes": {
                "moveset": [
                    {
                        "move": {
                            "id": "33",
                            "type": "move_names",
                            "attributes": {
                                "name": "TACKLE"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/33"
                            }
                        }
                    },
                    {
                        "move": {
                            "id": "45",
                            "type": "move_names",
                            "attributes": {
                                "name": "GROWL"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/moves/names/45"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/movesets/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/movesets/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_moveset_401, (client) {
    let request = client.get("/v1/pokemon/movesets/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_pokemon_moveset_404, (client, access_token) {
    let request = client
        .get("/v1/pokemon/movesets/200")
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

test!(post_pokemon_moveset_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_movesets",
            "attributes": {
                "moveset": [
                    {
                        "move": {
                            "id": "1"
                        }
                    },
                    {
                        "move": {
                            "id": "2"
                        }
                    },
                    {
                        "move": {
                            "id": "3"
                        }
                    },
                    {
                        "move": {
                            "id": "4"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/movesets/1")
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
        .get("/v1/pokemon/movesets/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_movesets",
            "attributes": {
                "moveset": [
                    {
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
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/movesets/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/movesets/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_moveset_401, (client) {
    let request_body = json!({
        "data": {
            "type": "pokemon_movesets",
            "attributes": {
                "moveset": [
                    {
                        "move": {
                            "id": "1"
                        }
                    },
                    {
                        "move": {
                            "id": "2"
                        }
                    },
                    {
                        "move": {
                            "id": "3"
                        }
                    },
                    {
                        "move": {
                            "id": "4"
                        }
                    },
                    {
                        "move": {
                            "id": "5"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/movesets/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_pokemon_moveset_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_movesets",
            "attributes": {
                "moveset": [
                    {
                        "move": {
                            "id": "1"
                        }
                    },
                    {
                        "move": {
                            "id": "2"
                        }
                    },
                    {
                        "move": {
                            "id": "3"
                        }
                    },
                    {
                        "move": {
                            "id": "4"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/movesets/200")
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
