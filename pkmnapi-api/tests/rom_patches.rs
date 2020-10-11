use regex::Regex;
use rocket::http::{Accept, ContentType, Header, MediaType, Status};
use serde_json::json;

mod common;

test!(get_rom_patches_200, (client, access_token) {
    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": [],
        "links": {
            "self": "http://localhost:8080/v1/roms/patches"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ]).unwrap();

    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let response_body = Regex::new(r"[a-zA-Z0-9]{32}").unwrap().replace_all(response_body.as_str(), "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let headers = response.headers();

    let body = json!({
        "data": [
            {
                "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
                "type": "rom_patches",
                "attributes": {},
                "links": {
                    "self": "http://localhost:8080/v1/roms/patches/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                }
            }
        ],
        "links": {
            "self": "http://localhost:8080/v1/roms/patches"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_rom_patches_401, (client) {
    let request = client.get("/v1/roms/patches");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_rom_patches_raw_200, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(Accept::new(vec![
            MediaType::new("application", "patch").into()
        ]))
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_bytes().unwrap();
    let headers = response.headers();

    let body = vec![
        0x50, 0x41, 0x54, 0x43, 0x48, // PATCH
        0x02, 0x7D, 0xE4, 0x00, 0x06, 0x81, 0x8E, 0x91, 0x88, 0x8D, 0x86, 0x00, 0x01, 0x4E,
        0x00, 0x02, 0x91, 0xDE, // DATA
        0x45, 0x4F, 0x46 // EOF
    ];

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Disposition", "attachment; filename=\"patch.ips\""),
        ("Content-Type", "application/patch"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_rom_patches_raw_401, (client) {
    let request = client
        .get("/v1/roms/patches")
        .header(Accept::new(vec![
            MediaType::new("application", "patch").into()
        ]));

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_rom_patch_200, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .header(Header::new("X-Patch-Description", "NORMAL -> BORING"))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let patch_id = (&response_body[16..48]).to_string();

    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ]).unwrap();

    let request = client
        .get(format!("/v1/roms/patches/{}", patch_id))
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let response_body = Regex::new(r"[a-zA-Z0-9]{32}").unwrap().replace_all(response_body.as_str(), "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            "type": "rom_patches",
            "attributes": {
                "description": "NORMAL -> BORING"
            },
            "links": {
                "self": "http://localhost:8080/v1/roms/patches/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/roms/patches/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
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

test!(get_rom_patch_401, (client) {
    let request = client.get("/v1/roms/patches/abcdefgh");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_rom_patch_404, (client, access_token) {
    let request = client
        .get("/v1/roms/patches/abcdefgh")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_rom_patches",
            "type": "errors",
            "attributes": {
                "message": "No ROM patch found"
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

test!(delete_rom_patch_204, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let patch_id = (&response_body[16..48]).to_string();

    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ]).unwrap();

    let request = client
        .get(format!("/v1/roms/patches/{}", patch_id))
        .header(common::auth_header(&access_token));

    let response = request.dispatch();
    let headers = response.headers();

    let etag = headers.get("ETag").next().unwrap().to_owned();

    let request = client
        .delete(format!("/v1/roms/patches/{}", patch_id))
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

test!(delete_rom_patch_400, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let patch_id = (&response_body[16..48]).to_string();

    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ]).unwrap();

    let request = client
        .delete(format!("/v1/roms/patches/{}", patch_id))
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

test!(delete_rom_patch_403, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let patch_id = (&response_body[16..48]).to_string();

    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ]).unwrap();

    let request = client
        .delete(format!("/v1/roms/patches/{}", patch_id))
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
