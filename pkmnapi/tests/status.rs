use rocket::http::{ContentType, Status};

mod common;

#[test]
fn status_ok() {
    let client = common::setup();

    let request = client.get("/status");

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("OK".to_string()));

    common::teardown();
}
