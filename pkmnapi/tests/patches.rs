use rocket::http::{Accept, ContentType, Header, MediaType, Status};

mod common;

#[test]
fn get_patches_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(r#"{"data":[],"links":{"self":"foo"}}"#.to_string())
    );

    client
        .post("/v1/types/0")
        .body(r#"{"data":{"type":"types","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let body_a = (&body[..15]).to_string();
    let body_b = (&body[15..49]).to_string();
    let body_c = (&body[49..]).to_string();

    assert_eq!(body_a, r#"{"data":[{"id":"#);
    assert_eq!(body_b.len(), 34);
    assert_eq!(
        body_c,
        r#","type":"patches","attributes":{},"links":{"self":"foo"}}],"links":{"self":"foo"}}"#
    );

    common::teardown();
}

#[test]
fn get_patches_401() {
    let client = common::setup();

    let request = client.get("/v1/patches");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_patches_raw_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    client
        .post("/v1/types/0")
        .body(r#"{"data":{"type":"types","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .get("/v1/patches")
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

    common::teardown();
}

#[test]
fn get_patches_raw_401() {
    let client = common::setup();

    let request =
        client.get("/v1/patches").header(Accept::new(vec![
            MediaType::new("application", "patch").into()
        ]));

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_patch_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    client
        .post("/v1/types/0")
        .body(r#"{"data":{"type":"types","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token))
        .header(Header::new("X-Patch-Description", "NORMAL -> BORING"))
        .dispatch();

    let request = client
        .get("/v1/patches")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let patch_id = (&body[16..48]).to_string();

    let request = client
        .get(format!("/v1/patches/{}", patch_id))
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    let body = response.body_string().unwrap();
    let body_a = (&body[..6]).to_string();
    let body_b = (&body[6..40]).to_string();
    let body_c = (&body[40..]).to_string();

    assert_eq!(body_a, r#"{"id":"#);
    assert_eq!(body_b.len(), 34);
    assert_eq!(
        body_c,
        r#","type":"patches","attributes":{"description":"NORMAL -> BORING"},"links":{"self":"foo"}}"#
    );

    common::teardown();
}

#[test]
fn get_patch_401() {
    let client = common::setup();

    let request = client.get("/v1/patches/abcdefgh");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_patch_404() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .get("/v1/patches/abcdefgh")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_patches","type":"errors","attributes":{"message":"No patch found"}}}"#.to_string()));

    common::teardown();
}
