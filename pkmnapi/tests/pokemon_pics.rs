use rocket::http::{Accept, ContentType, Status};

mod common;

#[test]
fn get_pokemon_pic_png_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/pics/1")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::PNG));
    assert_eq!(
        response.body_bytes(),
        Some(vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x28, 0x08, 0x00, 0x00, 0x00,
            0x00, 0xA9, 0x95, 0xE7, 0xB1, 0x00, 0x00, 0x01, 0xB8, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0xCD, 0xD4, 0x6D, 0x56, 0x84, 0x30, 0x0C, 0x85, 0xE1, 0x76, 0x91, 0xB0, 0xC8,
            0x76, 0x91, 0xF8, 0xDC, 0xB4, 0x38, 0xE2, 0x19, 0x3D, 0xFE, 0xF4, 0x0A, 0x21, 0x1F,
            0x2F, 0x69, 0x09, 0x8C, 0xFD, 0x6A, 0x7F, 0xD3, 0x3F, 0x05, 0x1F, 0xB5, 0x47, 0xF0,
            0x54, 0xBF, 0xCE, 0xE1, 0xB2, 0xF5, 0x33, 0xA8, 0xD2, 0x8F, 0x17, 0x29, 0x7C, 0xA3,
            0x73, 0xB6, 0x76, 0xB5, 0xF3, 0x38, 0x5F, 0xD5, 0x37, 0x20, 0xEA, 0xEA, 0xE3, 0x50,
            0x1B, 0xBF, 0x80, 0xA1, 0x64, 0x71, 0x73, 0xCE, 0x5F, 0xC0, 0x84, 0xE7, 0x84, 0xB5,
            0x69, 0xE5, 0x9F, 0x97, 0x16, 0xA5, 0x59, 0x30, 0x1D, 0x8F, 0x63, 0xCE, 0xCF, 0xB2,
            0xD2, 0x4B, 0x5D, 0x8F, 0x70, 0xC1, 0x1C, 0xE3, 0x98, 0x67, 0x36, 0x52, 0xFA, 0x0A,
            0x76, 0x1B, 0x3B, 0xDA, 0xE8, 0xAD, 0x81, 0xB9, 0x3A, 0x9E, 0xC7, 0xDD, 0x73, 0x83,
            0x8A, 0x56, 0x2D, 0x50, 0x45, 0x7D, 0x83, 0xD6, 0x17, 0x47, 0x0B, 0xEC, 0x97, 0xBD,
            0x5F, 0x68, 0xC2, 0x65, 0x7F, 0x0B, 0xF4, 0x44, 0xDF, 0x40, 0x5D, 0x5A, 0xBD, 0x86,
            0xBC, 0x36, 0xE4, 0x11, 0x2E, 0xCE, 0x3D, 0x23, 0x08, 0x2D, 0x2B, 0x4F, 0xE7, 0x50,
            0xC6, 0xB5, 0x80, 0xDC, 0x07, 0xB8, 0x2F, 0xD5, 0xB5, 0xE8, 0x09, 0xC3, 0xE5, 0xEF,
            0x01, 0x2E, 0xBB, 0x38, 0xBD, 0x28, 0x64, 0xAC, 0x67, 0x5B, 0xC5, 0xB2, 0x8C, 0x43,
            0x56, 0x1E, 0xC1, 0xA6, 0x9B, 0x6B, 0xBC, 0xA1, 0x44, 0x65, 0xBB, 0xBB, 0xBA, 0x87,
            0x90, 0x9F, 0x57, 0x66, 0x53, 0x14, 0xDA, 0x39, 0xE6, 0x97, 0xA5, 0x7B, 0x1B, 0x59,
            0x95, 0x34, 0x1C, 0x46, 0xC2, 0x23, 0x4B, 0xE0, 0x92, 0xA1, 0x80, 0xDA, 0xED, 0x86,
            0x9E, 0xD9, 0x1C, 0x0D, 0x9E, 0x34, 0x34, 0x22, 0xB7, 0x22, 0xA8, 0xC0, 0x0D, 0x95,
            0x74, 0xC8, 0xBA, 0xDA, 0x81, 0x38, 0x9B, 0x0B, 0xE8, 0xB7, 0x61, 0x6E, 0xFC, 0xA8,
            0x5E, 0x44, 0x5A, 0x16, 0x28, 0x21, 0x2C, 0x01, 0x9D, 0x06, 0x73, 0x4B, 0x6C, 0x4E,
            0xC8, 0xA8, 0xBE, 0xE2, 0x25, 0xBB, 0xAB, 0x60, 0x8F, 0xCF, 0x1B, 0x67, 0x8C, 0x59,
            0x58, 0x3D, 0x13, 0x96, 0x74, 0x74, 0xD0, 0xFA, 0x22, 0xAA, 0xA3, 0x15, 0x1D, 0x4B,
            0x9F, 0x64, 0xA8, 0x9C, 0xBE, 0x13, 0x96, 0xEC, 0x23, 0x91, 0xD3, 0x5B, 0xCA, 0x65,
            0xA5, 0xE5, 0x63, 0x32, 0x86, 0x95, 0x72, 0x99, 0xFB, 0xAB, 0x70, 0x49, 0x98, 0x2C,
            0x15, 0x68, 0x6B, 0x92, 0x87, 0x1E, 0x25, 0x4F, 0xE6, 0x46, 0xBF, 0x09, 0x49, 0xD3,
            0x5A, 0xC4, 0x06, 0xA3, 0x8C, 0x29, 0xA8, 0x32, 0x53, 0xFF, 0x24, 0x34, 0xBC, 0x5B,
            0x3E, 0x40, 0xF7, 0xE7, 0x51, 0xA3, 0x34, 0x64, 0xF3, 0xB7, 0x08, 0xD5, 0x5B, 0xFB,
            0xDB, 0xAE, 0x86, 0x16, 0xB7, 0xEF, 0x84, 0x6F, 0xC0, 0xF8, 0x28, 0x87, 0x62, 0x0F,
            0x89, 0x8A, 0x2F, 0x51, 0xC5, 0x97, 0xB2, 0x47, 0x25, 0x1B, 0x2C, 0x92, 0xC7, 0xDF,
            0xC0, 0x03, 0x0C, 0xCA, 0x20, 0x52, 0xF6, 0x0A, 0xB6, 0x17, 0x7D, 0x03, 0x23, 0x9F,
            0xC4, 0xAA, 0x72, 0xB6, 0x47, 0x6F, 0x40, 0xAD, 0x76, 0x52, 0xCF, 0xED, 0xB5, 0xF6,
            0x01, 0x34, 0x10, 0x2B, 0x10, 0xBE, 0x2C, 0x46, 0x7D, 0x00, 0x00, 0x00, 0x00, 0x49,
            0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ])
    );

    common::teardown(&client);
}

#[test]
fn get_pokemon_pic_png_401() {
    let client = common::setup();

    let request = client.get("/v1/pokemon/pics/1");

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_pokemon_pic_png_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/pics/200")
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokemon_pics","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_string()
        )
    );

    common::teardown(&client);
}

#[test]
fn get_pokemon_pic_jpeg_200() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/pics/1")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JPEG));
    assert_eq!(
        response.body_bytes(),
        Some(vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x02, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x28, 0x00,
            0x28, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07,
            0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C,
            0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A,
            0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37,
            0x29, 0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C,
            0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC4, 0x00, 0x1F, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02,
            0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0xFF, 0xC4, 0x00, 0xB5, 0x10,
            0x00, 0x02, 0x01, 0x03, 0x03, 0x02, 0x04, 0x03, 0x05, 0x05, 0x04, 0x04, 0x00, 0x00,
            0x01, 0x7D, 0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06,
            0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xA1, 0x08, 0x23, 0x42,
            0xB1, 0xC1, 0x15, 0x52, 0xD1, 0xF0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0A, 0x16,
            0x17, 0x18, 0x19, 0x1A, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x34, 0x35, 0x36, 0x37,
            0x38, 0x39, 0x3A, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55,
            0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x73,
            0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
            0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5,
            0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA,
            0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6,
            0xD7, 0xD8, 0xD9, 0xDA, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA,
            0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFF, 0xDA, 0x00, 0x08,
            0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0xF7, 0xFA, 0x2A, 0xA6, 0xA1, 0xAA, 0x69, 0xFA,
            0x45, 0xB7, 0xDA, 0x75, 0x2B, 0xEB, 0x6B, 0x2B, 0x7D, 0xC1, 0x7C, 0xDB, 0x99, 0x96,
            0x35, 0xC9, 0xE8, 0x32, 0xC4, 0x0C, 0xD6, 0x4D, 0xAF, 0x8B, 0xEC, 0x6E, 0xB5, 0xCB,
            0x7D, 0x3D, 0x41, 0x11, 0x5E, 0xA3, 0x1B, 0x1B, 0x8C, 0xE4, 0x5C, 0xED, 0x04, 0xB1,
            0x00, 0x0E, 0x10, 0x80, 0x4A, 0x31, 0x3F, 0xBC, 0xD9, 0x21, 0x03, 0x68, 0x56, 0x7E,
            0x86, 0xA9, 0x6A, 0x9A, 0xB5, 0x96, 0x8D, 0x69, 0xF6, 0x9B, 0xE9, 0x59, 0x10, 0xB6,
            0xD5, 0x58, 0xE2, 0x69, 0x5D, 0xCE, 0x09, 0xC2, 0xA2, 0x02, 0xCC, 0x40, 0x0C, 0xC7,
            0x00, 0xE0, 0x29, 0x27, 0x80, 0x4D, 0x79, 0xC7, 0x8E, 0xBE, 0x2C, 0xD8, 0x69, 0x32,
            0x58, 0xDA, 0x58, 0xB1, 0x96, 0x1B, 0xA8, 0x56, 0xE6, 0x59, 0x56, 0x6F, 0x25, 0xD9,
            0x0B, 0xB2, 0x08, 0x94, 0x91, 0xB9, 0x1B, 0x7A, 0x30, 0x90, 0x91, 0xB9, 0x00, 0x20,
            0x0D, 0xE7, 0x29, 0x8D, 0x71, 0xAB, 0x68, 0xFA, 0x55, 0x96, 0x9F, 0xAC, 0x5D, 0x68,
            0xF7, 0x97, 0xB2, 0x5D, 0xC5, 0x0B, 0x43, 0x79, 0x76, 0x10, 0x45, 0x23, 0x8F, 0x99,
            0x41, 0x2C, 0xED, 0x72, 0xE8, 0x58, 0x71, 0xE7, 0x33, 0x26, 0x51, 0x0A, 0x64, 0x04,
            0x35, 0x95, 0x69, 0xAE, 0x6A, 0x73, 0x2E, 0xA3, 0xF1, 0x0D, 0x8F, 0xDA, 0xAE, 0x34,
            0xED, 0x4A, 0x35, 0x8E, 0xD6, 0x52, 0x76, 0xF9, 0x0A, 0x92, 0xC7, 0x20, 0xC8, 0xFB,
            0xA0, 0x09, 0x26, 0x61, 0x80, 0x79, 0xC1, 0xC1, 0xFB, 0xB5, 0xEE, 0x3A, 0x0E, 0xB7,
            0x69, 0xE2, 0x2D, 0x16, 0xDB, 0x55, 0xB1, 0xDF, 0xF6, 0x79, 0xC3, 0x61, 0x64, 0x5D,
            0xAC, 0xAC, 0xAC, 0x55, 0x94, 0x8E, 0x99, 0x0C, 0xA4, 0x70, 0x48, 0xE3, 0x82, 0x46,
            0x0D, 0x72, 0xFE, 0x3C, 0xF1, 0x1D, 0xB6, 0x91, 0xA8, 0xE9, 0x29, 0x22, 0x93, 0x25,
            0xB1, 0x97, 0x50, 0x12, 0x20, 0x66, 0x31, 0xB2, 0x46, 0xC0, 0x2B, 0x60, 0x36, 0xC5,
            0x78, 0xCD, 0xC3, 0x16, 0x2A, 0xC7, 0x64, 0x32, 0x84, 0x52, 0xFB, 0x4A, 0xF2, 0x7E,
            0x27, 0xD5, 0x2D, 0x97, 0x4D, 0x8B, 0x58, 0xD1, 0xEC, 0x3F, 0xB2, 0x6C, 0x62, 0x56,
            0xBE, 0xBB, 0xB5, 0x8A, 0xCA, 0x34, 0xB8, 0xB9, 0x66, 0x50, 0x48, 0x93, 0xB2, 0x9F,
            0xEF, 0x1C, 0x16, 0xE0, 0xF2, 0x2B, 0xCD, 0x75, 0xB8, 0xB5, 0x09, 0x6D, 0xED, 0xAC,
            0xA7, 0xE1, 0x1E, 0x35, 0xB8, 0xB4, 0x7B, 0x62, 0xC2, 0x24, 0xB7, 0x90, 0xB3, 0x47,
            0x24, 0x61, 0xC0, 0x25, 0x06, 0x30, 0x01, 0x00, 0x8D, 0xBC, 0x01, 0xB5, 0x6B, 0x66,
            0xCB, 0xC5, 0x97, 0xD7, 0x3E, 0x1A, 0xB2, 0xF0, 0x7D, 0x9E, 0x9D, 0x0A, 0x5C, 0xB3,
            0x90, 0x3E, 0xCE, 0xEC, 0x23, 0x94, 0xE7, 0x04, 0xEE, 0x3C, 0x22, 0x8E, 0x87, 0x24,
            0x60, 0x67, 0x83, 0xC5, 0x7A, 0xCF, 0xC2, 0x7B, 0x14, 0xB0, 0xF0, 0xED, 0xF4, 0x50,
            0xCB, 0xE7, 0xDB, 0x8B, 0xB4, 0x8E, 0x2B, 0x80, 0x3E, 0x59, 0xBC, 0xBB, 0x58, 0x21,
            0x76, 0x5F, 0xF6, 0x7C, 0xC8, 0xE4, 0x03, 0xE9, 0x54, 0xBC, 0x51, 0xA1, 0xDB, 0x49,
            0x75, 0xAC, 0xDC, 0xF8, 0x8F, 0x50, 0xBE, 0xB0, 0x12, 0xE1, 0xEC, 0xF5, 0x2D, 0x3B,
            0x7C, 0x71, 0xC7, 0x0A, 0xAE, 0x36, 0x48, 0xC4, 0xB0, 0x57, 0x05, 0x88, 0xC3, 0x15,
            0x43, 0x90, 0xD1, 0x85, 0x76, 0x96, 0xBC, 0xBB, 0x40, 0xF0, 0xDE, 0xAB, 0xE2, 0x1D,
            0x2E, 0x6B, 0xBD, 0x37, 0xC5, 0xA5, 0x66, 0x80, 0xB2, 0x95, 0x9F, 0xC4, 0x12, 0x1C,
            0x30, 0xCE, 0xD7, 0xF2, 0xFE, 0xCF, 0xF3, 0x23, 0x63, 0x23, 0x07, 0x9E, 0x57, 0x21,
            0x83, 0x01, 0xB5, 0xE3, 0x69, 0xFC, 0x1F, 0x1D, 0xAD, 0x95, 0xB6, 0x8B, 0x60, 0x34,
            0xAD, 0x4A, 0x34, 0x0B, 0xE5, 0xF9, 0x3B, 0x65, 0x94, 0x90, 0x33, 0xE6, 0xF7, 0x94,
            0x2F, 0xFC, 0xF4, 0xDC, 0xD9, 0x60, 0xD8, 0x6C, 0x6E, 0x27, 0x97, 0xB7, 0xF0, 0x85,
            0xCC, 0x7A, 0x46, 0x97, 0xAD, 0x0B, 0x37, 0x6D, 0x28, 0x9F, 0x3E, 0x18, 0x52, 0x57,
            0x89, 0xA4, 0x84, 0x3F, 0xCE, 0xAC, 0xC0, 0x92, 0x15, 0xF1, 0x9C, 0x81, 0x95, 0x04,
            0x1C, 0x57, 0xD2, 0xBE, 0x1B, 0xD4, 0xB4, 0xDD, 0x5F, 0xC3, 0x5A, 0x6D, 0xFE, 0x90,
            0xA8, 0x9A, 0x7C, 0xD6, 0xE8, 0x60, 0x8D, 0x02, 0x81, 0x12, 0xE3, 0x1B, 0x30, 0xA4,
            0x85, 0x2B, 0x8D, 0xA4, 0x0E, 0x84, 0x11, 0xDA, 0xB5, 0x2B, 0x8A, 0xF1, 0x47, 0x85,
            0xA4, 0xB8, 0xFB, 0x6D, 0xC5, 0xAE, 0x99, 0x15, 0xF9, 0xB9, 0x94, 0x5C, 0x1D, 0xB3,
            0x79, 0x37, 0x50, 0x4B, 0xE5, 0xAC, 0x4C, 0x61, 0x7C, 0x81, 0x86, 0x44, 0x40, 0x46,
            0xE4, 0xC6, 0x18, 0xE5, 0xB2, 0x14, 0x78, 0x99, 0xF0, 0x96, 0xBD, 0xAA, 0x78, 0xB4,
            0x78, 0x72, 0xED, 0x35, 0x48, 0x2E, 0x67, 0x04, 0x4B, 0x14, 0xEF, 0x14, 0xAF, 0x1C,
            0x0C, 0x30, 0x26, 0xDE, 0x01, 0x0E, 0x8B, 0x9E, 0x4E, 0xE1, 0xB8, 0xA1, 0x40, 0x43,
            0x30, 0x23, 0xB1, 0xF1, 0x37, 0x89, 0x3C, 0x45, 0xA3, 0x69, 0x76, 0x5E, 0x0F, 0xD7,
            0x74, 0x98, 0x05, 0xD3, 0x44, 0x12, 0xDE, 0xF2, 0xD1, 0x8B, 0x45, 0x75, 0xC6, 0xC0,
            0x91, 0x26, 0x03, 0x17, 0x1B, 0xB0, 0x57, 0x19, 0xC9, 0x5E, 0x30, 0x41, 0x6F, 0x4D,
            0xF0, 0x26, 0x8B, 0x71, 0xE1, 0xEF, 0x05, 0x69, 0xDA, 0x75, 0xE7, 0x17, 0x4A, 0xAF,
            0x34, 0xC9, 0xC7, 0xEE, 0xDE, 0x47, 0x69, 0x19, 0x32, 0x09, 0x07, 0x69, 0x72, 0xB9,
            0x1D, 0x71, 0x9E, 0x33, 0x5F, 0xFF, 0xD9,
        ])
    );

    common::teardown(&client);
}

#[test]
fn get_pokemon_pic_jpeg_401() {
    let client = common::setup();

    let request = client.get("/v1/pokemon/pics/1").header(Accept::JPEG);

    let mut response = request.dispatch();

    common::assert_unauthorized(&mut response);
    common::teardown(&client);
}

#[test]
fn get_pokemon_pic_jpeg_404() {
    let (client, access_token) = common::setup_with_access_token();

    common::post_rom(&client, &access_token);

    let request = client
        .get("/v1/pokemon/pics/200")
        .header(Accept::JPEG)
        .header(common::auth_header(&access_token));

    let mut response = request.dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(
            r#"{"data":{"id":"error_pokemon_pics","type":"errors","attributes":{"message":"Invalid Pokédex ID: 200"}}}"#
                .to_string()
        )
    );

    common::teardown(&client);
}
