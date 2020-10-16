use pkmnapi_api::responses::type_names::TypeNameResponseAll;
use rocket::http::{ContentType, Status};
use serde_json::json;

mod common;

test!(get_type_name_all_200, (client, access_token) {
    let request = client
        .get("/v1/types/names")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = common::load_json::<TypeNameResponseAll>("../secrets/data/json/get_type_name_all_200.json");

    assert_eq!(response_body, body);
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_type_name_200, (client, access_token) {
    let request = client
        .get("/v1/types/names/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "type_names",
            "attributes": {
                "name": "NORMAL"
            },
            "links": {
                "self": "http://localhost:8080/v1/types/names/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/types/names/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(get_type_name_401, (client) {
    let request = client.get("/v1/types/names/0");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(get_type_name_404, (client, access_token) {
    let request = client
        .get("/v1/types/names/100")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid type ID 100: valid range is 0-26"
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

test!(post_type_name_202, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    let request = client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({});

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Accepted);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ]).unwrap();

    let request = client
        .get("/v1/types/names/0")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "0",
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            },
            "links": {
                "self": "http://localhost:8080/v1/types/names/0"
            }
        },
        "links": {
            "self": "http://localhost:8080/v1/types/names/0"
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Ok);

    common::assert_headers(headers, vec![
        ("Content-Type", "application/json"),
        ("Server", "pkmnapi/0.1.0"),
    ])
});

test!(post_type_name_401, (client) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    let request = client
        .post("/v1/types/names/0")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response)
});

test!(post_type_name_404, (client, access_token) {
    let request_body = json!({
        "data": {
            "type": "type_names",
            "attributes": {
                "name": "BORING"
            }
        }
    });

    let request = client
        .post("/v1/types/names/100")
        .body(request_body.to_string())
        .header(ContentType::JSON)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_not_found",
            "type": "errors",
            "attributes": {
                "message": "Invalid type ID 100: valid range is 0-26"
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
