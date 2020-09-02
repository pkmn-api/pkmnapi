use rocket::local::Client;
use rocket::http::{ContentType, Status};
use pkmnapi::*;

#[test]
fn ok() {
    let api = Pkmnapi::init();
    let client = Client::new(api).unwrap();

    let response = client.get("/status").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
}
