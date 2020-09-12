use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_stats_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/stats/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"stats","attributes":{"base_hp":45,"base_attack":49,"base_defence":49,"base_speed":45,"base_special":65,"types":[{"id":"22","type":"types","attributes":{"name":"GRASS"},"links":{"self":"http://localhost:8080/v1/types/22"}},{"id":"3","type":"types","attributes":{"name":"POISON"},"links":{"self":"http://localhost:8080/v1/types/3"}}],"catch_rate":45,"base_exp_yield":64},"links":{"self":"http://localhost:8080/v1/stats/1"}},"links":{"self":"http://localhost:8080/v1/stats/1"}}"#
                .to_owned()
        )
    );

    common::teardown();
}

#[test]
fn get_stats_401() {
    let client = common::setup();

    let request = client.get("/v1/stats/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn get_stats_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/stats/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_stats","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_owned()
        )
    );

    common::teardown();
}

#[test]
fn post_stats_202() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/stats/1")
        .body(r#"{"data":{"type":"stats","attributes":{"base_hp":42,"base_attack":42,"base_defence":42,"base_speed":42,"base_special":42,"types":[{"id":"0"},{"id":"0"}],"catch_rate":42,"base_exp_yield":42}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{}".to_owned()));

    let request = client
        .get("/v1/stats/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"1","type":"stats","attributes":{"base_hp":42,"base_attack":42,"base_defence":42,"base_speed":42,"base_special":42,"types":[{"id":"0","type":"types","attributes":{"name":"NORMAL"},"links":{"self":"http://localhost:8080/v1/types/0"}},{"id":"0","type":"types","attributes":{"name":"NORMAL"},"links":{"self":"http://localhost:8080/v1/types/0"}}],"catch_rate":42,"base_exp_yield":42},"links":{"self":"http://localhost:8080/v1/stats/1"}},"links":{"self":"http://localhost:8080/v1/stats/1"}}"#
                .to_owned()
        )
    );

    common::teardown();
}

#[test]
fn post_stats_401() {
    let client = common::setup();

    let request = client
        .post("/v1/stats/1")
        .body(r#"{"data":{"type":"stats","attributes":{"base_hp":42,"base_attack":42,"base_defence":42,"base_speed":42,"base_special":42,"types":[{"id":"0"},{"id":"0"}],"catch_rate":42,"base_exp_yield":42}}}"#)
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown();
}

#[test]
fn post_stats_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .post("/v1/stats/200")
        .body(r#"{"data":{"type":"stats","attributes":{"base_hp":42,"base_attack":42,"base_defence":42,"base_speed":42,"base_special":42,"types":[{"id":"0"},{"id":"0"}],"catch_rate":42,"base_exp_yield":42}}}"#)
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_stats","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_owned()
        )
    );

    common::teardown();
}
