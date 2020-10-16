use pkmnapi_api::responses::pokemon_stats::PokemonStatsResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_pokemon_stats_all_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/stats")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<PokemonStatsResponseAll>("../secrets/data/json/get_pokemon_stats_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_stats_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/stats/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_stats",
            "attributes": {
                "base_hp": 45,
                "base_attack": 49,
                "base_defence": 49,
                "base_speed": 45,
                "base_special": 65,
                "types": [
                    {
                        "id": "22",
                        "type": "type_names",
                        "attributes": {
                            "name": "GRASS"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/types/names/22"
                        }
                    },
                    {
                        "id": "3",
                        "type": "type_names",
                        "attributes": {
                            "name": "POISON"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/types/names/3"
                        }
                    }
                ],
                "catch_rate": 45,
                "base_exp_yield": 64
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/stats/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/stats/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_stats_401, (client) {
    let request = client.get("/v1/pokemon/stats/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_pokemon_stats_404, (client, access_token) {
    let request = client
        .get("/v1/pokemon/stats/200")
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

test!(post_pokemon_stats_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_stats",
            "attributes": {
                "base_hp": 42,
                "base_attack": 42,
                "base_defence": 42,
                "base_speed": 42,
                "base_special": 42,
                "types": [
                    {
                        "id": "0"
                    },
                    {
                        "id": "0"
                    }
                ],
                "catch_rate": 42,
                "base_exp_yield": 42
            }
        }
    });

    let request = client
        .post("/v1/pokemon/stats/1")
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
        .get("/v1/pokemon/stats/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_stats",
            "attributes": {
                "base_hp": 42,
                "base_attack": 42,
                "base_defence": 42,
                "base_speed": 42,
                "base_special": 42,
                "types": [
                    {
                        "id": "0",
                        "type": "type_names",
                        "attributes": {
                            "name": "NORMAL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/types/names/0"
                        }
                    },
                    {
                        "id": "0",
                        "type": "type_names",
                        "attributes": {
                            "name": "NORMAL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/types/names/0"
                        }
                    }
                ],
                "catch_rate": 42,
                "base_exp_yield": 42
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/stats/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/stats/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_stats_401, (client) {
    let request_body = json!({
        "data": {
            "type": "pokemon_stats",
            "attributes": {
                "base_hp": 42,
                "base_attack": 42,
                "base_defence": 42,
                "base_speed": 42,
                "base_special": 42,
                "types": [
                    {
                        "id": "0"
                    },
                    {
                        "id": "0"
                    }
                ],
                "catch_rate": 42,
                "base_exp_yield": 42
            }
        }
    });

    let request = client
        .post("/v1/pokemon/stats/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_pokemon_stats_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_stats",
            "attributes": {
                "base_hp": 42,
                "base_attack": 42,
                "base_defence": 42,
                "base_speed": 42,
                "base_special": 42,
                "types": [
                    {
                        "id": "0"
                    },
                    {
                        "id": "0"
                    }
                ],
                "catch_rate": 42,
                "base_exp_yield": 42
            }
        }
    });

    let request = client
        .post("/v1/pokemon/stats/200")
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
