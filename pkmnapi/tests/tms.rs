use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_tm_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/tms/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"tms","attributes":{"move":{"id":"5","type":"moves","attributes":{"name":"MEGA PUNCH"},"links":{"self":"http://localhost:8080/v1/moves/5"}}},"links":{"self":"http://localhost:8080/v1/tms/1"}},"links":{"self":"http://localhost:8080/v1/tms/1"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn get_tm_401() {
    let client = common::setup();

    let request = client.get("/v1/tms/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_tm_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/tms/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_tms","type":"errors","attributes":{"message":"Invalid ID: valid range is 1-50"}}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn post_tms_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/tms/1")
        .body(r#"{"data":{"type":"tms","attributes":{"move":{"id":"1"}}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_string()));

    let request = client
        .get("/v1/tms/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"tms","attributes":{"move":{"id":"1","type":"moves","attributes":{"name":"POUND"},"links":{"self":"http://localhost:8080/v1/moves/1"}}},"links":{"self":"http://localhost:8080/v1/tms/1"}},"links":{"self":"http://localhost:8080/v1/tms/1"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn post_tms_401() {
    let client = common::setup();

    let request = client
        .post("/v1/tms/1")
        .body(r#"{"data":{"type":"tms","attributes":{"name":"TESTS"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_tms_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/tms/200")
        .body(r#"{"data":{"type":"tms","attributes":{"move":{"id":"1"}}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_tms","type":"errors","attributes":{"message":"Invalid ID: valid range is 1-50"}}}"#
                .to_string()
        )
    );

    common::teardown();
}
