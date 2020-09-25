use rocket::http::{Accept, ContentType, Header, MediaType, Status};

mod common;

#[test]
fn get_rom_patches_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(r#"{"data":[],"links":{"self":"http://localhost:8080/v1/roms/patches"}}"#.to_string())
    );

    client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let body_a = (&body[..16]).to_string();
    let body_b = (&body[16..48]).to_string();
    let body_c = (&body[48..]).to_string();

    assert_eq!(body_a, r#"{"data":[{"id":""#);
    assert_eq!(body_b.len(), 32);
    assert_eq!(
        body_c,
        format!(
            r#"","type":"rom_patches","attributes":{{}},"links":{{"self":"http://localhost:8080/v1/roms/patches/{}"}}}}],"links":{{"self":"http://localhost:8080/v1/roms/patches"}}}}"#,
            body_b
        )
    );

    common::teardown(&client);
}

#[test]
fn get_rom_patches_401() {
    let client = common::setup();

    let request = client.get("/v1/roms/patches");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_rom_patches_raw_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
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

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.content_type(),
        Some(ContentType::new("application", "patch"))
    );
    assert_eq!(
        response.body_bytes(),
        Some(vec![
            0x50, 0x41, 0x54, 0x43, 0x48, // PATCH
            0x02, 0x7D, 0xE4, 0x00, 0x06, 0x81, 0x8E, 0x91, 0x88, 0x8D, 0x86, // DATA
            0x45, 0x4F, 0x46 // EOF
        ])
    );

    let headers = response.headers();

    assert_eq!(
        headers.get("Content-Disposition").next(),
        Some(r#"attachment; filename="patch.ips""#)
    );

    common::teardown(&client);
}

#[test]
fn get_rom_patches_raw_401() {
    let client = common::setup();

    let request = client
        .get("/v1/roms/patches")
        .header(Accept::new(vec![
            MediaType::new("application", "patch").into()
        ]));

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_rom_patch_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .header(Header::new("X-Patch-Description", "NORMAL -> BORING"))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let patch_id = (&body[16..48]).to_string();

    let request = client
        .get(format!("/v1/roms/patches/{}", patch_id))
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    let body = response.body_string().unwrap();
    let body_a = (&body[..15]).to_string();
    let body_b = (&body[15..47]).to_string();
    let body_c = (&body[47..]).to_string();

    assert_eq!(body_a, r#"{"data":{"id":""#);
    assert_eq!(body_b.len(), 32);
    assert_eq!(
        body_c,
        format!(
            r#"","type":"rom_patches","attributes":{{"description":"NORMAL -> BORING"}},"links":{{"self":"http://localhost:8080/v1/roms/patches/{}"}}}},"links":{{"self":"http://localhost:8080/v1/roms/patches/{}"}}}}"#,
            body_b, body_b
        )
    );

    common::teardown(&client);
}

#[test]
fn get_rom_patch_401() {
    let client = common::setup();

    let request = client.get("/v1/roms/patches/abcdefgh");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_rom_patch_404() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .get("/v1/roms/patches/abcdefgh")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_rom_patches","type":"errors","attributes":{"message":"No ROM patch found"}}}"#.to_string()));

    common::teardown(&client);
}

#[test]
fn delete_rom_patch_204() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let patch_id = (&body[16..48]).to_string();

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

    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(response.content_type(), None);
    assert_eq!(response.body_string(), None);

    common::teardown(&client);
}

#[test]
fn delete_rom_patch_400() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let patch_id = (&body[16..48]).to_string();

    let request = client
        .delete(format!("/v1/roms/patches/{}", patch_id))
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_etag_mismatch","type":"errors","attributes":{"message":"ETag mismatch"}}}"#.to_string()));

    common::teardown(&client);
}

#[test]
fn delete_rom_patch_403() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/roms/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let patch_id = (&body[16..48]).to_string();

    let request = client
        .delete(format!("/v1/roms/patches/{}", patch_id))
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_etag_missing","type":"errors","attributes":{"message":"If-Match header must be set"}}}"#.to_string()));

    common::teardown(&client);
}
