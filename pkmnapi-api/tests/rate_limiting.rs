use rocket::http::{ContentType, Status};

mod common;

#[test]
fn rate_limiting() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    for _ in 1..=120 {
        client
            .get("/v1/types/names/1")
            .header(common::auth_header(&access_token))
            .dispatch();
    }

    let request = client
        .get("/v1/types/names/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::TooManyRequests);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string().unwrap();
    let body_a = (&body[..120]).to_string();
    let body_b = (&body[(body.len() - 13)..]).to_string();

    assert_eq!(
        body_a,
        r#"{"data":{"id":"error_too_many_requests","type":"errors","attributes":{"message":"Too many requests. Please try again in "#
    );
    assert_eq!(body_b, r#" seconds."}}}"#);

    common::teardown(&client);
}
