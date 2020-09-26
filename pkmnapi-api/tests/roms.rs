use rocket::http::{ContentType, Header, Status};

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
    let body_a = (&body[..15]).to_string();
    let body_b = (&body[15..47]).to_string();
    let body_c = (&body[47..]).to_string();

    assert_eq!(body_a, r#"{"data":{"id":""#);
    assert_eq!(body_b.len(), 32);
    assert_eq!(
        body_c,
        format!(
            r#"","type":"roms","attributes":{{"name":"POKEMON RED","hash":"3d45c1ee9abd5738df46d2bdda8b57dc","valid":true}},"links":{{"self":"http://localhost:8080/v1/roms/{}"}}}},"links":{{"self":"http://localhost:8080/v1/roms/{}"}}}}"#,
            body_b, body_b
        )
    );

    let headers = response.headers();

    assert_eq!(
        headers.get("Location").next(),
        Some("http://localhost:8080/v1/roms")
    );

    common::teardown(&client);
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

    common::teardown(&client);
}

#[test]
fn post_rom_401() {
    let client = common::setup();

    let request = client.post("/v1/roms").body("");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
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

    common::teardown(&client);
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
    let body_a = (&body[..15]).to_string();
    let body_b = (&body[15..47]).to_string();
    let body_c = (&body[47..]).to_string();

    assert_eq!(body_a, r#"{"data":{"id":""#);
    assert_eq!(body_b.len(), 32);
    assert_eq!(
        body_c,
        format!(
            r#"","type":"roms","attributes":{{"name":"POKEMON RED","hash":"3d45c1ee9abd5738df46d2bdda8b57dc","valid":true}},"links":{{"self":"http://localhost:8080/v1/roms/{}"}}}},"links":{{"self":"http://localhost:8080/v1/roms/{}"}}}}"#,
            body_b, body_b
        )
    );

    common::teardown(&client);
}

#[test]
fn get_rom_401() {
    let client = common::setup();

    let request = client.get("/v1/roms");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
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

    common::teardown(&client);
}

#[test]
fn delete_rom_204() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

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

    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(response.content_type(), None);
    assert_eq!(response.body_string(), None);

    common::teardown(&client);
}

#[test]
fn delete_rom_400() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_etag_mismatch","type":"errors","attributes":{"message":"ETag mismatch"}}}"#.to_string()));

    common::teardown(&client);
}

#[test]
fn delete_rom_401() {
    let client = common::setup();

    let request = client.delete("/v1/roms");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn delete_rom_403_no_rom() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_roms_no_rom","type":"errors","attributes":{"message":"No ROM uploaded"}}}"#.to_string()));

    common::teardown(&client);
}

#[test]
fn delete_rom_403_etag() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .delete("/v1/roms")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_etag_missing","type":"errors","attributes":{"message":"If-Match header must be set"}}}"#.to_string()));

    common::teardown(&client);
}
