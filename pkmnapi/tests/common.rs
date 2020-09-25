use pkmnapi::*;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header, Status};
use rocket::local::Client;
use rocket::response::Response;
use std::env;
use std::fs;

pub fn setup() -> Client {
    let api = Pkmnapi::init();
    let client = Client::new(api).unwrap();

    client
}

#[allow(dead_code)]
pub fn setup_with_access_token() -> (Client, String) {
    let client = setup();

    let request = client
        .post("/v1/access_tokens")
        .body(r#"{"data":{"type":"access_tokens","attributes":{"email_address":"foo@bar.com"}}}"#)
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
pub fn assert_unauthorized(response: &mut Response) {
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"data":{"id":"error_access_tokens_unauthorized","type":"errors","attributes":{"message":"Authorization header must be set"}}}"#.to_string()));
}

pub fn teardown(client: &Client) {
    let rocket = client.rocket();
    let sql = rocket.state::<PkmnapiSQL>().unwrap();

    sql.revert_migration();
}
