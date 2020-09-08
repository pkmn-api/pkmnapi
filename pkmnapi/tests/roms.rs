use rocket::http::{ContentType, Status};

mod common;

#[test]
fn post_rom_201() {
    let (client, access_token) = common::setup_with_access_token();

    let rom = common::load_rom();

    let request = client
        .post("/v1/roms")
        .body(&rom)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let body_a = (&body[..6]).to_string();
    let body_b = (&body[6..40]).to_string();
    let body_c = (&body[40..]).to_string();

    assert_eq!(body_a, r#"{"id":"#);
    assert_eq!(body_b.len(), 34);
    assert_eq!(
        body_c,
        r#","type":"roms","attributes":{"name":"POKEMON RED","hash":"3d45c1ee9abd5738df46d2bdda8b57dc","valid":true},"links":{"self":"foo"}}"#
    );

    common::teardown();
}

#[test]
fn post_rom_400_invalid_rom() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .post("/v1/roms")
        .body("")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_roms_invalid_rom","type":"errors","attributes":{"message":"Invalid ROM provided"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn post_rom_401_unauthorized() {
    let client = common::setup();

    let request = client.post("/v1/roms").body("");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_rom_403_rom_exists() {
    let (client, access_token) = common::setup_with_access_token();

    let rom = common::load_rom();

    client
        .post("/v1/roms")
        .body(&rom)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .post("/v1/roms")
        .body(&rom)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_roms_rom_exists","type":"errors","attributes":{"message":"ROM already exists"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn get_rom_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let body_a = (&body[..6]).to_string();
    let body_b = (&body[6..40]).to_string();
    let body_c = (&body[40..]).to_string();

    assert_eq!(body_a, r#"{"id":"#);
    assert_eq!(body_b.len(), 34);
    assert_eq!(
        body_c,
        r#","type":"roms","attributes":{"name":"POKEMON RED","hash":"3d45c1ee9abd5738df46d2bdda8b57dc","valid":true},"links":{"self":"foo"}}"#
    );

    common::teardown();
}

#[test]
fn get_rom_401_unauthorized() {
    let client = common::setup();

    let request = client.get("/v1/roms");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_rom_403_no_rom() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .get("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_roms_no_rom","type":"errors","attributes":{"message":"No ROM uploaded"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn delete_rom_204() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(response.content_type(), None);
    assert_eq!(response.body_string(), None);

    common::teardown();
}

#[test]
fn delete_rom_401_unauthorized() {
    let client = common::setup();

    let request = client.delete("/v1/roms");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn delete_rom_403_no_rom() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_roms_no_rom","type":"errors","attributes":{"message":"No ROM uploaded"}}}"#.to_string()));

    common::teardown();
}