use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_pokemon_name_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/names/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"pokemon_names","attributes":{"name":"BULBASAUR"},"links":{"self":"http://localhost:8080/v1/pokemon/names/1"}},"links":{"self":"http://localhost:8080/v1/pokemon/names/1"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn get_pokemon_name_401() {
    let client = common::setup();

    let request = client.get("/v1/pokemon/names/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_pokemon_name_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/names/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokemon_names","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn post_pokemon_name_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokemon/names/1")
        .body(r#"{"data":{"type":"pokemon_names","attributes":{"name":"DINOSAUR"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_string()));

    let request = client
        .get("/v1/pokemon/names/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"pokemon_names","attributes":{"name":"DINOSAUR"},"links":{"self":"http://localhost:8080/v1/pokemon/names/1"}},"links":{"self":"http://localhost:8080/v1/pokemon/names/1"}}"#
                .to_string()
        )
    );

    common::teardown();
}

#[test]
fn post_pokemon_name_401() {
    let client = common::setup();

    let request = client
        .post("/v1/pokemon/names/1")
        .body(r#"{"data":{"type":"pokemon_names","attributes":{"name":"DINOSAUR"}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_pokemon_name_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokemon/names/200")
        .body(r#"{"data":{"type":"pokemon_names","attributes":{"name":"DINOSAUR"}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokemon_names","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_string()
        )
    );

    common::teardown();
}
