use pkmnapi_api::responses::pokemon_cries::PokemonCryResponseAll;
use rocket::http::{Accept, ContentType, Status};
use serde_json::json;
use std::fs;

mod common;

test!(get_pokemon_cry_all_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/cries")
        .header(common::auth_header(&access_token))
        .header(Accept::JSON);

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<PokemonCryResponseAll>("../secrets/data/json/get_pokemon_cry_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_cry_200, (client, access_token) {
    let request = client
        .get("/v1/pokemon/cries/1")
        .header(common::auth_header(&access_token))
        .header(Accept::JSON);

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_cries",
            "attributes": {
                "base": 15,
                "pitch": 128,
                "length": 1
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/cries/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/cries/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_cry_200_wav, (client, access_token) {
    let request = client
        .get("/v1/pokemon/cries/1")
        .header(common::auth_header(&access_token))
        .header(Accept::WAV);

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/pokemon_cry/1.wav").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"BULBASAUR.wav\""),
        ("Content-Type", "audio/wav"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_cry_401, (client) {
    let request = client.get("/v1/pokemon/cries/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_pokemon_cry_404, (client, access_token) {
    let request = client
        .get("/v1/pokemon/cries/200")
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

test!(post_pokemon_cry_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_cries",
            "attributes": {
                "base": 13,
                "pitch": 128,
                "length": 10
            }
        }
    });

    let request = client
        .post("/v1/pokemon/cries/1")
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
        .get("/v1/pokemon/cries/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokemon_cries",
            "attributes": {
                "base": 13,
                "pitch": 128,
                "length": 10
            },
            "links": {
                "self": "http://localhost:8080/v1/pokemon/cries/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokemon/cries/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_cry_401, (client) {
    let request_body = json!({
        "data": {
            "type": "pokemon_cries",
            "attributes": {
                "base": 13,
                "pitch": 128,
                "length": 10
            }
        }
    });

    let request = client
        .post("/v1/pokemon/cries/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_pokemon_cry_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokemon_cries",
            "attributes": {
                "base": 13,
                "pitch": 128,
                "length": 10
            }
        }
    });

    let request = client
        .post("/v1/pokemon/cries/200")
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
