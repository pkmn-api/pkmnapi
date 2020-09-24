use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_type_name_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/types/names/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"0","type":"type_names","attributes":{"name":"NORMAL"},"links":{"self":"http://localhost:8080/v1/types/names/0"}},"links":{"self":"http://localhost:8080/v1/types/names/0"}}"#
                .to_owned()
        )
    );

    common::teardown();
}

#[test]
fn get_type_name_401() {
    let client = common::setup();

    let request = client.get("/v1/types/names/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_type_name_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/types/names/100")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_type_names","type":"errors","attributes":{"message":"Invalid type ID 100: valid range is 0-26"}}}"#
                .to_owned()
        )
    );

    common::teardown();
}

#[test]
fn post_type_name_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/types/names/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"0","type":"type_names","attributes":{"name":"BORING"},"links":{"self":"http://localhost:8080/v1/types/names/0"}},"links":{"self":"http://localhost:8080/v1/types/names/0"}}"#
                .to_owned()
        )
    );

    common::teardown();
}

#[test]
fn post_type_name_401() {
    let client = common::setup();

    let request = client
        .post("/v1/types/names/0")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_type_name_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/types/names/100")
        .body(r#"{"data":{"type":"type_names","attributes":{"name":"BORING"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_type_names","type":"errors","attributes":{"message":"Invalid type ID 100: valid range is 0-26"}}}"#
                .to_owned()
        )
    );

    common::teardown();
}
