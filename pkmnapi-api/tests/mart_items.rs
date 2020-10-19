use pkmnapi_api::responses::mart_items::MartItemsResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_mart_items_all_200, (client, access_token) {
    let request = client
        .get("/v1/marts/items")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<MartItemsResponseAll>("../secrets/data/json/get_mart_items_all_200.json");

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_mart_items_200, (client, access_token) {
    let request = client
        .get("/v1/marts/items/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "mart_items",
            "attributes": {
                "mart_items": [
                    {
                        "id": "4",
                        "type": "item_names",
                        "attributes": {
                            "name": "POKé BALL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/4"
                        }
                    },
                    {
                        "id": "11",
                        "type": "item_names",
                        "attributes": {
                            "name": "ANTIDOTE"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/11"
                        }
                    },
                    {
                        "id": "15",
                        "type": "item_names",
                        "attributes": {
                            "name": "PARLYZ HEAL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/15"
                        }
                    },
                    {
                        "id": "12",
                        "type": "item_names",
                        "attributes": {
                            "name": "BURN HEAL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/12"
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/marts/items/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/marts/items/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_mart_items_401, (client) {
    let request = client.get("/v1/marts/items/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_mart_items_404, (client, access_token) {
    let request = client
        .get("/v1/marts/items/100")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid mart ID 100: valid range is 0-15"
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

test!(post_mart_items_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "mart_items",
            "attributes": {
                "mart_items": [
                    {
                        "id": "1",
                        "type": "item_names"
                    },
                    {
                        "id": "2",
                        "type": "item_names"
                    },
                    {
                        "id": "3",
                        "type": "item_names"
                    },
                    {
                        "id": "4",
                        "type": "item_names"
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/marts/items/0")
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
        .get("/v1/marts/items/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "mart_items",
            "attributes": {
                "mart_items": [
                    {
                        "id": "1",
                        "type": "item_names",
                        "attributes": {
                            "name": "MASTER BALL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/1"
                        }
                    },
                    {
                        "id": "2",
                        "type": "item_names",
                        "attributes": {
                            "name": "ULTRA BALL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/2"
                        }
                    },
                    {
                        "id": "3",
                        "type": "item_names",
                        "attributes": {
                            "name": "GREAT BALL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/3"
                        }
                    },
                    {
                        "id": "4",
                        "type": "item_names",
                        "attributes": {
                            "name": "POKé BALL"
                        },
                        "links": {
                            "self": "http://localhost:8080/v1/items/names/4"
                        }
                    }
                ]
            },
            "links": {
                "self": "http://localhost:8080/v1/marts/items/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/marts/items/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_mart_items_401, (client) {
    let request_body = json!({
        "data": {
            "type": "mart_items",
            "attributes": {
                "mart_items": [
                    {
                        "id": "1",
                        "type": "item_names"
                    },
                    {
                        "id": "2",
                        "type": "item_names"
                    },
                    {
                        "id": "3",
                        "type": "item_names"
                    },
                    {
                        "id": "4",
                        "type": "item_names"
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/marts/items/0")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_mart_items_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "mart_items",
            "attributes": {
                "mart_items": [
                    {
                        "id": "1",
                        "type": "item_names"
                    },
                    {
                        "id": "2",
                        "type": "item_names"
                    },
                    {
                        "id": "3",
                        "type": "item_names"
                    },
                    {
                        "id": "4",
                        "type": "item_names"
                    }
                ]
            }
        }
    });

    let request = client
        .post("/v1/marts/items/100")
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
                "message": "Invalid mart ID 100: valid range is 0-15"
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
