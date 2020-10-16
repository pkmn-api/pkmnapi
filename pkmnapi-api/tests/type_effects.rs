use pkmnapi_api::responses::type_effects::TypeEffectResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_type_effect_all_200, (client, access_token) {
    let request = client
        .get("/v1/types/effects")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<TypeEffectResponseAll>("../secrets/data/json/get_type_effect_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_type_effect_200, (client, access_token) {
    let request = client
        .get("/v1/types/effects/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "type_effects",
            "attributes": {
                "attacking_type": {
                    "id": "21",
                    "type": "type_names",
                    "attributes": {
                        "name": "WATER"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/types/names/21"
                    }
                },
                "defending_type": {
                    "id": "20",
                    "type": "type_names",
                    "attributes": {
                        "name": "FIRE"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/types/names/20"
                    }
                },
                "multiplier": 2.0
            },
            "links": {
                "self": "http://localhost:8080/v1/types/effects/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/types/effects/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_type_effect_401, (client) {
    let request = client.get("/v1/types/effects/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_type_effect_404, (client, access_token) {
    let request = client
        .get("/v1/types/effects/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid type effect ID 200: valid range is 0-81"
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

test!(post_type_effect_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_effects",
            "attributes": {
                "attacking_type": {
                    "id": "0"
                },
                "defending_type": {
                    "id": "0"
                },
                "multiplier": 0
            }
        }
    });

    let request = client
        .post("/v1/types/effects/0")
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
        .get("/v1/types/effects/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "type_effects",
            "attributes": {
                "attacking_type": {
                    "id": "0",
                    "type": "type_names",
                    "attributes": {
                        "name": "NORMAL"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/types/names/0"
                    }
                },
                "defending_type": {
                    "id": "0",
                    "type": "type_names",
                    "attributes": {
                        "name": "NORMAL"
                    },
                    "links": {
                        "self": "http://localhost:8080/v1/types/names/0"
                    }
                },
                "multiplier": 0.0
            },
            "links": {
                "self": "http://localhost:8080/v1/types/effects/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/types/effects/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_type_effect_401, (client) {
    let request_body = json!({
        "data": {
            "type": "type_effects",
            "attributes": {
                "attacking_type": {
                    "id": "0"
                },
                "defending_type": {
                    "id": "0"
                },
                "multiplier": 0
            }
        }
    });

    let request = client
        .post("/v1/types/effects/0")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_type_effect_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_effects",
            "attributes": {
                "attacking_type": {
                    "id": "0"
                },
                "defending_type": {
                    "id": "0"
                },
                "multiplier": 0
            }
        }
    });

    let request = client
        .post("/v1/types/effects/200")
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
                "message": "Invalid type effect ID 200: valid range is 0-81"
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
