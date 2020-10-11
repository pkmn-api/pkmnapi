use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_tm_price_200, (client, access_token) {
    let request = client
        .get("/v1/tms/prices/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "tm_prices",
            "attributes": {
                "price": 3000
            },
            "links": {
                "self": "http://localhost:8080/v1/tms/prices/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/tms/prices/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_tm_price_401, (client) {
    let request = client.get("/v1/tms/prices/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_tm_price_404, (client, access_token) {
    let request = client
        .get("/v1/tms/prices/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_tm_prices",
            "type": "errors",
            "attributes": {
                "message": "Invalid TM ID 200: valid range is 1-50"
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

test!(post_tm_price_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "tm_prices",
            "attributes": {
                "price": 1337
            }
        }
    });

    let request = client
        .post("/v1/tms/prices/1")
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
        .get("/v1/tms/prices/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "tm_prices",
            "attributes": {
                "price": 1000
            },
            "links": {
                "self": "http://localhost:8080/v1/tms/prices/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/tms/prices/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_tm_price_401, (client) {
    let request_body = json!({
        "data": {
            "type": "tm_prices",
            "attributes": {
                "price": 1337
            }
        }
    });

    let request = client
        .post("/v1/tms/prices/1")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_tm_price_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "tm_prices",
            "attributes": {
                "price": 1337
            }
        }
    });

    let request = client
        .post("/v1/tms/prices/200")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_tm_prices",
            "type": "errors",
            "attributes": {
                "message": "Invalid TM ID 200: valid range is 1-50"
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
