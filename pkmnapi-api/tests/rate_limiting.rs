use regex::Regex;
use rocket::http::Status;
use serde_json::json;

mod common;

test!(rate_limiting, (client, access_token) {
    for _ in 1..=120 {
        client
            .get("/v1/types/names/1")
            .header(common::auth_header(&access_token))
            .dispatch();
    }

    let request = client
        .get("/v1/types/names/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let response_body = Regex::new(r"\d+").unwrap().replace_all(response_body.as_str(), "XX");
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_too_many_requests",
            "type": "errors",
            "attributes": {
                "message": "Too many requests. Please try again in XX seconds."
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::TooManyRequests);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});
