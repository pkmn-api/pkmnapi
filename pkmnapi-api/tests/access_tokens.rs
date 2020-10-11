use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(post_access_token_201, (client) {
    let request_body = json!({
        "data": {
            "type": "access_tokens",
            "attributes": {
                "email_address": "foo@bar.com"
            }
        }
    });

    let request = client
        .post("/v1/access_tokens")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    assert_eq!(response_body.len(), 66);
    assert_eq!(response.status(), Status::Created);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Location", "http://localhost:8080/v1/access_tokens"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_access_token_400, (client) {
    let request_body = json!({
        "data": {
            "type": "access_tokens",
            "attributes": {
                "email_address": "foo"
            }
        }
    });
    let request = client
        .post("/v1/access_tokens")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_access_tokens_invalid",
            "type": "errors",
            "attributes": {
                "message": "Invalid email address: foo"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::BadRequest);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_access_token_403_authorization, (client) {
    let request_body = json!({
        "data": {
            "type": "access_tokens",
            "attributes": {
                "email_address": "foo@bar.com"
            }
        }
    });

    let request = client
        .post("/v1/access_tokens")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&"foo".to_owned()));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_access_tokens_forbidden",
            "type": "errors",
            "attributes": {
                "message": "Authorization header must not be set"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Forbidden);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_access_token_403_timeout, (client) {
    let request_body = json!({
        "data": {
            "type": "access_tokens",
            "attributes": {
                "email_address": "foo@bar.com"
            }
        }
    });

    client
        .post("/v1/access_tokens")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .dispatch();

    let request = client
        .post("/v1/access_tokens")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_access_tokens_timeout",
            "type": "errors",
            "attributes": {
                "message": "Please try again in 600 seconds"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Forbidden);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});
