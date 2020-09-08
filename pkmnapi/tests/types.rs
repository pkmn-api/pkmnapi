use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_type_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/types/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"id":"0","type":"types","attributes":{"name":"NORMAL"},"links":{"self":"http://localhost:8080/v1/types/0"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn get_type_401() {
    let client = common::setup();

    let request = client.get("/v1/types/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_type_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/types/0")
        .body(r#"{"data":{"type":"types","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_string()));

    let request = client
        .get("/v1/types/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"id":"0","type":"types","attributes":{"name":"BORING"},"links":{"self":"http://localhost:8080/v1/types/0"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn post_type_401() {
    let client = common::setup();

    let request = client
        .post("/v1/types/0")
        .body(r#"{"data":{"type":"types","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}
