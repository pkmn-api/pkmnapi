use rocket::http::Status;
use serde_json::json;
use std::fs;

mod common;

test!(get_icon_200, (client, access_token) {
    let request = client
        .get("/v1/icons/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("tests/data/icon-0.gif").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"icon-0.gif\""),
        ("Content-Type", "image/gif"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_icon_401, (client) {
    let request = client.get("/v1/icons/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_icon_404, (client, access_token) {
    let request = client
        .get("/v1/icons/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_icons",
            "type": "errors",
            "attributes": {
                "message": "Invalid icon ID 200: valid range is 0-9"
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
