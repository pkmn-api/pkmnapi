use pkmnapi_api::*;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header, HeaderMap, Status};
use rocket::local::Client;
use rocket::response::Response;
use serde_json::json;
use std::env;
use std::fs;
use std::panic;

#[macro_export]
#[allow(dead_code)]
macro_rules! test {
    ($test_name: ident, () { $($tt:tt)* }) => {
        #[test]
        fn $test_name() {
            common::test_raw(|| {
                $($tt)*
            });
        }
    };
    ($test_name: ident, ($client: ident) { $($tt:tt)* }) => {
        #[test]
        fn $test_name() {
            common::test_client(|$client| {
                $($tt)*
            });
        }
    };
    ($test_name: ident, ($client: ident, $access_token: ident) { $($tt:tt)* }) => {
        #[test]
        fn $test_name() {
            common::test(|$client, $access_token| {
                $($tt)*
            });
        }
    }
}

#[allow(dead_code)]
pub fn test_raw<T>(test: T) -> ()
where
    T: FnOnce() -> Result<(), String> + panic::UnwindSafe,
{
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        test().unwrap();
    }));

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn test_client<T>(test: T) -> ()
where
    T: FnOnce(&Client) -> Result<(), String> + panic::UnwindSafe,
{
    let client = setup();

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        test(&client).unwrap();
    }));

    teardown(&client);

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn test<T>(test: T) -> ()
where
    T: FnOnce(&Client, &String) -> Result<(), String> + panic::UnwindSafe,
{
    let (client, access_token) = setup_with_access_token();

    post_rom(&client, &access_token);
    post_sav(&client, &access_token);

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        test(&client, &access_token).unwrap();
    }));

    teardown(&client);

    assert!(result.is_ok())
}

pub fn setup() -> Client {
    let api = Pkmnapi::init();
    let client = Client::new(api).unwrap();

    client
}

#[allow(dead_code)]
pub fn setup_with_access_token() -> (Client, String) {
    let client = setup();

    let request_body = json!({
        "data": {
            "type": "access_tokens",
            "attributes": {
                "email_address": "foo@bar.com"
            }
        }
    });

    let request = client
        .post("/v1/access_tokens")
        .body(request_body.to_string())
        .header(ContentType::JSON);

    let mut response = request.dispatch();
    let body = response.body_string().unwrap();

    let access_token = (&body[1..65]).to_string();

    let api = Pkmnapi::init();
    let client = Client::new(api).unwrap();

    (client, access_token)
}

pub fn auth_header(access_token: &String) -> Header<'static> {
    Header::new(
        "Authorization",
        format!("Bearer {}", access_token.to_owned()),
    )
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn load_rom() -> Vec<u8> {
    let PKMN_ROM = env::var("PKMN_ROM")
        .expect("Set the PKMN_ROM environment variable to point to the ROM location");

    fs::read(PKMN_ROM).unwrap()
}

#[allow(dead_code)]
pub fn post_rom(client: &Client, access_token: &String) {
    let rom = load_rom();

    client
        .post("/v1/roms")
        .body(&rom)
        .header(auth_header(&access_token))
        .dispatch();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn load_sav() -> Vec<u8> {
    let PKMN_SAV = env::var("PKMN_SAV")
        .expect("Set the PKMN_SAV environment variable to point to the SAV location");

    fs::read(PKMN_SAV).unwrap()
}

#[allow(dead_code)]
pub fn post_sav(client: &Client, access_token: &String) {
    let sav = load_sav();

    client
        .post("/v1/savs")
        .body(&sav)
        .header(auth_header(&access_token))
        .dispatch();
}

#[allow(dead_code)]
pub fn assert_unauthorized(response: &mut Response) -> Result<(), String> {
    let response_body = response.body_string().unwrap();
    let headers = response.headers();

    let body = json!({
        "data": {
            "id": "error_access_tokens_unauthorized",
            "type": "errors",
            "attributes": {
                "message": "Authorization header must be set"
            }
        }
    });

    assert_eq!(response_body, body.to_string());
    assert_eq!(response.status(), Status::Unauthorized);

    assert_headers(
        headers,
        vec![
            ("Content-Type", "application/json"),
            ("Server", "pkmnapi/0.1.0"),
        ],
    )
}

#[allow(dead_code)]
pub fn assert_headers(headers: &HeaderMap<'_>, expected: Vec<(&str, &str)>) -> Result<(), String> {
    let mut headers = headers.clone();

    for (key, val) in &expected {
        let header = match headers.get_one(key) {
            Some(header) => header,
            None => return Err(format!("Could not find header: {}", key)),
        };

        if val.len() != 0 {
            assert_eq!(header, *val);
        }

        headers.remove(key);
    }

    for header in headers.iter() {
        panic!("Extra header found: {}", header);
    }

    Ok(())
}

pub fn teardown(client: &Client) {
    let rocket = client.rocket();
    let sql = rocket.state::<PkmnapiSQL>().unwrap();

    sql.revert_migration();
}
