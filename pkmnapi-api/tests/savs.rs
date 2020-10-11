use regex::Regex;
use rocket::http::{Header, Status};
use serde_json::json;

mod common;

test!(post_sav_201, () {
    let (client, access_token) = common::setup_with_access_token();

    let request_body = common::load_sav();

    let request = client
        .post("/v1/savs")
        .body(request_body)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let response_body = Regex::new(r"[a-zA-Z0-9]{32}").unwrap().replace_all(response_body.as_str(), "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            "type": "savs",
            "attributes": {},
            "links": {
                "self": "http://localhost:8080/v1/savs/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/savs/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Created);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("ETag", ""),
        ("Location", "http://localhost:8080/v1/savs"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_sav_400_invalid_sav, () {
    let (client, access_token) = common::setup_with_access_token();

    let request_body = "";

    let request = client
        .post("/v1/savs")
        .body(request_body)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_savs_invalid_sav",
            "type": "errors",
            "attributes": {
                "message": "Invalid SAV provided"
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

test!(post_sav_401, (client) {
    let request_body = "";

    let request = client.post("/v1/savs").body(request_body);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_sav_403_sav_exists, () {
    let (client, access_token) = common::setup_with_access_token();

    let request_body = common::load_sav();

    client
        .post("/v1/savs")
        .body(&request_body)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .post("/v1/savs")
        .body(&request_body)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_savs_sav_exists",
            "type": "errors",
            "attributes": {
                "message": "SAV already exists"
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

test!(get_sav_200, (client, access_token) {
    let request = client
        .get("/v1/savs")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let response_body = Regex::new(r"[a-zA-Z0-9]{32}").unwrap().replace_all(response_body.as_str(), "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            "type": "savs",
            "attributes": {},
            "links": {
                "self": "http://localhost:8080/v1/savs/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/savs/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("ETag", ""),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_sav_401, (client) {
    let request = client.get("/v1/savs");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_sav_403_no_sav, () {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .get("/v1/savs")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_savs_no_sav",
            "type": "errors",
            "attributes": {
                "message": "No SAV uploaded"
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

test!(delete_sav_204, (client, access_token) {
    let request = client
        .get("/v1/savs")
        .header(common::auth_header(&access_token));

    let response = request.dispatch();
    let headers = response.headers();

    let etag = headers.get("ETag").next().unwrap().to_owned();

    let request = client
        .delete("/v1/savs")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", etag));

    let mut response = request.dispatch();
    let response_body = response.body_string();
    let headers = response.headers();

    assert_eq!(response_body, None);
    assert_eq!(response.status(), Status::NoContent);

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(delete_sav_400, (client, access_token) {
    let request = client
        .delete("/v1/savs")
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

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(delete_sav_401, (client) {
    let request = client.delete("/v1/savs");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(delete_sav_403_no_sav, () {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .delete("/v1/savs")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_savs_no_sav",
            "type": "errors",
            "attributes": {
                "message": "No SAV uploaded"
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
        .delete("/v1/savs")
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

    common::teardown(&client);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});
