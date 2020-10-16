use rocket::http::{Accept, Status};
use serde_json::json;
use std::fs;

mod common;

test!(get_trainer_pic_png_200, (client, access_token) {
    let request = client
        .get("/v1/trainers/pics/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/trainer_pic/1.png").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"YOUNGSTER.png\""),
        ("Content-Type", "image/png"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_trainer_pic_png_401, (client) {
    let request = client.get("/v1/trainers/pics/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_trainer_pic_png_404, (client, access_token) {
    let request = client
        .get("/v1/trainers/pics/100")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid trainer ID 100: valid range is 1-47"
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

test!(get_trainer_pic_jpeg_200, (client, access_token) {
    let request = client
        .get("/v1/trainers/pics/1")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/trainer_pic/1.jpg").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"YOUNGSTER.jpg\""),
        ("Content-Type", "image/jpeg"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_trainer_pic_jpeg_401, (client) {
    let request = client.get("/v1/trainers/pics/1").header(Accept::JPEG);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_trainer_pic_jpeg_404, (client, access_token) {
    let request = client
        .get("/v1/trainers/pics/100")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid trainer ID 100: valid range is 1-47"
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
