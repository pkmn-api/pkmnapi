use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_pokemon_evolutions_200_level, (client, access_token) {
    let request = client
        .get("/v1/pokemon/evolutions/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "level",
                        "level": 16,
                        "pokemon": {
                            "id": "2",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "IVYSAUR"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/pokemon/names/2"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/evolutions/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/evolutions/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_evolutions_200_item, (client, access_token) {
    let request = client
        .get("/v1/pokemon/evolutions/25")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "25",
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "item",
                        "item": {
                            "id": "33",
                            "type": "item_names",
                            "attributes": {
                                "name": "THUNDERSTONE"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/items/names/33"
                            }
                        },
                        "pokemon": {
                            "id": "26",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "RAICHU"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/pokemon/names/26"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/evolutions/25"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/evolutions/25"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_evolutions_200_trade, (client, access_token) {
    let request = client
        .get("/v1/pokemon/evolutions/64")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "64",
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "trade",
                        "pokemon": {
                            "id": "65",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "ALAKAZAM"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/pokemon/names/65"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/evolutions/64"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/evolutions/64"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_evolutions_401, (client) {
    let request = client.get("/v1/pokemon/evolutions/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_pokemon_evolutions_404, (client, access_token) {
    let request = client
        .get("/v1/pokemon/evolutions/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_pokemon_evolutions",
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

test!(post_pokemon_evolutions_202_level, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "level",
                        "level": 16,
                        "pokemon": {
                            "id": "4"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/evolutions/1")
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
        .get("/v1/pokemon/evolutions/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "level",
                        "level": 16,
                        "pokemon": {
                            "id": "4",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "CHARMANDER"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/pokemon/names/4"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/evolutions/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/evolutions/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_evolutions_202_item, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "item",
                        "item": {
                            "id": "10"
                        },
                        "pokemon": {
                            "id": "4"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/evolutions/25")
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
        .get("/v1/pokemon/evolutions/25")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "25",
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "item",
                        "item": {
                            "id": "10",
                            "type": "item_names",
                            "attributes": {
                                "name": "MOON STONE"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/items/names/10"
                            }
                        },
                        "pokemon": {
                            "id": "4",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "CHARMANDER"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/pokemon/names/4"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/evolutions/25"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/evolutions/25"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_evolutions_202_trade, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "trade",
                        "pokemon": {
                            "id": "4"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/evolutions/64")
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
        .get("/v1/pokemon/evolutions/64")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "64",
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "trade",
                        "pokemon": {
                            "id": "4",
                            "type": "pokemon_names",
                            "attributes": {
                                "name": "CHARMANDER"
                            },
                            "links": {
                                "self": "http://localhost:8080/v1/pokemon/names/4"
                            }
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/evolutions/64"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/evolutions/64"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_evolutions_401, (client) {
    let request_body = json!({
        "data": {
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "level",
                        "level": 16,
                        "pokemon": {
                            "id": "4"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/evolutions/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_pokemon_evolutions_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_evolutions",
            "attributes": {
                "evolutions": [
                    {
                        "evolution_type": "level",
                        "level": 16,
                        "pokemon": {
                            "id": "4"
                        }
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/evolutions/200")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_pokemon_evolutions",
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
