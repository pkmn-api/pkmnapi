use rocket::http::{ContentType, Status};

mod common;

#[test]
fn post_access_token_201() {
    let client = common::setup();

    let request = client
        .post("/v1/access_tokens")
        .body(r#"{"data":{"type":"access_tokens","attributes":{"email_address":"foo@bar.com"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();

    assert_eq!(body.len(), 66);

    let headers = response.headers();

    assert_eq!(
        headers.get("Location").next(),
        Some("http://localhost:8080/v1/access_tokens")
    );

    common::teardown();
}

#[test]
fn post_access_token_400() {
    let client = common::setup();

    let request = client
        .post("/v1/access_tokens")
        .body(r#"{"data":{"type":"access_tokens","attributes":{"email_address":"foo"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_access_tokens_invalid","type":"errors","attributes":{"message":"Invalid email address: foo"}}}"#.to_owned()));

    common::teardown();
}

#[test]
fn post_access_token_403_authorization() {
    let client = common::setup();

    let request = client
        .post("/v1/access_tokens")
        .body(r#"{"data":{"type":"access_tokens","attributes":{"email_address":"foo@bar.com"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&"foo".to_owned()));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_access_tokens_forbidden","type":"errors","attributes":{"message":"Authorization header must not be set"}}}"#.to_owned()));

    common::teardown();
}

#[test]
fn post_access_token_403_timeout() {
    let client = common::setup();

    client
        .post("/v1/access_tokens")
        .body(r#"{"data":{"type":"access_tokens","attributes":{"email_address":"foo@bar.com"}}}"#)
        .header(ContentType::JSON)
        .dispatch();

    let request = client
        .post("/v1/access_tokens")
        .body(r#"{"data":{"type":"access_tokens","attributes":{"email_address":"foo@bar.com"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_access_tokens_timeout","type":"errors","attributes":{"message":"Please try again in 600 seconds"}}}"#.to_owned()));

    common::teardown();
}
