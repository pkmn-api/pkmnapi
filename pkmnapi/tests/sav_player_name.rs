use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_sav_player_name_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);
    common::post_sav(&client, &access_token);

    let request = client
        .get("/v1/savs/player_names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"666","type":"sav_player_names","attributes":{"name":"RED"},"links":{"self":"http://localhost:8080/v1/savs/player_names"}},"links":{"self":"http://localhost:8080/v1/savs/player_names"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_sav_player_name_401() {
    let client = common::setup();

    let request = client.get("/v1/savs/player_names");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn post_sav_player_name_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);
    common::post_sav(&client, &access_token);

    let request = client
        .post("/v1/savs/player_names")
        .body(r#"{"data":{"type":"sav_player_names","attributes":{"name":"BLUE"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/savs/player_names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"666","type":"sav_player_names","attributes":{"name":"BLUE"},"links":{"self":"http://localhost:8080/v1/savs/player_names"}},"links":{"self":"http://localhost:8080/v1/savs/player_names"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_sav_player_name_401() {
    let client = common::setup();

    let request = client
        .post("/v1/savs/player_names")
        .body(r#"{"data":{"type":"sav_player_names","attributes":{"name":"BLUE"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}
