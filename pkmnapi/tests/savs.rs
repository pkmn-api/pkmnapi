use rocket::http::{ContentType, Header, Status};

mod common;

#[test]
fn post_sav_201() {
    let (client, access_token) = common::setup_with_access_token();

    let sav = common::load_sav();

    let request = client
        .post("/v1/savs")
        .body(&sav)
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
            r#"","type":"savs","attributes":{{}},"links":{{"self":"http://localhost:8080/v1/savs/{}"}}}},"links":{{"self":"http://localhost:8080/v1/savs/{}"}}}}"#,
            body_b, body_b
        )
    );

    let headers = response.headers();

    assert_eq!(
        headers.get("Location").next(),
        Some("http://localhost:8080/v1/savs")
    );

    common::teardown();
}

#[test]
fn post_sav_400_invalid_sav() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .post("/v1/savs")
        .body("")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_savs_invalid_sav","type":"errors","attributes":{"message":"Invalid SAV provided"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn post_sav_401() {
    let client = common::setup();

    let request = client.post("/v1/savs").body("");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_sav_403_sav_exists() {
    let (client, access_token) = common::setup_with_access_token();

    let sav = common::load_sav();

    client
        .post("/v1/savs")
        .body(&sav)
        .header(common::auth_header(&access_token))
        .dispatch();

    let request = client
        .post("/v1/savs")
        .body(&sav)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_savs_sav_exists","type":"errors","attributes":{"message":"SAV already exists"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn get_sav_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_sav(&client, &access_token);

    let request = client
        .get("/v1/savs")
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
            r#"","type":"savs","attributes":{{}},"links":{{"self":"http://localhost:8080/v1/savs/{}"}}}},"links":{{"self":"http://localhost:8080/v1/savs/{}"}}}}"#,
            body_b, body_b
        )
    );

    common::teardown();
}

#[test]
fn get_sav_401() {
    let client = common::setup();

    let request = client.get("/v1/savs");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_sav_403_no_sav() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .get("/v1/savs")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_savs_no_sav","type":"errors","attributes":{"message":"No SAV uploaded"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn delete_sav_204() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_sav(&client, &access_token);

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

    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(response.content_type(), None);
    assert_eq!(response.body_string(), None);

    common::teardown();
}

#[test]
fn delete_sav_400() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_sav(&client, &access_token);

    let request = client
        .delete("/v1/savs")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_etag_mismatch","type":"errors","attributes":{"message":"ETag mismatch"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn delete_sav_401() {
    let client = common::setup();

    let request = client.delete("/v1/savs");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn delete_sav_403_no_sav() {
    let (client, access_token) = common::setup_with_access_token();

    let request = client
        .delete("/v1/savs")
        .header(common::auth_header(&access_token))
        .header(Header::new("If-Match", "wrong".to_string()));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_savs_no_sav","type":"errors","attributes":{"message":"No SAV uploaded"}}}"#.to_string()));

    common::teardown();
}

#[test]
fn delete_rom_403_etag() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_sav(&client, &access_token);

    let request = client
        .delete("/v1/savs")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_etag_missing","type":"errors","attributes":{"message":"If-Match header must be set"}}}"#.to_string()));

    common::teardown();
}
