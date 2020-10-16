use pkmnapi_api::responses::pokedex_entries::PokedexEntryResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_pokedex_entry_all_200, (client, access_token) {
    let request = client
        .get("/v1/pokedex/entries")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<PokedexEntryResponseAll>("../secrets/data/json/get_pokedex_entry_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokedex_entry_200, (client, access_token) {
    let request = client
        .get("/v1/pokedex/entries/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokedex_entries",
            "attributes": {
                "species": "SEED",
                "height": 28,
                "weight": 150
            },
            "links": {
                "self": "http://localhost:8080/v1/pokedex/entries/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokedex/entries/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokedex_entry_401, (client) {
    let request = client.get("/v1/pokedex/entries/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_pokedex_entry_404, (client, access_token) {
    let request = client
        .get("/v1/pokedex/entries/200")
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

test!(post_pokedex_entry_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokedex_entries",
            "attributes": {
                "species": "LEAF",
                "height": 42,
                "weight": 42
            }
        }
    });

    let request = client
        .post("/v1/pokedex/entries/1")
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
        .get("/v1/pokedex/entries/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "pokedex_entries",
            "attributes": {
                "species": "LEAF",
                "height": 42,
                "weight": 42
            },
            "links": {
                "self": "http://localhost:8080/v1/pokedex/entries/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/pokedex/entries/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokedex_entry_401, (client) {
    let request_body = json!({
        "data": {
            "type": "pokedex_entries",
            "attributes": {
                "species": "LEAF",
                "height": 42,
                "weight": 42
            }
        }
    });

    let request = client
        .post("/v1/pokedex/entries/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_pokedex_entry_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "pokedex_entries",
            "attributes": {
                "species": "LEAF",
                "height": 42,
                "weight": 42
            }
        }
    });

    let request = client
        .post("/v1/pokedex/entries/200")
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
