use rocket::http::{Accept, ContentType, Status};
use serde_json::json;
use std::fs;

mod common;

test!(get_game_boy_png_200, (client, access_token) {
    let request = client
        .get("/v1/imgs/game_boy")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/game_boy.png").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"game_boy.png\""),
        ("Content-Type", "image/png"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_game_boy_jpeg_200, (client, access_token) {
    let request = client
        .get("/v1/imgs/game_boy")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/game_boy.jpg").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"game_boy.jpg\""),
        ("Content-Type", "image/jpeg"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_logo_png_200, (client, access_token) {
    let request = client
        .get("/v1/imgs/pokemon_logo")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/pokemon_logo.png").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"pokemon_logo.png\""),
        ("Content-Type", "image/png"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_pokemon_logo_jpeg_200, (client, access_token) {
    let request = client
        .get("/v1/imgs/pokemon_logo")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/pokemon_logo.jpg").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"pokemon_logo.jpg\""),
        ("Content-Type", "image/jpeg"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_pokemon_logo_png_202, (client, access_token) {
    let request_body = fs::read("../secrets/data/pokemon_logo.png").unwrap();

    let request = client
        .post("/v1/imgs/pokemon_logo")
        .body(request_body)
        .header(ContentType::PNG)
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
    ])
});

test!(post_pokemon_logo_png_405, (client, access_token) {
    let request_body = vec![0x01];

    let request = client
        .post("/v1/imgs/pokemon_logo")
        .body(request_body)
        .header(ContentType::PNG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Could not read image"
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

test!(post_pokemon_logo_jpeg_202, (client, access_token) {
    let request_body = fs::read("../secrets/data/pokemon_logo.jpg").unwrap();

    let request = client
        .post("/v1/imgs/pokemon_logo")
        .body(request_body)
        .header(ContentType::JPEG)
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
    ])
});

test!(post_pokemon_logo_jpeg_405, (client, access_token) {
    let request_body = vec![0x01];

    let request = client
        .post("/v1/imgs/pokemon_logo")
        .body(request_body)
        .header(ContentType::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Could not read image"
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

test!(get_town_map_png_200, (client, access_token) {
    let request = client
        .get("/v1/imgs/town_map")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/town_map.png").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"town_map.png\""),
        ("Content-Type", "image/png"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_town_map_jpeg_200, (client, access_token) {
    let request = client
        .get("/v1/imgs/town_map")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = fs::read("../secrets/data/town_map.jpg").unwrap();

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"town_map.jpg\""),
        ("Content-Type", "image/jpeg"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});
