use rocket::http::{Accept, ContentType, Status};

mod common;

#[test]
fn get_map_pic_png_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/maps/pics/1")
        .header(common::auth_header(&access_token));

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::PNG));

    common::teardown(&client);
}

#[test]
fn get_map_pic_png_401() {
    let client = common::setup();

    let request = client.get("/v1/maps/pics/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_map_pic_png_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/maps/pics/255")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_map_pics","type":"errors","attributes":{"message":"Invalid map ID 255: valid range is 0-247"}}}"#
                .to_string()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_map_pic_jpeg_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/maps/pics/1")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JPEG));

    common::teardown(&client);
}

#[test]
fn get_map_pic_jpeg_401() {
    let client = common::setup();

    let request = client.get("/v1/maps/pics/1").header(Accept::JPEG);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_map_pic_jpeg_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/maps/pics/255")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_map_pics","type":"errors","attributes":{"message":"Invalid map ID 255: valid range is 0-247"}}}"#
                .to_string()
        )
    );

    common::teardown(&client);
}
