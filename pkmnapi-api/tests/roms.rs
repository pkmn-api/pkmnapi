use regex::Regex;
use rocket::http::{Header, Status};
use serde_json::json;

mod common;

test!(post_rom_201, () {
    let (client, access_token) = common::setup_with_access_token();

    let request_body = common::load_rom();

    let request = client
        .post("/v1/roms")
        .body(request_body)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let response_body = Regex::new(r"[a-zA-Z0-9]{32}").unwrap().replace(response_body.as_str(), "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            "type": "roms",
            "attributes": {
                "name": "POKEMON RED",
                "hash": "3d45c1ee9abd5738df46d2bdda8b57dc",
                "valid": true
            },
            "links": {
                "self": "http://localhost:8080/v1/roms"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/roms"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Created);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("ETag", ""),
        ("Location", "http://localhost:8080/v1/roms"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_rom_400_invalid_rom, () {
    let (client, access_token) = common::setup_with_access_token();

    let request_body = "";

    let request = client
        .post("/v1/roms")
        .body(request_body)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_roms_invalid_rom",
            "type": "errors",
            "attributes": {
                "message": "Invalid ROM provided"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::BadRequest);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_rom_401, (client) {
    let request_body = "";

    let request = client.post("/v1/roms").body(request_body);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_rom_403_rom_exists, () {
    let (client, access_token) = common::setup_with_access_token();

    let request_body = common::load_rom();

    client
        .post("/v1/roms")
        .body(&request_body)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .post("/v1/roms")
        .body(&request_body)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_roms_rom_exists",
            "type": "errors",
            "attributes": {
                "message": "ROM already exists"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Forbidden);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_rom_200, (client, access_token) {
    let request = client
        .get("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let response_body = Regex::new(r"[a-zA-Z0-9]{32}").unwrap().replace(response_body.as_str(), "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            "type": "roms",
            "attributes": {
                "name": "POKEMON RED",
                "hash": "3d45c1ee9abd5738df46d2bdda8b57dc",
                "valid": true
            },
            "links": {
                "self": "http://localhost:8080/v1/roms"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/roms"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("ETag", ""),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_rom_401, (client) {
    let request = client.get("/v1/roms");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_rom_403_no_rom, () {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .get("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_roms_no_rom",
            "type": "errors",
            "attributes": {
                "message": "No ROM uploaded"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Forbidden);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(delete_rom_204, (client, access_token) {
    let request = client
        .get("/v1/roms")
        .header(common::auth_header(&access_token));

    let response = request.dispatch();
    let headers = response.headers();

    let etag = headers.get("ETag").next().unwrap().to_owned();

    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", etag));

    let mut response = request.dispatch();
    let response_body = response.body_string();
    let headers = response.headers();

    assert_eq!(response_body, None);
    assert_eq!(response.status(), Status::NoContent);

    common::assert_headers(headers, vec![
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(delete_rom_400, (client, access_token) {
    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_etag_mismatch",
            "type": "errors",
            "attributes": {
                "message": "ETag mismatch"
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

test!(delete_rom_401, (client) {
    let request = client.delete("/v1/roms");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(delete_rom_403_no_rom, () {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_roms_no_rom",
            "type": "errors",
            "attributes": {
                "message": "No ROM uploaded"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Forbidden);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(delete_rom_403_etag, (client, access_token) {
    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_etag_missing",
            "type": "errors",
            "attributes": {
                "message": "If-Match header must be set"
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
