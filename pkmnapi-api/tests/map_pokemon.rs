use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_map_pokemon_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/maps/pokemon/12")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"12","type":"map_pokemon","attributes":{"grass":{"encounter_rate":25,"pokemon":[{"level":3,"pokemon":{"id":"16","type":"pokemon_names","attributes":{"name":"PIDGEY"},"links":{"self":"http://localhost:8080/v1/pokemon/names/16"}}},{"level":3,"pokemon":{"id":"19","type":"pokemon_names","attributes":{"name":"RATTATA"},"links":{"self":"http://localhost:8080/v1/pokemon/names/19"}}},{"level":3,"pokemon":{"id":"19","type":"pokemon_names","attributes":{"name":"RATTATA"},"links":{"self":"http://localhost:8080/v1/pokemon/names/19"}}},{"level":2,"pokemon":{"id":"19","type":"pokemon_names","attributes":{"name":"RATTATA"},"links":{"self":"http://localhost:8080/v1/pokemon/names/19"}}},{"level":2,"pokemon":{"id":"16","type":"pokemon_names","attributes":{"name":"PIDGEY"},"links":{"self":"http://localhost:8080/v1/pokemon/names/16"}}},{"level":3,"pokemon":{"id":"16","type":"pokemon_names","attributes":{"name":"PIDGEY"},"links":{"self":"http://localhost:8080/v1/pokemon/names/16"}}},{"level":3,"pokemon":{"id":"16","type":"pokemon_names","attributes":{"name":"PIDGEY"},"links":{"self":"http://localhost:8080/v1/pokemon/names/16"}}},{"level":4,"pokemon":{"id":"19","type":"pokemon_names","attributes":{"name":"RATTATA"},"links":{"self":"http://localhost:8080/v1/pokemon/names/19"}}},{"level":4,"pokemon":{"id":"16","type":"pokemon_names","attributes":{"name":"PIDGEY"},"links":{"self":"http://localhost:8080/v1/pokemon/names/16"}}},{"level":5,"pokemon":{"id":"16","type":"pokemon_names","attributes":{"name":"PIDGEY"},"links":{"self":"http://localhost:8080/v1/pokemon/names/16"}}}]},"water":{"encounter_rate":0,"pokemon":[]}},"links":{"self":"http://localhost:8080/v1/maps/pokemon/12"}},"links":{"self":"http://localhost:8080/v1/maps/pokemon/12"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_map_pokemon_401() {
    let client = common::setup();

    let request = client.get("/v1/maps/pokemon/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_map_pokemon_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/maps/pokemon/255")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_map_pokemon","type":"errors","attributes":{"message":"Invalid map ID 255: valid range is 0-247"}}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_map_pokemon_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/maps/pokemon/12")
        .body(r#"{"data":{"type":"map_pokemon","attributes":{"grass":{"encounter_rate":25,"pokemon":[{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}}]},"water":{"encounter_rate":0,"pokemon":[]}}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/maps/pokemon/12")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"12","type":"map_pokemon","attributes":{"grass":{"encounter_rate":25,"pokemon":[{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}},{"level":10,"pokemon":{"id":"25","type":"pokemon_names","attributes":{"name":"PIKACHU"},"links":{"self":"http://localhost:8080/v1/pokemon/names/25"}}}]},"water":{"encounter_rate":0,"pokemon":[]}},"links":{"self":"http://localhost:8080/v1/maps/pokemon/12"}},"links":{"self":"http://localhost:8080/v1/maps/pokemon/12"}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}

#[test]
fn post_map_pokemon_401() {
    let client = common::setup();

    let request = client
        .post("/v1/maps/pokemon/12")
        .body(r#"{"data":{"type":"map_pokemon","attributes":{"grass":{"encounter_rate":25,"pokemon":[{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}}]},"water":{"encounter_rate":0,"pokemon":[]}}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn post_map_pokemon_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/maps/pokemon/255")
        .body(r#"{"data":{"type":"map_pokemon","attributes":{"grass":{"encounter_rate":25,"pokemon":[{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}},{"level":10,"pokemon":{"id":"25"}}]},"water":{"encounter_rate":0,"pokemon":[]}}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_map_pokemon","type":"errors","attributes":{"message":"Invalid map ID 255: valid range is 0-247"}}}"#
                .to_owned()
        )
    );

    common::teardown(&client);
}
