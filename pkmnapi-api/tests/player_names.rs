use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_player_names_200, (client, access_token) {
    let request = client
        .get("/v1/player_names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "player_names",
            "attributes": {
                "player": [
                    "RED",
                    "ASH",
                    "JACK"
                ],
                "rival": [
                    "BLUE",
                    "GARY",
                    "JOHN"
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/player_names"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/player_names"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_player_names_401, (client) {
    let request = client.get("/v1/player_names");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_player_names_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "player_names",
            "attributes": {
                "player": [
                    "BED",
                    "ASK",
                    "JILL"
                ],
                "rival": [
                    "TRUE",
                    "MARY",
                    "JANE"
                ]
            }
        }
    });

    let request = client
        .post("/v1/player_names")
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
        .get("/v1/player_names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "player_names",
            "attributes": {
                "player": [
                    "BED",
                    "ASK",
                    "JILL"
                ],
                "rival": [
                    "TRUE",
                    "MARY",
                    "JANE"
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/player_names"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/player_names"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_player_names_401, (client) {
    let request_body = json!({
        "data": {
            "type": "player_names",
            "attributes": {
                "player": [
                    "RED",
                    "ASH",
                    "JACK"
                ],
                "rival": [
                    "BLUE",
                    "GARY",
                    "JOHN"
                ]
            }
        }
    });

    let request = client
        .post("/v1/player_names")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});
