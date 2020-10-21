use pkmnapi_api::responses::hm_names::HMNameResponseAll;
use rocket::http::Status;
use serde_json::json;

mod common;

test!(get_hm_name_all_200, (client, access_token) {
    let request = client
        .get("/v1/hms/names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<HMNameResponseAll>("../secrets/data/json/get_hm_name_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_hm_name_200, (client, access_token) {
    let request = client
        .get("/v1/hms/names/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "1",
            "type": "hm_names",
            "attributes": {
                "name": "HM01"
            },
            "links": {
                "self": "http://localhost:8080/v1/hms/names/1"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/hms/names/1"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_hm_name_401, (client) {
    let request = client.get("/v1/hms/names/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_hm_name_404, (client, access_token) {
    let request = client
        .get("/v1/hms/names/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid HM ID 200: valid range is 1-5"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::NotFound);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});
