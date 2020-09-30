use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_pokemon_evolutions_200_level() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/evolutions/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"level","level":16,"pokemon":{"id":"2","type":"pokemon_names","attributes":{"name":"IVYSAUR"},"links":{"self":"http://localhost:8080/v1/pokemon/names/2"}}}]},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/1"}},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/1"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_pokemon_evolutions_200_item() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/evolutions/25")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"25","type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"item","item":{"id":"33","type":"item_names","attributes":{"name":"THUNDERSTONE"},"links":{"self":"http://localhost:8080/v1/items/names/33"}},"pokemon":{"id":"26","type":"pokemon_names","attributes":{"name":"RAICHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/26"}}}]},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/25"}},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/25"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_pokemon_evolutions_200_trade() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/evolutions/64")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"64","type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"trade","pokemon":{"id":"65","type":"pokemon_names","attributes":{"name":"ALAKAZAM"},"links":{"self":"http://localhost:8080/v1/pokemon/names/65"}}}]},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/64"}},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/64"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_pokemon_evolutions_401() {
    let client = common::setup();

    let request = client.get("/v1/pokemon/evolutions/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_pokemon_evolutions_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/evolutions/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokemon_evolutions","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_pokemon_evolutions_202_level() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokemon/evolutions/1")
        .body(r#"{"data":{"type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"level","level":16,"pokemon":{"id":"4"}}]}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/pokemon/evolutions/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"level","level":16,"pokemon":{"id":"4","type":"pokemon_names","attributes":{"name":"CHARMANDER"},"links":{"self":"http://localhost:8080/v1/pokemon/names/4"}}}]},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/1"}},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/1"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_pokemon_evolutions_202_item() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokemon/evolutions/25")
        .body(r#"{"data":{"type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"item","item":{"id":"10"},"pokemon":{"id":"4"}}]}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/pokemon/evolutions/25")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"25","type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"item","item":{"id":"10","type":"item_names","attributes":{"name":"MOON STONE"},"links":{"self":"http://localhost:8080/v1/items/names/10"}},"pokemon":{"id":"4","type":"pokemon_names","attributes":{"name":"CHARMANDER"},"links":{"self":"http://localhost:8080/v1/pokemon/names/4"}}}]},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/25"}},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/25"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_pokemon_evolutions_202_trade() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokemon/evolutions/64")
        .body(r#"{"data":{"type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"trade","pokemon":{"id":"4"}}]}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/pokemon/evolutions/64")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"64","type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"trade","pokemon":{"id":"4","type":"pokemon_names","attributes":{"name":"CHARMANDER"},"links":{"self":"http://localhost:8080/v1/pokemon/names/4"}}}]},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/64"}},"links":{"self":"http://localhost:8080/v1/pokemon/evolutions/64"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_pokemon_evolutions_401() {
    let client = common::setup();

    let request = client
        .post("/v1/pokemon/evolutions/1")
        .body(r#"{"data":{"type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"level","level":16,"pokemon":{"id":"4"}}]}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn post_pokemon_evolutions_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/pokemon/evolutions/200")
        .body(r#"{"data":{"type":"pokemon_evolutions","attributes":{"evolutions":[{"evolution_type":"level","level":16,"pokemon":{"id":"4"}}]}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokemon_evolutions","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}
