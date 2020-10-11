use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_trainer_parties_200, (client, access_token) {
    let request = client
        .get("/v1/trainers/parties/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "trainer_parties",
            "attributes": {
                "parties": [
                    {
                        "party": [
                            {
                                "level": 11,
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
                                "level": 11,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 10,
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
                                "level": 10,
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
                                "level": 10,
                                "pokemon": {
                                    "id": "41",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "ZUBAT"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/41"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
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
                                "level": 14,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "41",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "ZUBAT"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/41"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 15,
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
                                "level": 15,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "79",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SLOWPOKE"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/79"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "27",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SANDSHREW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/27"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "32",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "NIDORAN♂"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/32"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "27",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SANDSHREW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/27"
                                    }
                                }
                            },
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "41",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "ZUBAT"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/41"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
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
                                "level": 17,
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
                                "level": 17,
                                "pokemon": {
                                    "id": "20",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "RATICATE"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/20"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "32",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "NIDORAN♂"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/32"
                                    }
                                }
                            },
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "33",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "NIDORINO"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/33"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            },
                            {
                                "level": 17,
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
                                "level": 17,
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
                                "level": 17,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            }
                        ]
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/trainers/parties/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/trainers/parties/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_trainer_parties_401, (client) {
    let request = client.get("/v1/trainers/parties/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_trainer_parties_404, (client, access_token) {
    let request = client
        .get("/v1/trainers/parties/100")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_trainer_parties",
            "type": "errors",
            "attributes": {
                "message": "Invalid trainer ID 100: valid range is 1-47"
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

test!(post_trainer_parties_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "trainer_parties",
            "attributes": {
                "parties": [
                    {
                        "party": [
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "20"
                                }
                            },
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "23"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 15,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 15,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "79"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "27"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "32"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "23"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "27"
                                }
                            },
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "20"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "32"
                                }
                            },
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "33"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/trainers/parties/1")
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
        .get("/v1/trainers/parties/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "trainer_parties",
            "attributes": {
                "parties": [
                    {
                        "party": [
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "20",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "RATICATE"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/20"
                                    }
                                }
                            },
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 10,
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
                                "level": 10,
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
                                "level": 10,
                                "pokemon": {
                                    "id": "41",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "ZUBAT"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/41"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
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
                                "level": 14,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "41",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "ZUBAT"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/41"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 15,
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
                                "level": 15,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "79",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SLOWPOKE"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/79"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "27",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SANDSHREW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/27"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "32",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "NIDORAN♂"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/32"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "23",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "EKANS"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/23"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "27",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SANDSHREW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/27"
                                    }
                                }
                            },
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "41",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "ZUBAT"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/41"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
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
                                "level": 17,
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
                                "level": 17,
                                "pokemon": {
                                    "id": "20",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "RATICATE"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/20"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "32",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "NIDORAN♂"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/32"
                                    }
                                }
                            },
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "33",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "NIDORINO"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/33"
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            },
                            {
                                "level": 17,
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
                                "level": 17,
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
                                "level": 17,
                                "pokemon": {
                                    "id": "21",
                                    "type": "pokemon_names",
                                    "attributes": {
                                        "name": "SPEAROW"
                                    },
                                    "links": {
                                        "self": "http://localhost:8080/v1/pokemon/names/21"
                                    }
                                }
                            }
                        ]
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/trainers/parties/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/trainers/parties/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_trainer_parties_401, (client) {
    let request_body = json!({
        "data": {
            "type": "trainer_parties",
            "attributes": {
                "parties": [
                    {
                        "party": [
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "20"
                                }
                            },
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "23"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 15,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 15,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "79"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "27"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "32"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "23"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "27"
                                }
                            },
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "20"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "32"
                                }
                            },
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "33"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/trainers/parties/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_trainer_parties_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "trainer_parties",
            "attributes": {
                "parties": [
                    {
                        "party": [
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "20"
                                }
                            },
                            {
                                "level": 11,
                                "pokemon": {
                                    "id": "23"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 10,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 15,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 15,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "79"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "23"
                                }
                            },
                            {
                                "level": 14,
                                "pokemon": {
                                    "id": "27"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "32"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 21,
                                "pokemon": {
                                    "id": "23"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "27"
                                }
                            },
                            {
                                "level": 19,
                                "pokemon": {
                                    "id": "41"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "20"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "32"
                                }
                            },
                            {
                                "level": 18,
                                "pokemon": {
                                    "id": "33"
                                }
                            }
                        ]
                    },
                    {
                        "party": [
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "19"
                                }
                            },
                            {
                                "level": 17,
                                "pokemon": {
                                    "id": "21"
                                }
                            }
                        ]
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/trainers/parties/100")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_trainer_parties",
            "type": "errors",
            "attributes": {
                "message": "Invalid trainer ID 100: valid range is 1-47"
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
