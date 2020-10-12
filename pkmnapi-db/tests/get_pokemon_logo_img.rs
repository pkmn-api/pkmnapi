mod common;

#[test]
#[ignore]
fn get_pokemon_logo_img() {
    let db = common::load_rom();
    let img = db.get_pokemon_logo_img().unwrap();
    let img_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x38, 0x08, 0x00, 0x00, 0x00, 0x00, 0x10,
        0xCC, 0xAF, 0x0F, 0x00, 0x00, 0x05, 0x3E, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0xED, 0x98,
        0x0B, 0x72, 0xDB, 0x38, 0x10, 0x44, 0xC5, 0x43, 0x4A, 0x87, 0x94, 0x0E, 0xE9, 0x7D, 0xAF,
        0x67, 0x00, 0x82, 0xB4, 0x36, 0x29, 0xCB, 0xF2, 0x6E, 0x55, 0x2A, 0x1D, 0x19, 0x98, 0x4F,
        0xCF, 0x4C, 0x03, 0xA4, 0x95, 0x54, 0xB6, 0x8F, 0xCB, 0xFF, 0x8B, 0x3F, 0x45, 0xC0, 0x76,
        0xBD, 0xB3, 0xBE, 0x82, 0xF7, 0x08, 0xB8, 0x3D, 0x2E, 0xF7, 0x2B, 0xFB, 0x0B, 0x78, 0x8B,
        0x00, 0xE6, 0x5F, 0x2E, 0x2F, 0x36, 0x7A, 0x87, 0x80, 0xC7, 0xED, 0x72, 0x7F, 0x3C, 0x5E,
        0x7C, 0x08, 0x6F, 0x10, 0xE0, 0x7C, 0x6E, 0xE1, 0xC5, 0x87, 0xF0, 0x7D, 0x01, 0x35, 0xFF,
        0x65, 0x05, 0xDF, 0x17, 0xB0, 0xD5, 0xFC, 0xC7, 0x8B, 0xEF, 0xE1, 0x97, 0x05, 0xF0, 0xBE,
        0x5D, 0x1F, 0x8F, 0xCC, 0x0C, 0xB6, 0xCB, 0x95, 0xB9, 0x8C, 0xBF, 0xBA, 0x7F, 0x1D, 0x5F,
        0x13, 0xF0, 0x60, 0x10, 0x07, 0xBD, 0xED, 0x6F, 0x1C, 0xE6, 0x95, 0x30, 0xAA, 0x46, 0x24,
        0xE0, 0xB9, 0x04, 0xBF, 0xBF, 0x94, 0xA3, 0x80, 0x07, 0x7F, 0xFC, 0x50, 0x46, 0xDF, 0x13,
        0x32, 0x1D, 0xDC, 0x79, 0xDC, 0x63, 0x1E, 0x83, 0x98, 0x8F, 0xCB, 0x87, 0x1F, 0x38, 0x2C,
        0xA0, 0xB7, 0x41, 0xFB, 0x05, 0x56, 0x01, 0xB7, 0x59, 0x17, 0x1C, 0x8A, 0x7B, 0xBA, 0xD3,
        0xFC, 0x95, 0xE3, 0xC6, 0x93, 0x9D, 0x61, 0x7E, 0x5C, 0xF2, 0x8D, 0x50, 0x30, 0x7F, 0x2B,
        0x96, 0x50, 0xB4, 0x8C, 0x33, 0xA6, 0x80, 0xD9, 0x2A, 0xAC, 0x5C, 0xC5, 0xFA, 0x54, 0x37,
        0x7E, 0xE2, 0x66, 0x7A, 0x2E, 0xC1, 0xCA, 0x45, 0x73, 0x6E, 0xBB, 0xDE, 0x08, 0x48, 0xF8,
        0xEC, 0xB7, 0x90, 0x82, 0x48, 0x23, 0x69, 0x7A, 0x45, 0x0B, 0xA8, 0xF1, 0xE4, 0x77, 0x18,
        0x49, 0xD3, 0xA0, 0xDF, 0xF5, 0x0E, 0xB3, 0xA3, 0xE0, 0x7E, 0xE5, 0x09, 0x50, 0xC5, 0x4F,
        0x74, 0xC0, 0x1E, 0x34, 0xE2, 0x1A, 0x67, 0x01, 0x02, 0x0D, 0xA9, 0x68, 0x44, 0x40, 0xC6,
        0x1F, 0xC2, 0x01, 0x51, 0x7A, 0x16, 0x46, 0x67, 0x83, 0xC5, 0xD4, 0x58, 0xCB, 0x32, 0x93,
        0x2F, 0x03, 0x4C, 0x93, 0xD9, 0x0F, 0x02, 0x8C, 0xE4, 0x66, 0x2F, 0x1F, 0x5C, 0x23, 0x42,
        0x02, 0x05, 0x28, 0x6E, 0xF6, 0x59, 0xC1, 0x0C, 0xD2, 0x20, 0x0A, 0xAB, 0x37, 0xB5, 0x4D,
        0x35, 0x36, 0x6C, 0x41, 0xE6, 0x32, 0x92, 0x53, 0x40, 0x9F, 0x00, 0x75, 0x15, 0x01, 0x04,
        0x2D, 0xB5, 0x96, 0x24, 0x02, 0xB8, 0x47, 0xEC, 0xA7, 0x20, 0x93, 0x32, 0xEB, 0xA7, 0x80,
        0x6C, 0x80, 0x2E, 0xA9, 0x23, 0x63, 0xA7, 0xB0, 0x2A, 0x02, 0x6E, 0x61, 0xA1, 0x29, 0xBB,
        0xA9, 0x66, 0x63, 0x3C, 0x10, 0xE0, 0xE5, 0x81, 0xEB, 0x15, 0x01, 0xDC, 0x40, 0xA7, 0x3E,
        0x01, 0x62, 0x1A, 0xA4, 0xBE, 0x05, 0xD4, 0x30, 0x80, 0x3A, 0x4D, 0x74, 0x54, 0x2A, 0x46,
        0x42, 0x80, 0xDB, 0x10, 0xE3, 0xD7, 0x20, 0x0D, 0x3A, 0x27, 0x2F, 0xA6, 0xCF, 0xE3, 0x8E,
        0x00, 0xEF, 0x37, 0x81, 0x01, 0x08, 0x50, 0xF8, 0xB1, 0x30, 0x77, 0x48, 0x3D, 0xE1, 0x03,
        0xC9, 0x98, 0xCD, 0x29, 0x9E, 0x06, 0xC7, 0xD2, 0x5A, 0xE0, 0x4B, 0xE0, 0x1C, 0x31, 0xEB,
        0xA9, 0x6C, 0x13, 0x7D, 0xDB, 0x07, 0x2E, 0x8C, 0xB5, 0x90, 0x56, 0x20, 0x91, 0x7F, 0x17,
        0x40, 0x28, 0x3E, 0x7B, 0x0C, 0x8A, 0x78, 0x05, 0xAA, 0x6A, 0x07, 0xF5, 0x24, 0x1A, 0xD2,
        0x04, 0x25, 0x45, 0xF3, 0x82, 0xB7, 0xBB, 0x79, 0x8A, 0x3B, 0x19, 0xC0, 0x00, 0x92, 0x68,
        0xC0, 0xB1, 0x20, 0x62, 0x1F, 0x04, 0x48, 0x89, 0x7F, 0xCB, 0x0A, 0x45, 0x1F, 0x3A, 0xF6,
        0x02, 0x69, 0xC4, 0x59, 0x20, 0x64, 0x07, 0x04, 0x8B, 0x06, 0x9D, 0x77, 0x00, 0x97, 0x74,
        0x87, 0x0A, 0x90, 0x81, 0x7C, 0x28, 0xE5, 0x08, 0x03, 0x0D, 0x24, 0x75, 0xC9, 0x78, 0xDB,
        0x58, 0x9E, 0x3C, 0x02, 0xE2, 0xB3, 0x8A, 0x9A, 0x32, 0x09, 0x16, 0xCD, 0x07, 0xB4, 0xF1,
        0x3B, 0x89, 0xC9, 0xF1, 0xF8, 0x54, 0x3E, 0x20, 0xAA, 0xAB, 0x3A, 0xF7, 0x04, 0xBA, 0x0C,
        0xF0, 0xFE, 0x0D, 0xCD, 0x14, 0xB2, 0xC8, 0x66, 0x61, 0xC5, 0x5B, 0xC0, 0x01, 0xC8, 0xC2,
        0x16, 0x71, 0xC0, 0x50, 0xC2, 0x7E, 0x47, 0x00, 0x71, 0x40, 0xF1, 0x32, 0x00, 0x90, 0x65,
        0x08, 0xE1, 0x19, 0x4C, 0x24, 0x20, 0x7C, 0xB8, 0x6E, 0x7C, 0x6D, 0xB6, 0xC1, 0x68, 0x10,
        0xB9, 0xB3, 0x98, 0x04, 0xB3, 0x41, 0xDD, 0x1A, 0x83, 0x79, 0xC1, 0x10, 0x00, 0x0B, 0xB8,
        0x74, 0x3E, 0x48, 0x19, 0xDA, 0x58, 0xE9, 0x4B, 0x40, 0x0F, 0x27, 0xA8, 0x92, 0x95, 0x1E,
        0x18, 0x1E, 0x94, 0x20, 0x15, 0x59, 0x72, 0x53, 0xFD, 0xBE, 0xEC, 0x02, 0xF8, 0x0E, 0x40,
        0x40, 0xEA, 0x84, 0x72, 0x27, 0x24, 0x91, 0xC8, 0xCA, 0x22, 0xE6, 0x09, 0x04, 0x39, 0x93,
        0x47, 0xF0, 0x68, 0x26, 0x83, 0x89, 0x55, 0x50, 0x41, 0xFB, 0x95, 0x94, 0xA4, 0x00, 0x59,
        0x5C, 0x05, 0x90, 0x08, 0x38, 0x69, 0x32, 0x42, 0x0E, 0x33, 0x60, 0x90, 0x75, 0x15, 0xBB,
        0x25, 0x3A, 0xBD, 0x83, 0x40, 0x1A, 0xB0, 0xB1, 0x32, 0xB1, 0xF8, 0x84, 0x05, 0xE6, 0x3C,
        0x4A, 0x40, 0x16, 0x5E, 0x04, 0x34, 0x85, 0xCA, 0x95, 0x40, 0x34, 0xFD, 0x90, 0x47, 0x98,
        0x7E, 0x44, 0x58, 0x76, 0xD4, 0xD1, 0x26, 0xE0, 0x4B, 0xD4, 0x70, 0xA3, 0x80, 0x11, 0x18,
        0x6C, 0x40, 0xAB, 0xFD, 0x46, 0x9E, 0x40, 0x09, 0xC0, 0x0E, 0x0E, 0x02, 0x96, 0x7E, 0x2C,
        0xB1, 0xD3, 0x19, 0x59, 0x78, 0x81, 0x65, 0xAD, 0xA1, 0x48, 0xE5, 0x70, 0xB7, 0x75, 0xFD,
        0x63, 0x20, 0xB9, 0x24, 0xC6, 0x3B, 0x10, 0x40, 0x30, 0x5B, 0x02, 0xE8, 0x9C, 0x0E, 0x8B,
        0x82, 0xF8, 0xE5, 0x39, 0x27, 0x26, 0xB4, 0x78, 0xB3, 0x4F, 0x48, 0x03, 0x7C, 0x0B, 0x54,
        0x82, 0xA8, 0xA7, 0xDF, 0x05, 0x34, 0x52, 0xEE, 0xB3, 0x0D, 0x48, 0x6A, 0x96, 0x00, 0x41,
        0xE3, 0x45, 0x00, 0x4D, 0x38, 0x45, 0x7B, 0x8C, 0x91, 0x0B, 0x85, 0x80, 0x3C, 0x9A, 0xB3,
        0x08, 0x78, 0x05, 0x02, 0xA3, 0x1A, 0x86, 0xC3, 0x2F, 0x77, 0x45, 0xE0, 0x0F, 0xA4, 0x7C,
        0x11, 0x90, 0xD1, 0x47, 0x01, 0xA4, 0x3B, 0x4F, 0x2D, 0x1E, 0x05, 0x0B, 0xB8, 0x34, 0x82,
        0x64, 0x1C, 0xAA, 0x59, 0x20, 0xC0, 0x0A, 0xA6, 0x80, 0x10, 0x10, 0x50, 0x05, 0x03, 0x99,
        0xEF, 0xAB, 0x11, 0x90, 0x8B, 0xB5, 0x0B, 0x80, 0x40, 0x38, 0x24, 0x40, 0x9E, 0x65, 0xAD,
        0x97, 0x60, 0x32, 0xF3, 0x9C, 0xA0, 0x23, 0x42, 0x15, 0x04, 0xB1, 0xF8, 0x46, 0xE4, 0xEE,
        0x70, 0x21, 0xC0, 0x25, 0x52, 0x20, 0x8C, 0x9D, 0xC7, 0x20, 0x68, 0x86, 0xFB, 0x44, 0x80,
        0x2C, 0x41, 0x2D, 0x5E, 0xB3, 0x03, 0x6E, 0x5B, 0xD7, 0x84, 0x60, 0x46, 0x71, 0xE9, 0xCC,
        0x2A, 0x08, 0x05, 0xF8, 0x9A, 0x7E, 0x09, 0x62, 0xE3, 0xE1, 0x63, 0x69, 0x2C, 0x02, 0x6A,
        0xF2, 0x22, 0x00, 0x0E, 0x0B, 0x65, 0x41, 0xE6, 0x44, 0x74, 0xC3, 0xB4, 0xB9, 0x24, 0x02,
        0x02, 0xE6, 0x23, 0x20, 0x0F, 0xBD, 0xA1, 0xDF, 0x7F, 0x59, 0x59, 0x34, 0x80, 0x6B, 0x75,
        0x36, 0x12, 0xF5, 0x04, 0x56, 0x01, 0x24, 0x49, 0xD0, 0x8A, 0x4D, 0x87, 0x7D, 0x7A, 0x98,
        0xB6, 0xB2, 0x26, 0x89, 0x06, 0x31, 0xFF, 0x12, 0xCA, 0x65, 0x5A, 0xBD, 0x22, 0x02, 0x66,
        0xB8, 0xF4, 0xE8, 0x66, 0xA3, 0x5D, 0x8A, 0x8E, 0x02, 0x32, 0xA3, 0x89, 0x38, 0x12, 0x88,
        0x44, 0x82, 0xA9, 0xEE, 0x41, 0x0B, 0x9C, 0x46, 0x59, 0xF6, 0x92, 0x51, 0xE0, 0xE5, 0x17,
        0xC5, 0x96, 0x4F, 0x76, 0xBA, 0x54, 0xB3, 0xBA, 0xF7, 0xE0, 0x55, 0x00, 0xD1, 0x49, 0xD4,
        0xB1, 0x70, 0xB6, 0x8D, 0xE3, 0x6B, 0xC1, 0x04, 0x0E, 0xCD, 0xD6, 0xC0, 0x54, 0x00, 0xFC,
        0x86, 0xEF, 0x1E, 0x20, 0xDE, 0x30, 0x8B, 0xEB, 0x49, 0x28, 0x4C, 0x7B, 0x8E, 0x97, 0xFD,
        0x24, 0x80, 0x34, 0x4B, 0x51, 0xE8, 0x42, 0x01, 0x30, 0xB6, 0x9A, 0x4C, 0xC0, 0x0B, 0xB5,
        0x31, 0x9A, 0x11, 0xE2, 0x63, 0xF9, 0xCD, 0x5B, 0xD0, 0x0E, 0xA0, 0x2F, 0x1E, 0x69, 0xBC,
        0xF1, 0x04, 0x8E, 0x02, 0x18, 0x4A, 0xAE, 0x28, 0x70, 0x9A, 0x72, 0x82, 0x07, 0x11, 0x53,
        0x02, 0x3E, 0x9F, 0x13, 0xEC, 0xF4, 0x19, 0x96, 0xA4, 0xFB, 0x7C, 0x02, 0x4F, 0x04, 0x70,
        0xA0, 0xF4, 0xFB, 0x9D, 0x00, 0x19, 0x2C, 0x6B, 0x60, 0x41, 0x3A, 0x05, 0x64, 0xAF, 0xBB,
        0x5B, 0xC7, 0xA3, 0xB7, 0x9B, 0x38, 0x0A, 0x08, 0xB1, 0x38, 0xDA, 0xCB, 0xDB, 0xB6, 0xFF,
        0x9A, 0xAD, 0xF3, 0x22, 0x61, 0x0D, 0x4C, 0x0C, 0x76, 0xA7, 0xA7, 0x5B, 0xCD, 0x79, 0x02,
        0x46, 0x98, 0xBD, 0x3D, 0x11, 0x90, 0x2B, 0x48, 0x6B, 0xFE, 0x2B, 0x82, 0x15, 0x10, 0xA8,
        0x14, 0xD0, 0xDE, 0x21, 0xCF, 0x96, 0x67, 0xF0, 0x4F, 0x69, 0x56, 0x11, 0xFE, 0x28, 0x4E,
        0x94, 0xEE, 0xCC, 0xDE, 0x3E, 0xB6, 0x0B, 0x9F, 0x93, 0x00, 0xFB, 0x91, 0x0E, 0xF8, 0x0E,
        0xA3, 0x56, 0x87, 0x2D, 0x3D, 0x88, 0x60, 0x9C, 0xC0, 0xEB, 0xCD, 0xFA, 0x1C, 0xCA, 0xB0,
        0x09, 0x9B, 0x7D, 0x0A, 0x5A, 0x3C, 0x01, 0x04, 0xF8, 0xE7, 0x24, 0xC0, 0x31, 0xC5, 0xA8,
        0xC2, 0xF7, 0x22, 0xC7, 0x13, 0xFE, 0x0E, 0x30, 0x9C, 0xD9, 0x9F, 0x04, 0x84, 0x83, 0x6C,
        0xD6, 0x67, 0x37, 0xFB, 0x3D, 0xA4, 0xB9, 0x58, 0x0E, 0x77, 0x16, 0x90, 0x2B, 0x28, 0x2C,
        0xAC, 0x37, 0x61, 0x0A, 0x58, 0xCE, 0xF6, 0x49, 0xC0, 0x24, 0xFD, 0x80, 0x80, 0xD9, 0xFC,
        0x57, 0x02, 0xFA, 0x0A, 0x7E, 0x60, 0x7A, 0xC1, 0xD7, 0x71, 0x6D, 0xFE, 0x59, 0x00, 0x2A,
        0x57, 0xC2, 0x4F, 0xE3, 0xB3, 0x80, 0xFF, 0x18, 0x7F, 0x05, 0xFC, 0x15, 0xF0, 0x0F, 0xFB,
        0x8F, 0xDD, 0x79, 0x1D, 0xE0, 0x08, 0xBA, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82,
    ];

    assert_eq!(img.to_png().unwrap(), img_data);
}