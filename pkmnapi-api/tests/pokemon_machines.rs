use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_pokemon_machines_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/machines/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_machines",
            "attributes": {
                "machines": [
                    {
                        "id": "3",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "14",
                                "type": "move_names",
                                "attributes": {
                                    "name": "SWORDS DANCE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/14"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/3"
                        }
                    },
                    {
                        "id": "6",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "92",
                                "type": "move_names",
                                "attributes": {
                                    "name": "TOXIC"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/92"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/6"
                        }
                    },
                    {
                        "id": "8",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "34",
                                "type": "move_names",
                                "attributes": {
                                    "name": "BODY SLAM"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/34"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/8"
                        }
                    },
                    {
                        "id": "9",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "36",
                                "type": "move_names",
                                "attributes": {
                                    "name": "TAKE DOWN"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/36"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/9"
                        }
                    },
                    {
                        "id": "10",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "38",
                                "type": "move_names",
                                "attributes": {
                                    "name": "DOUBLE-EDGE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/38"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/10"
                        }
                    },
                    {
                        "id": "20",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "99",
                                "type": "move_names",
                                "attributes": {
                                    "name": "RAGE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/99"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/20"
                        }
                    },
                    {
                        "id": "21",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "72",
                                "type": "move_names",
                                "attributes": {
                                    "name": "MEGA DRAIN"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/72"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/21"
                        }
                    },
                    {
                        "id": "22",
                        "type": "tm_moves",
                        "attributes": {
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
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/22"
                        }
                    },
                    {
                        "id": "31",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "102",
                                "type": "move_names",
                                "attributes": {
                                    "name": "MIMIC"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/102"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/31"
                        }
                    },
                    {
                        "id": "32",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "104",
                                "type": "move_names",
                                "attributes": {
                                    "name": "DOUBLE TEAM"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/104"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/32"
                        }
                    },
                    {
                        "id": "33",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "115",
                                "type": "move_names",
                                "attributes": {
                                    "name": "REFLECT"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/115"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/33"
                        }
                    },
                    {
                        "id": "34",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "117",
                                "type": "move_names",
                                "attributes": {
                                    "name": "BIDE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/117"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/34"
                        }
                    },
                    {
                        "id": "44",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "156",
                                "type": "move_names",
                                "attributes": {
                                    "name": "REST"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/156"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/44"
                        }
                    },
                    {
                        "id": "50",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "164",
                                "type": "move_names",
                                "attributes": {
                                    "name": "SUBSTITUTE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/164"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/50"
                        }
                    },
                    {
                        "id": "1",
                        "type": "hm_moves",
                        "attributes": {
                            "move": {
                                "id": "15",
                                "type": "move_names",
                                "attributes": {
                                    "name": "CUT"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/15"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/hms/moves/1"
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/machines/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/machines/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_machines_401, (client) {
    let request = client.get("/v1/pokemon/machines/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_pokemon_machines_404, (client, access_token) {
    let request = client
        .get("/v1/pokemon/machines/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_pokemon_machines",
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

test!(post_pokemon_machines_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_machines",
            "attributes": {
                "machines": [
                    {
                        "id": "2",
                        "type": "tm_moves"
                    },
                    {
                        "id": "7",
                        "type": "tm_moves"
                    },
                    {
                        "id": "9",
                        "type": "tm_moves"
                    },
                    {
                        "id": "10",
                        "type": "tm_moves"
                    },
                    {
                        "id": "13",
                        "type": "tm_moves"
                    },
                    {
                        "id": "17",
                        "type": "tm_moves"
                    },
                    {
                        "id": "18",
                        "type": "tm_moves"
                    },
                    {
                        "id": "19",
                        "type": "tm_moves"
                    },
                    {
                        "id": "21",
                        "type": "tm_moves"
                    },
                    {
                        "id": "22",
                        "type": "tm_moves"
                    },
                    {
                        "id": "25",
                        "type": "tm_moves"
                    },
                    {
                        "id": "27",
                        "type": "tm_moves"
                    },
                    {
                        "id": "36",
                        "type": "tm_moves"
                    },
                    {
                        "id": "38",
                        "type": "tm_moves"
                    },
                    {
                        "id": "39",
                        "type": "tm_moves"
                    },
                    {
                        "id": "42",
                        "type": "tm_moves"
                    },
                    {
                        "id": "47",
                        "type": "tm_moves"
                    },
                    {
                        "id": "49",
                        "type": "tm_moves"
                    },
                    {
                        "id": "50",
                        "type": "tm_moves"
                    },
                    {
                        "id": "1",
                        "type": "hm_moves"
                    },
                    {
                        "id": "2",
                        "type": "hm_moves"
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/machines/1")
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
        .get("/v1/pokemon/machines/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_machines",
            "attributes": {
                "machines": [
                    {
                        "id": "2",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "13",
                                "type": "move_names",
                                "attributes": {
                                    "name": "RAZOR WIND"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/13"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/2"
                        }
                    },
                    {
                        "id": "7",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "32",
                                "type": "move_names",
                                "attributes": {
                                    "name": "HORN DRILL"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/32"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/7"
                        }
                    },
                    {
                        "id": "9",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "36",
                                "type": "move_names",
                                "attributes": {
                                    "name": "TAKE DOWN"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/36"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/9"
                        }
                    },
                    {
                        "id": "10",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "38",
                                "type": "move_names",
                                "attributes": {
                                    "name": "DOUBLE-EDGE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/38"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/10"
                        }
                    },
                    {
                        "id": "13",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "58",
                                "type": "move_names",
                                "attributes": {
                                    "name": "ICE BEAM"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/58"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/13"
                        }
                    },
                    {
                        "id": "17",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "66",
                                "type": "move_names",
                                "attributes": {
                                    "name": "SUBMISSION"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/66"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/17"
                        }
                    },
                    {
                        "id": "18",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "68",
                                "type": "move_names",
                                "attributes": {
                                    "name": "COUNTER"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/68"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/18"
                        }
                    },
                    {
                        "id": "19",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "69",
                                "type": "move_names",
                                "attributes": {
                                    "name": "SEISMIC TOSS"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/69"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/19"
                        }
                    },
                    {
                        "id": "21",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "72",
                                "type": "move_names",
                                "attributes": {
                                    "name": "MEGA DRAIN"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/72"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/21"
                        }
                    },
                    {
                        "id": "22",
                        "type": "tm_moves",
                        "attributes": {
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
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/22"
                        }
                    },
                    {
                        "id": "25",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "87",
                                "type": "move_names",
                                "attributes": {
                                    "name": "THUNDER"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/87"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/25"
                        }
                    },
                    {
                        "id": "27",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "90",
                                "type": "move_names",
                                "attributes": {
                                    "name": "FISSURE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/90"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/27"
                        }
                    },
                    {
                        "id": "36",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "120",
                                "type": "move_names",
                                "attributes": {
                                    "name": "SELFDESTRUCT"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/120"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/36"
                        }
                    },
                    {
                        "id": "38",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "126",
                                "type": "move_names",
                                "attributes": {
                                    "name": "FIRE BLAST"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/126"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/38"
                        }
                    },
                    {
                        "id": "39",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "129",
                                "type": "move_names",
                                "attributes": {
                                    "name": "SWIFT"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/129"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/39"
                        }
                    },
                    {
                        "id": "42",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "138",
                                "type": "move_names",
                                "attributes": {
                                    "name": "DREAM EATER"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/138"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/42"
                        }
                    },
                    {
                        "id": "47",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "153",
                                "type": "move_names",
                                "attributes": {
                                    "name": "EXPLOSION"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/153"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/47"
                        }
                    },
                    {
                        "id": "49",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "161",
                                "type": "move_names",
                                "attributes": {
                                    "name": "TRI ATTACK"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/161"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/49"
                        }
                    },
                    {
                        "id": "50",
                        "type": "tm_moves",
                        "attributes": {
                            "move": {
                                "id": "164",
                                "type": "move_names",
                                "attributes": {
                                    "name": "SUBSTITUTE"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/164"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/tms/moves/50"
                        }
                    },
                    {
                        "id": "1",
                        "type": "hm_moves",
                        "attributes": {
                            "move": {
                                "id": "15",
                                "type": "move_names",
                                "attributes": {
                                    "name": "CUT"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/15"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/hms/moves/1"
                        }
                    },
                    {
                        "id": "2",
                        "type": "hm_moves",
                        "attributes": {
                            "move": {
                                "id": "19",
                                "type": "move_names",
                                "attributes": {
                                    "name": "FLY"
                                },
                                "links": {
                                    "self": "http://localhost:8080/v1/moves/names/19"
                                }
                            }
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/hms/moves/2"
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/machines/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/machines/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_machines_401, (client) {
    let request_body = json!({
        "data": {
            "type": "pokemon_machines",
            "attributes": {
                "machines": [
                    {
                        "id": "2",
                        "type": "tm_moves"
                    },
                    {
                        "id": "7",
                        "type": "tm_moves"
                    },
                    {
                        "id": "9",
                        "type": "tm_moves"
                    },
                    {
                        "id": "10",
                        "type": "tm_moves"
                    },
                    {
                        "id": "13",
                        "type": "tm_moves"
                    },
                    {
                        "id": "17",
                        "type": "tm_moves"
                    },
                    {
                        "id": "18",
                        "type": "tm_moves"
                    },
                    {
                        "id": "19",
                        "type": "tm_moves"
                    },
                    {
                        "id": "21",
                        "type": "tm_moves"
                    },
                    {
                        "id": "22",
                        "type": "tm_moves"
                    },
                    {
                        "id": "25",
                        "type": "tm_moves"
                    },
                    {
                        "id": "27",
                        "type": "tm_moves"
                    },
                    {
                        "id": "36",
                        "type": "tm_moves"
                    },
                    {
                        "id": "38",
                        "type": "tm_moves"
                    },
                    {
                        "id": "39",
                        "type": "tm_moves"
                    },
                    {
                        "id": "42",
                        "type": "tm_moves"
                    },
                    {
                        "id": "47",
                        "type": "tm_moves"
                    },
                    {
                        "id": "49",
                        "type": "tm_moves"
                    },
                    {
                        "id": "50",
                        "type": "tm_moves"
                    },
                    {
                        "id": "1",
                        "type": "hm_moves"
                    },
                    {
                        "id": "2",
                        "type": "hm_moves"
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/machines/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_pokemon_machines_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_machines",
            "attributes": {
                "machines": [
                    {
                        "id": "2",
                        "type": "tm_moves"
                    },
                    {
                        "id": "7",
                        "type": "tm_moves"
                    },
                    {
                        "id": "9",
                        "type": "tm_moves"
                    },
                    {
                        "id": "10",
                        "type": "tm_moves"
                    },
                    {
                        "id": "13",
                        "type": "tm_moves"
                    },
                    {
                        "id": "17",
                        "type": "tm_moves"
                    },
                    {
                        "id": "18",
                        "type": "tm_moves"
                    },
                    {
                        "id": "19",
                        "type": "tm_moves"
                    },
                    {
                        "id": "21",
                        "type": "tm_moves"
                    },
                    {
                        "id": "22",
                        "type": "tm_moves"
                    },
                    {
                        "id": "25",
                        "type": "tm_moves"
                    },
                    {
                        "id": "27",
                        "type": "tm_moves"
                    },
                    {
                        "id": "36",
                        "type": "tm_moves"
                    },
                    {
                        "id": "38",
                        "type": "tm_moves"
                    },
                    {
                        "id": "39",
                        "type": "tm_moves"
                    },
                    {
                        "id": "42",
                        "type": "tm_moves"
                    },
                    {
                        "id": "47",
                        "type": "tm_moves"
                    },
                    {
                        "id": "49",
                        "type": "tm_moves"
                    },
                    {
                        "id": "50",
                        "type": "tm_moves"
                    },
                    {
                        "id": "1",
                        "type": "hm_moves"
                    },
                    {
                        "id": "2",
                        "type": "hm_moves"
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/pokemon/machines/200")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_pokemon_machines",
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
