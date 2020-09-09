use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_move_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/moves/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"id":"1","type":"moves","attributes":{"name":"POUND"},"links":{"self":"http://localhost:8080/v1/moves/1"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn get_move_401() {
    let client = common::setup();

    let request = client.get("/v1/moves/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_move_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/moves/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_moves","type":"errors","attributes":{"message":"Invalid move ID: 200"}}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn post_moves_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/moves/1")
        .body(r#"{"data":{"type":"moves","attributes":{"name":"TESTS"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_string()));

    let request = client
        .get("/v1/moves/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"id":"1","type":"moves","attributes":{"name":"TESTS"},"links":{"self":"http://localhost:8080/v1/moves/1"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn post_moves_401() {
    let client = common::setup();

    let request = client
        .post("/v1/moves/1")
        .body(r#"{"data":{"type":"moves","attributes":{"name":"TESTS"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_moves_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/moves/200")
        .body(r#"{"data":{"type":"moves","attributes":{"name":"TESTS"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_moves","type":"errors","attributes":{"message":"Invalid move ID: 200"}}}"#
                .to_string()
        )
    );

    common::teardown();
}
