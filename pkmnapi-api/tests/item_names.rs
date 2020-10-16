use pkmnapi_api::responses::item_names::ItemNameResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_item_name_all_200, (client, access_token) {
    let request = client
        .get("/v1/items/names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<ItemNameResponseAll>("../secrets/data/json/get_item_name_all_200.json");

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_item_name_200, (client, access_token) {
    let request = client
        .get("/v1/items/names/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "item_names",
            "attributes": {
                "name": "MASTER BALL"
            },
            "links": {
                "self": "http://localhost:8080/v1/items/names/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/items/names/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_item_name_401, (client) {
    let request = client.get("/v1/items/names/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_item_name_404, (client, access_token) {
    let request = client
        .get("/v1/items/names/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid item ID 200: valid range is 1-97"
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

test!(post_item_name_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "item_names",
            "attributes": {
                "name": "CHEATERBALL"
            }
        }
    });

    let request = client
        .post("/v1/items/names/1")
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
        .get("/v1/items/names/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "item_names",
            "attributes": {
                "name":"CHEATERBALL"
            },
            "links": {
                "self": "http://localhost:8080/v1/items/names/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/items/names/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_item_name_401, (client) {
    let request_data = json!({
        "data": {
            "type": "item_names",
            "attributes": {
                "name": "CHEATERBALL"
            }
        }
    });

    let request = client
        .post("/v1/items/names/1")
        .body(request_data.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_item_name_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "item_names",
            "attributes": {
                "name": "CHEATERBALL"
            }
        }
    });

    let request = client
        .post("/v1/items/names/200")
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
                "message": "Invalid item ID 200: valid range is 1-97"
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
