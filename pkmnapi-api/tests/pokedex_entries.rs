use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_pokedex_entry_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokedex/entries/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"pokedex_entries","attributes":{"species":"SEED","height":28,"weight":150},"links":{"self":"http://localhost:8080/v1/pokedex/entries/1"}},"links":{"self":"http://localhost:8080/v1/pokedex/entries/1"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_pokedex_entry_401() {
    let client = common::setup();

    let request = client.get("/v1/pokedex/entries/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_pokedex_entry_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokedex/entries/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokedex_entries","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_pokedex_entry_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokedex/entries/1")
        .body(r#"{"data":{"type":"pokedex_entries","attributes":{"species":"LEAF","height":42,"weight":42}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/pokedex/entries/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"pokedex_entries","attributes":{"species":"LEAF","height":42,"weight":42},"links":{"self":"http://localhost:8080/v1/pokedex/entries/1"}},"links":{"self":"http://localhost:8080/v1/pokedex/entries/1"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_pokedex_entry_401() {
    let client = common::setup();

    let request = client
        .post("/v1/pokedex/entries/1")
        .body(r#"{"data":{"type":"pokedex_entries","attributes":{"species":"LEAF","height":42,"weight":42}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn post_pokedex_entry_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokedex/entries/200")
        .body(r#"{"data":{"type":"pokedex_entries","attributes":{"species":"LEAF","height":42,"weight":42}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokedex_entries","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}
