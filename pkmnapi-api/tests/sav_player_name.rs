use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_sav_player_name_200, (client, access_token) {
    let request = client
        .get("/v1/savs/player_names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "666",
            "type": "sav_player_names",
            "attributes": {
                "name": "RED"
            },
            "links": {
                "self": "http://localhost:8080/v1/savs/player_names"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/savs/player_names"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_sav_player_name_401, (client) {
    let request = client.get("/v1/savs/player_names");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_sav_player_name_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "sav_player_names",
            "attributes": {
                "name": "BLUE"
            }
        }
    });

    let request = client
        .post("/v1/savs/player_names")
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
        .get("/v1/savs/player_names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "666",
            "type": "sav_player_names",
            "attributes": {
                "name": "BLUE"
            },
            "links": {
                "self": "http://localhost:8080/v1/savs/player_names"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/savs/player_names"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_sav_player_name_401, (client) {
    let request_body = json!({
        "data": {
            "type": "sav_player_names",
            "attributes": {
                "name": "BLUE"
            }
        }
    });

    let request = client
        .post("/v1/savs/player_names")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});
