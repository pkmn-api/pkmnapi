use pkmnapi_api::responses::trades::TradeResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_trade_all_200, (client, access_token) {
    let request = client
        .get("/v1/trades")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<TradeResponseAll>("../secrets/data/json/get_trade_all_200.json");

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_trade_200, (client, access_token) {
    let request = client
        .get("/v1/trades/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "trades",
            "attributes": {
                "give": {
                    "id": "33",
                    "type": "pokemon_names",
                    "attributes": {
                        "name": "NIDORINO"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/pokemon/names/33"
                    }
                },
                "get": {
                    "id": "30",
                    "type": "pokemon_names",
                    "attributes": {
                        "name": "NIDORINA"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/pokemon/names/30"
                    }
                },
                "nickname": "TERRY"
            },
            "links": {
                "self": "http://localhost:8080/v1/trades/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/trades/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_trade_401, (client) {
    let request = client.get("/v1/trades/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_trade_404, (client, access_token) {
    let request = client
        .get("/v1/trades/100")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid trade ID 100: valid range is 0-9"
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

test!(post_trade_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "trades",
            "attributes": {
                "give": {
                    "id": "4"
                },
                "get": {
                    "id": "6"
                },
                "nickname": "CHARCHAR"
            }
        }
    });

    let request = client
        .post("/v1/trades/0")
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
        .get("/v1/trades/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "trades",
            "attributes": {
                "give": {
                    "id": "4",
                    "type": "pokemon_names",
                    "attributes": {
                        "name": "CHARMANDER"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/pokemon/names/4"
                    }
                },
                "get": {
                    "id": "6",
                    "type": "pokemon_names",
                    "attributes": {
                        "name": "CHARIZARD"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/pokemon/names/6"
                    }
                },
                "nickname": "CHARCHAR"
            },
            "links": {
                "self": "http://localhost:8080/v1/trades/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/trades/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_trade_401, (client) {
    let request_body = json!({
        "data": {
            "type": "trades",
            "attributes": {
                "give": {
                    "id": "4"
                },
                "get": {
                    "id": "6"
                },
                "nickname": "CHARCHAR"
            }
        }
    });

    let request = client
        .post("/v1/trades/0")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_trade_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "trades",
            "attributes": {
                "give": {
                    "id": "4"
                },
                "get": {
                    "id": "6"
                },
                "nickname": "CHARCHAR"
            }
        }
    });

    let request = client
        .post("/v1/trades/100")
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
                "message": "Invalid trade ID 100: valid range is 0-9"
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
