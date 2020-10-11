use rocket::http::Status;

mod common;

test!(status_ok, (client) {
    let request = client.get("/status");

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    assert_eq!(response_body, "OK");
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "text/plain; charset=utf-8"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});
