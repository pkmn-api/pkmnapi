use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_map_pokemon_200, (client, access_token) {
    let request = client
        .get("/v1/maps/pokemon/12")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "12",
            "type": "map_pokemon",
            "attributes": {
                "grass": {
                    "encounter_rate": 25,
                    "pokemon": [
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 2,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 2,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 3,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 4,
                            "pokemon": {
                                "id": "19",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "RATTATA"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/19"
                                }
                            }
                        },
                        {
                            "level": 4,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/16"
                                }
                            }
                        },
                        {
                            "level": 5,
                            "pokemon": {
                                "id": "16",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIDGEY"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/16"
                                }
                            }
                        }
                    ]
                },
                "water": {
                    "encounter_rate": 0,
                    "pokemon": []
                }
            },
            "links": {
                "self": "http://localhost:8080/v1/maps/pokemon/12"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/maps/pokemon/12"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_map_pokemon_401, (client) {
    let request = client.get("/v1/maps/pokemon/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_map_pokemon_404, (client, access_token) {
    let request = client
        .get("/v1/maps/pokemon/255")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_map_pokemon",
            "type": "errors",
            "attributes": {
                "message": "Invalid map ID 255: valid range is 0-247"
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

test!(post_map_pokemon_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "map_pokemon",
            "attributes": {
                "grass": {
                    "encounter_rate": 25,
                    "pokemon": [
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        }
                    ]
                },
                "water": {
                    "encounter_rate": 0,
                    "pokemon": []
                }
            }
        }
    });

    let request = client
        .post("/v1/maps/pokemon/12")
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
        .get("/v1/maps/pokemon/12")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "12",
            "type": "map_pokemon",
            "attributes": {
                "grass": {
                    "encounter_rate": 25,
                    "pokemon": [
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25",
                                "type": "pokemon_names",
                                "attributes": {
                                    "name": "PIKACHU"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/pokemon/names/25"
                                }
                            }
                        }
                    ]
                },
                "water": {
                    "encounter_rate": 0,
                    "pokemon": []
                }
            },
            "links": {
                "self": "http://localhost:8080/v1/maps/pokemon/12"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/maps/pokemon/12"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_map_pokemon_401, (client) {
    let request_body = json!({
        "data": {
            "type": "map_pokemon",
            "attributes": {
                "grass": {
                    "encounter_rate": 25,
                    "pokemon": [
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        }
                    ]
                },
                "water": {
                    "encounter_rate": 0,
                    "pokemon": []
                }
            }
        }
    });

    let request = client
        .post("/v1/maps/pokemon/12")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_map_pokemon_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "map_pokemon",
            "attributes": {
                "grass": {
                    "encounter_rate": 25,
                    "pokemon": [
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        },
                        {
                            "level": 10,
                            "pokemon": {
                                "id": "25"
                            }
                        }
                    ]
                },
                "water": {
                    "encounter_rate": 0,
                    "pokemon": []
                }
            }
        }
    });

    let request = client
        .post("/v1/maps/pokemon/255")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_map_pokemon",
            "type": "errors",
            "attributes": {
                "message": "Invalid map ID 255: valid range is 0-247"
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
