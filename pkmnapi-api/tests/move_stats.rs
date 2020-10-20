use pkmnapi_api::responses::move_stats::MoveStatsResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_move_stats_all_200, (client, access_token) {
    let request = client
        .get("/v1/moves/stats")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<MoveStatsResponseAll>("../secrets/data/json/get_move_stats_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_move_stats_200, (client, access_token) {
    let request = client
        .get("/v1/moves/stats/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "move_stats",
            "attributes": {
                "effect": 0,
                "power": 40,
                "type": {
                    "id": "0",
                    "type": "type_names",
                    "attributes": {
                        "name": "NORMAL"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/types/names/0"
                    }
                },
                "accuracy": 1.0,
                "pp": 35
            },
            "links": {
                "self": "http://localhost:8080/v1/moves/stats/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/moves/stats/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_move_stats_401, (client) {
    let request = client.get("/v1/moves/stats/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_move_stats_404, (client, access_token) {
    let request = client
        .get("/v1/moves/stats/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid move ID 200: valid range is 1-165"
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

test!(post_move_stats_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "move_stats",
            "attributes": {
                "effect": 0,
                "power": 20,
                "type": {
                    "id": "1"
                },
                "accuracy": 0.1337,
                "pp": 42
            }
        }
    });

    let request = client
        .post("/v1/moves/stats/1")
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
        .get("/v1/moves/stats/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "move_stats",
            "attributes": {
                "effect": 0,
                "power": 20,
                "type": {
                    "id": "1",
                    "type": "type_names",
                    "attributes": {
                        "name": "FIGHTING"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/types/names/1"
                    }
                },
                "accuracy": 0.13333334,
                "pp": 42
            },
            "links": {
                "self": "http://localhost:8080/v1/moves/stats/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/moves/stats/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_move_stats_401, (client) {
    let request_body = json!({
        "data": {
            "type": "move_stats",
            "attributes": {
                "effect": 0,
                "power": 20,
                "type": {
                    "id": "1"
                },
                "accuracy": 0.1337,
                "pp": 42
            }
        }
    });

    let request = client
        .post("/v1/moves/stats/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_move_stats_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "move_stats",
            "attributes": {
                "effect": 0,
                "power": 20,
                "type": {
                    "id": "1"
                },
                "accuracy": 0.1337,
                "pp": 42
            }
        }
    });

    let request = client
        .post("/v1/moves/stats/200")
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
                "message": "Invalid move ID 200: valid range is 1-165"
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
