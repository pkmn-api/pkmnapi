mod common;

#[test]
#[ignore]
fn get_town_map_img() {
    let db = common::load_rom();
    let img = db.get_town_map_img().unwrap();
    let img_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0xA0, 0x00, 0x00, 0x00, 0x90, 0x08, 0x00, 0x00, 0x00, 0x00, 0xAA,
        0xDE, 0x3F, 0x5C, 0x00, 0x00, 0x09, 0x64, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0xED, 0x9C,
        0x0B, 0x72, 0xDB, 0x38, 0x10, 0x44, 0xE3, 0x43, 0xC6, 0x87, 0xB4, 0x0F, 0x99, 0x7D, 0x3D,
        0x40, 0xE3, 0x33, 0x24, 0x48, 0xCA, 0xB2, 0x62, 0x6E, 0x55, 0xBA, 0x4C, 0x34, 0x07, 0xF3,
        0x41, 0x13, 0x20, 0x60, 0x39, 0xDA, 0xDA, 0xB7, 0x3F, 0xBF, 0xEE, 0x8D, 0x7F, 0x02, 0x9F,
        0xC5, 0x3F, 0x81, 0xCF, 0xE2, 0x9F, 0xC0, 0x67, 0x71, 0x7F, 0x81, 0x1F, 0x34, 0xAF, 0xC4,
        0x6F, 0xAE, 0x3D, 0x7C, 0xFE, 0x7E, 0xA7, 0xD5, 0xE8, 0xC7, 0xFC, 0x73, 0x02, 0xF1, 0xBC,
        0x7F, 0xA0, 0x02, 0x01, 0x47, 0xFC, 0x83, 0x02, 0xAB, 0x2B, 0xA4, 0x80, 0x05, 0xFF, 0xA4,
        0xC0, 0x5F, 0xB1, 0xCA, 0xE0, 0xE3, 0x80, 0x7F, 0x54, 0x60, 0x69, 0x98, 0x29, 0x84, 0x80,
        0x5D, 0xFE, 0x59, 0x81, 0xA5, 0x0D, 0x21, 0x60, 0x97, 0x7F, 0x58, 0x20, 0x24, 0x19, 0x5A,
        0xCB, 0x05, 0xFF, 0xB4, 0x40, 0x18, 0x15, 0x56, 0xB3, 0xC3, 0x7F, 0x4D, 0xA0, 0x05, 0x2D,
        0x11, 0x3B, 0x46, 0x6A, 0x66, 0xBE, 0x8F, 0x40, 0x22, 0x75, 0xEE, 0xA1, 0x69, 0xE2, 0x1B,
        0x09, 0xAC, 0xA1, 0x21, 0x0D, 0x54, 0xBE, 0x93, 0xC0, 0xB2, 0xCA, 0x40, 0x6F, 0x9F, 0x20,
        0xBE, 0x95, 0xC0, 0x1A, 0xC4, 0xC2, 0xD2, 0x56, 0xBE, 0x97, 0xC0, 0x12, 0x15, 0xC2, 0x40,
        0xF0, 0xCD, 0x04, 0x12, 0x26, 0x59, 0x5A, 0xDB, 0xCA, 0x77, 0x13, 0x48, 0x1C, 0xAA, 0xAC,
        0x0E, 0xDE, 0x08, 0xA4, 0x37, 0x90, 0xFB, 0x33, 0x3C, 0xF0, 0x19, 0xDE, 0xB8, 0x84, 0xB3,
        0x0F, 0xC6, 0xF9, 0x01, 0x38, 0x74, 0x68, 0xF7, 0x04, 0x96, 0x4A, 0x9B, 0xFE, 0x8C, 0xCB,
        0x02, 0x6B, 0xBD, 0x42, 0x6B, 0x64, 0x81, 0x9F, 0x08, 0xD8, 0x3D, 0x07, 0xDF, 0xFF, 0x84,
        0xF2, 0x4D, 0x7F, 0xC6, 0x20, 0xF0, 0xED, 0x68, 0x7A, 0xDE, 0x6A, 0xBD, 0x83, 0x90, 0x40,
        0x16, 0xC8, 0x91, 0x13, 0x12, 0x36, 0x42, 0x10, 0x88, 0xEE, 0x47, 0x04, 0x32, 0x36, 0x3F,
        0x2B, 0x20, 0x30, 0xEA, 0xAD, 0x23, 0x0A, 0xB2, 0x40, 0xD9, 0x7A, 0x0B, 0x37, 0x42, 0x6E,
        0x24, 0x90, 0x3C, 0xF2, 0xB3, 0x10, 0x04, 0xD2, 0x3E, 0x22, 0xF0, 0x35, 0x4B, 0x8C, 0xBD,
        0x12, 0x48, 0xB3, 0xD3, 0x9F, 0x31, 0x08, 0x3C, 0x44, 0x55, 0x56, 0x69, 0x0D, 0x04, 0x4D,
        0x60, 0x93, 0xF0, 0x16, 0xEE, 0x9C, 0x83, 0x88, 0x0E, 0xE4, 0xFE, 0x8C, 0xCB, 0x02, 0xB9,
        0x84, 0x87, 0x05, 0x4A, 0x1D, 0x2A, 0x36, 0x02, 0x3D, 0xB0, 0x13, 0x88, 0x0A, 0xAC, 0x06,
        0x70, 0xDC, 0x0A, 0xB9, 0x5E, 0xC6, 0xCA, 0xBF, 0x3C, 0x07, 0x73, 0x82, 0x97, 0xBC, 0xD0,
        0x16, 0x8E, 0x5B, 0x21, 0xD7, 0xCB, 0x58, 0xF8, 0x99, 0x40, 0xC6, 0x66, 0x99, 0x2F, 0x08,
        0x8C, 0x27, 0x79, 0xA5, 0xC0, 0xD8, 0x64, 0xB3, 0x9F, 0x5E, 0xDE, 0x40, 0x6E, 0xAE, 0xCC,
        0xA0, 0x9E, 0xE3, 0x95, 0x02, 0x29, 0xCD, 0xCF, 0xE4, 0xAF, 0x86, 0xDE, 0xC2, 0x5B, 0x0A,
        0xE4, 0xB7, 0x88, 0xC0, 0xB8, 0xD7, 0x04, 0xD2, 0xBE, 0x52, 0xE0, 0x66, 0x89, 0xCB, 0xFE,
        0x60, 0x50, 0xF1, 0x05, 0x81, 0x34, 0xC4, 0x15, 0xDA, 0xC2, 0x71, 0x2B, 0xE4, 0x7A, 0x19,
        0x5B, 0x3F, 0xF3, 0xA7, 0xB5, 0x95, 0x3C, 0xF8, 0x5C, 0x20, 0x97, 0xF0, 0xD7, 0x04, 0x7E,
        0xA2, 0xCA, 0xEA, 0xE0, 0x53, 0x81, 0xD9, 0x7E, 0x14, 0x67, 0xF9, 0xD9, 0x5F, 0xD6, 0x57,
        0xAA, 0x0A, 0xDF, 0x4D, 0x60, 0x3B, 0xFF, 0xCC, 0x7B, 0x02, 0xA7, 0x97, 0x36, 0xDB, 0x8F,
        0x82, 0xFC, 0xC0, 0x2A, 0x7F, 0xF6, 0x63, 0x85, 0x34, 0x50, 0x79, 0x47, 0x20, 0xFB, 0x81,
        0x9F, 0x92, 0xB0, 0xB5, 0x1F, 0x05, 0x43, 0x06, 0x56, 0xF9, 0x93, 0x3F, 0x1A, 0x16, 0x56,
        0xAB, 0x0B, 0xC4, 0xB7, 0x12, 0xC8, 0xFE, 0x15, 0x58, 0x58, 0xDA, 0xCA, 0x3B, 0x02, 0xE7,
        0x25, 0xCD, 0xF6, 0xA3, 0x20, 0x3F, 0xB0, 0xCA, 0x1F, 0xFC, 0x65, 0x7F, 0x54, 0x61, 0x20,
        0x78, 0x4F, 0x60, 0x80, 0x84, 0x40, 0xB6, 0x1F, 0xC5, 0x59, 0x7E, 0xF7, 0x33, 0x7F, 0x5A,
        0x53, 0xC9, 0x1A, 0xF8, 0x3E, 0x02, 0xD3, 0xF9, 0x67, 0x7E, 0x58, 0xA0, 0xED, 0x8C, 0xD5,
        0x07, 0xD3, 0x55, 0xBF, 0xD1, 0xFD, 0xE8, 0x41, 0x90, 0xB6, 0xEF, 0xC8, 0xDF, 0x27, 0xB0,
        0x28, 0xA8, 0xD4, 0x51, 0x3B, 0x2A, 0x6D, 0x51, 0x1D, 0x6F, 0xBF, 0x11, 0xA2, 0x73, 0x0F,
        0x4D, 0x13, 0x7F, 0xA3, 0xC0, 0x78, 0xE2, 0x3A, 0x5E, 0xC7, 0xAA, 0xDF, 0x68, 0xFE, 0x90,
        0x04, 0x12, 0x7F, 0xA7, 0x40, 0x9E, 0x97, 0x81, 0xB8, 0x1F, 0xB1, 0xEA, 0x37, 0xEC, 0x57,
        0x5D, 0xBD, 0x75, 0xC2, 0xC8, 0x77, 0x12, 0x08, 0xD3, 0x91, 0xF8, 0x3B, 0x05, 0xD2, 0x6E,
        0x85, 0xAC, 0xFA, 0x0D, 0xFB, 0xA9, 0x1B, 0x82, 0xC0, 0xC4, 0xDF, 0x28, 0x90, 0xA6, 0x51,
        0x47, 0xED, 0xA8, 0xB4, 0x45, 0x75, 0x68, 0x93, 0xF0, 0xD6, 0xA1, 0x28, 0xF1, 0xF7, 0x09,
        0xE4, 0x12, 0xCA, 0x78, 0x1D, 0xAB, 0x7E, 0xC3, 0xFE, 0xDF, 0x52, 0x63, 0x55, 0x03, 0x7F,
        0x59, 0xA0, 0xED, 0x33, 0x9C, 0xC5, 0xDB, 0xAF, 0x6D, 0x2B, 0x41, 0x99, 0x6F, 0x23, 0x90,
        0xAD, 0xA2, 0x8B, 0x89, 0x9B, 0xF9, 0x36, 0x02, 0x8B, 0x24, 0x90, 0xF8, 0x36, 0x02, 0x63,
        0x41, 0xD5, 0x80, 0x91, 0xEF, 0x24, 0x90, 0x05, 0xE5, 0x26, 0xF1, 0x3F, 0x81, 0x67, 0xF1,
        0xF6, 0x6B, 0x43, 0xC4, 0x9A, 0x26, 0xBE, 0x8F, 0x40, 0xA9, 0xB1, 0xAA, 0x81, 0x6F, 0x23,
        0x50, 0x12, 0x25, 0x28, 0xF3, 0xB3, 0x02, 0xF9, 0xA4, 0x4E, 0x7B, 0x00, 0xBB, 0x6B, 0xFC,
        0x06, 0xF6, 0xB3, 0xC4, 0xBA, 0x98, 0xB8, 0x99, 0x9F, 0x14, 0x88, 0xF5, 0xCE, 0x75, 0x00,
        0x7B, 0x4B, 0xFC, 0x16, 0xF6, 0x17, 0x49, 0x20, 0xF1, 0x73, 0x02, 0xA3, 0x69, 0x5D, 0xBB,
        0xB0, 0xB3, 0x84, 0x6E, 0x61, 0x7F, 0x2C, 0xA8, 0x1A, 0x30, 0xF2, 0x53, 0x02, 0xF9, 0x3B,
        0x4C, 0xF8, 0xD0, 0xFD, 0x0A, 0x63, 0xFC, 0x1E, 0xEC, 0xA7, 0x14, 0x0B, 0xCA, 0x4D, 0xE2,
        0x67, 0x04, 0xF6, 0xBF, 0x63, 0x31, 0x56, 0x18, 0xE2, 0x77, 0x61, 0x3F, 0xA5, 0x42, 0x10,
        0x98, 0xF8, 0x09, 0x81, 0xCC, 0x9F, 0xD6, 0x40, 0x65, 0x0E, 0xE6, 0xB0, 0xC7, 0xEF, 0xC3,
        0x7E, 0x6D, 0x88, 0x5E, 0xAF, 0xF3, 0xD7, 0x05, 0xCE, 0x7F, 0xC7, 0xDA, 0xBD, 0x41, 0x8B,
        0xE7, 0xDA, 0x83, 0xFD, 0xB5, 0xCE, 0x86, 0x97, 0x02, 0x0D, 0x7F, 0xA0, 0x74, 0x5C, 0xF7,
        0x93, 0x1F, 0xBD, 0x33, 0x67, 0x21, 0x8E, 0xCF, 0xFD, 0x86, 0xFD, 0xCE, 0xCF, 0x7C, 0x2E,
        0xB0, 0x7C, 0x14, 0x6E, 0x71, 0xF6, 0xB3, 0x24, 0xBA, 0x78, 0xD0, 0xC4, 0x0E, 0xA8, 0xB0,
        0x79, 0x26, 0x90, 0x5C, 0x5D, 0xBD, 0x4E, 0xE5, 0x36, 0xB0, 0xE1, 0x04, 0xC3, 0x7F, 0xD4,
        0x38, 0xAE, 0xF9, 0xA3, 0x04, 0xD8, 0x70, 0x8B, 0x08, 0xD8, 0x3A, 0x13, 0xD8, 0xF3, 0x67,
        0x6E, 0x03, 0x1B, 0x2D, 0xA1, 0x02, 0x81, 0x3C, 0xC7, 0x56, 0x60, 0xC8, 0x56, 0x03, 0x12,
        0x3B, 0x24, 0x60, 0xE3, 0x4C, 0xA0, 0x52, 0x6B, 0xFE, 0xC4, 0x6D, 0x60, 0xC3, 0x09, 0xC6,
        0x81, 0x40, 0xFA, 0xB9, 0xD9, 0xE1, 0x51, 0x8C, 0xE3, 0xC7, 0xBE, 0x11, 0xF6, 0x93, 0xDA,
        0xF2, 0x47, 0x6E, 0x03, 0x1B, 0x4E, 0x30, 0x10, 0x48, 0xFB, 0x98, 0xC0, 0x51, 0x8D, 0xE3,
        0x87, 0xAE, 0x09, 0xF6, 0x93, 0xDA, 0xF3, 0x07, 0x6E, 0x03, 0x1B, 0x4E, 0x30, 0x0E, 0x36,
        0x09, 0x6F, 0x09, 0x15, 0x76, 0xB9, 0xCB, 0x71, 0x7C, 0xEF, 0x99, 0x61, 0xFF, 0xAA, 0x5E,
        0x1B, 0xD8, 0x70, 0x82, 0xB1, 0x3A, 0x66, 0x22, 0x9B, 0xDE, 0x05, 0x37, 0x3D, 0x8E, 0x6F,
        0x1D, 0x09, 0xF6, 0xD7, 0xBC, 0x0D, 0x6F, 0x04, 0x66, 0xB8, 0x80, 0x07, 0xD8, 0xDA, 0xD4,
        0xA1, 0x90, 0x86, 0x18, 0xD9, 0x7E, 0x59, 0x42, 0x59, 0x88, 0x23, 0x28, 0xF2, 0xE3, 0x33,
        0xD7, 0xCB, 0x02, 0xF9, 0x0D, 0x46, 0x3B, 0xC0, 0xA6, 0x07, 0xCC, 0x36, 0x5F, 0xDD, 0xD7,
        0xE5, 0x99, 0xD9, 0x71, 0x67, 0x5F, 0xA5, 0x19, 0xE4, 0x70, 0x7D, 0x6E, 0xEA, 0x25, 0x81,
        0x94, 0x6D, 0xA5, 0x0B, 0x6C, 0x59, 0xD0, 0xC6, 0x8E, 0x52, 0x20, 0xB1, 0xFD, 0x67, 0x5F,
        0x46, 0x36, 0x28, 0x8F, 0xA4, 0x5C, 0x6F, 0x16, 0x48, 0x00, 0xB0, 0x86, 0x80, 0x8D, 0xE2,
        0xDA, 0xDA, 0x62, 0xBD, 0x2D, 0xC2, 0xC8, 0x8E, 0x43, 0x20, 0xF3, 0x70, 0x2E, 0x90, 0x94,
        0xC8, 0xC9, 0xF5, 0x26, 0x81, 0xA8, 0x17, 0x98, 0xE8, 0x0E, 0x25, 0x09, 0xEE, 0xCB, 0x36,
        0x8C, 0x00, 0x6E, 0x12, 0x3B, 0xEE, 0x01, 0x81, 0x91, 0x92, 0xEB, 0x8D, 0x02, 0xCB, 0xFB,
        0x29, 0x07, 0x51, 0x46, 0x64, 0x01, 0x77, 0x65, 0x1B, 0x8E, 0x42, 0x60, 0x62, 0xC7, 0x21,
        0x90, 0xF6, 0x8A, 0xC0, 0x92, 0x91, 0xEB, 0x0D, 0x02, 0x99, 0x3F, 0xCD, 0xA9, 0xBA, 0x87,
        0x39, 0x2C, 0x69, 0x91, 0x18, 0xC8, 0x36, 0x2F, 0x35, 0xF1, 0xCE, 0xEB, 0xEC, 0xB8, 0xCB,
        0x9B, 0xA4, 0x26, 0xE4, 0x7A, 0x5D, 0xE0, 0xE2, 0xF3, 0x5D, 0xCD, 0x6B, 0x82, 0xB2, 0xCD,
        0xD7, 0x93, 0x8A, 0x77, 0x5E, 0x63, 0xFB, 0x31, 0x02, 0x67, 0x02, 0x1D, 0x9F, 0xEB, 0x35,
        0x81, 0x65, 0x7D, 0x65, 0x15, 0x76, 0x42, 0x16, 0xB4, 0xB5, 0x4B, 0xBC, 0xF3, 0xCC, 0xF6,
        0x9F, 0xE1, 0xAC, 0x9E, 0x05, 0x22, 0x9C, 0xAE, 0x10, 0x5E, 0xD8, 0x89, 0xE6, 0x5E, 0xA0,
        0xA0, 0xDA, 0x9B, 0xBC, 0xC6, 0x0E, 0x3C, 0x81, 0xC3, 0x56, 0xF5, 0xAA, 0x40, 0xC2, 0xA2,
        0x0B, 0x54, 0xAE, 0x09, 0xF2, 0x04, 0xF6, 0x6D, 0xAC, 0x1A, 0xBF, 0x65, 0x7C, 0x17, 0xE0,
        0xA8, 0x55, 0xBD, 0x22, 0xB0, 0x78, 0x63, 0xD5, 0x05, 0x31, 0xA1, 0x01, 0x73, 0x0D, 0x99,
        0xED, 0x68, 0x4A, 0xBC, 0x90, 0xD8, 0xA1, 0x87, 0x70, 0x50, 0x94, 0x8A, 0xA6, 0xE5, 0x07,
        0x87, 0x40, 0xF6, 0xAF, 0xC0, 0x84, 0xD2, 0x56, 0x76, 0xA2, 0xB9, 0xE6, 0x4E, 0xF6, 0x5E,
        0x1E, 0x68, 0xAC, 0x98, 0x33, 0x9C, 0xD5, 0x93, 0xC0, 0xF2, 0x5E, 0xD6, 0x0E, 0x10, 0xEC,
        0x44, 0xB3, 0x0A, 0x08, 0x83, 0xBD, 0x9B, 0x07, 0x3A, 0x13, 0x74, 0x86, 0xB3, 0x7A, 0x08,
        0x44, 0xB7, 0xE6, 0x52, 0xE6, 0xC0, 0x4E, 0x34, 0x53, 0x20, 0xD0, 0xED, 0xFD, 0xBC, 0x99,
        0x9D, 0xB5, 0xC6, 0x59, 0xBD, 0xB7, 0x8F, 0xF9, 0xFC, 0x6B, 0xEC, 0xD2, 0xBD, 0x40, 0x41,
        0xB3, 0x17, 0x79, 0x89, 0x9D, 0xB6, 0xC4, 0x59, 0x3D, 0x4E, 0x78, 0x98, 0x1B, 0x6D, 0x9B,
        0x91, 0x5D, 0x59, 0x96, 0x20, 0x8F, 0x50, 0xD6, 0x41, 0xD6, 0x35, 0x76, 0x1D, 0x0B, 0x59,
        0xA3, 0xC4, 0x67, 0xD6, 0x57, 0x50, 0x71, 0xDE, 0x70, 0x3F, 0xB1, 0x0B, 0xFA, 0x57, 0x15,
        0xDD, 0x82, 0xCE, 0x29, 0xFD, 0x37, 0x0C, 0x2D, 0xEE, 0x8C, 0x5D, 0xC7, 0xBC, 0x02, 0xB1,
        0xBA, 0x5A, 0x9E, 0x59, 0x7F, 0x13, 0x71, 0x0B, 0x12, 0xFB, 0xC9, 0xFD, 0xCB, 0x3E, 0x3A,
        0x35, 0xCC, 0x3B, 0x2E, 0x89, 0xAE, 0x71, 0x67, 0xEC, 0x3A, 0x64, 0x1E, 0xA3, 0xC6, 0x67,
        0xD6, 0xB7, 0x8C, 0xB1, 0xDA, 0xC2, 0xC8, 0xEA, 0x17, 0x10, 0xC8, 0x73, 0x54, 0x81, 0x75,
        0x34, 0xF9, 0x85, 0x2B, 0xEC, 0x3A, 0xE6, 0x15, 0x08, 0x8D, 0x78, 0x61, 0x64, 0x09, 0x44,
        0x00, 0x56, 0x62, 0xFA, 0x03, 0xA3, 0x40, 0xF6, 0x99, 0x80, 0x4D, 0x7B, 0x91, 0x5D, 0xC7,
        0xBC, 0x02, 0xA1, 0x11, 0x0F, 0x26, 0xBE, 0x22, 0x90, 0x36, 0x04, 0x96, 0xFD, 0xA1, 0x79,
        0x2C, 0xCE, 0x88, 0x03, 0x87, 0x5C, 0x42, 0x3B, 0xAF, 0x40, 0x68, 0xC4, 0x83, 0x89, 0xB5,
        0x49, 0x58, 0x6D, 0xEE, 0x12, 0xBB, 0x60, 0xDF, 0x24, 0xCC, 0x1F, 0xFD, 0xF8, 0xD9, 0x24,
        0xC1, 0xB6, 0x0F, 0xD9, 0x75, 0xCC, 0x2B, 0x68, 0x9D, 0x14, 0xEF, 0x3C, 0x33, 0x33, 0x48,
        0x6B, 0x6B, 0xE0, 0xFA, 0xBA, 0x61, 0x17, 0xAC, 0xCE, 0xCB, 0x33, 0x76, 0x9D, 0x53, 0x81,
        0x35, 0x3E, 0xF3, 0xE9, 0x39, 0xD8, 0x0B, 0x97, 0xFE, 0x13, 0x76, 0xDA, 0x06, 0xAE, 0x93,
        0x03, 0xCA, 0x7B, 0xD3, 0xF2, 0xB9, 0x66, 0xD6, 0x12, 0xD7, 0xE9, 0x4D, 0x5C, 0x2B, 0x56,
        0x52, 0x9F, 0x2E, 0xF9, 0x59, 0xE2, 0x60, 0xDB, 0x03, 0xE7, 0xE1, 0x3B, 0x5C, 0x27, 0x47,
        0xF0, 0x11, 0x9F, 0x3C, 0xE7, 0x6F, 0x79, 0x79, 0x0E, 0xFA, 0x88, 0x2D, 0x2D, 0x68, 0xFD,
        0x8C, 0xA1, 0xF7, 0xB2, 0xDB, 0x9D, 0xF3, 0xE8, 0x03, 0x5C, 0x27, 0x87, 0xE8, 0xCD, 0x86,
        0x4A, 0xBE, 0x90, 0x98, 0x77, 0x90, 0x89, 0xD4, 0x6C, 0x82, 0xC4, 0x72, 0x95, 0x06, 0xA8,
        0xAB, 0xF6, 0x2F, 0xD9, 0xB1, 0x7B, 0xB0, 0x2F, 0x0B, 0x94, 0xED, 0xFC, 0x3D, 0x96, 0x40,
        0x26, 0x12, 0x6B, 0x87, 0x95, 0x8C, 0x3F, 0x40, 0x57, 0xEB, 0x5F, 0xB0, 0x43, 0x77, 0x61,
        0xA7, 0x6A, 0x8E, 0xC0, 0x76, 0xFE, 0x2E, 0x1F, 0x0A, 0x54, 0x36, 0xFE, 0x00, 0x5D, 0xEE,
        0x5F, 0x9C, 0x83, 0xA5, 0x73, 0x05, 0x7B, 0x29, 0x39, 0x01, 0xBB, 0xE6, 0xEF, 0xB3, 0x36,
        0x09, 0xAB, 0xCD, 0xDD, 0x2E, 0x5B, 0x0B, 0x02, 0xC3, 0x8E, 0x7E, 0x36, 0x49, 0xB0, 0xED,
        0xC2, 0x0E, 0x5C, 0xC0, 0x6E, 0x04, 0x4D, 0x60, 0x93, 0x94, 0xFC, 0x05, 0x33, 0x83, 0xB4,
        0xB6, 0xB6, 0xCC, 0x3F, 0x87, 0x15, 0x54, 0x7B, 0xC9, 0x79, 0xE0, 0x0C, 0xD7, 0xC9, 0x71,
        0x7C, 0x3C, 0x8A, 0x7C, 0xD7, 0xC9, 0xBC, 0x3C, 0x07, 0xAF, 0xF3, 0x0C, 0xFF, 0x83, 0xA7,
        0x36, 0xFA, 0x1E, 0xB2, 0xC0, 0xAF, 0x9F, 0x83, 0x9D, 0x59, 0xD2, 0x60, 0xDB, 0x13, 0x43,
        0x13, 0x78, 0x62, 0xA1, 0xD2, 0x16, 0x49, 0x20, 0x13, 0x58, 0xEA, 0xAC, 0xF8, 0xE0, 0x1C,
        0x6C, 0xBC, 0x3E, 0xF7, 0x76, 0xA0, 0x82, 0x3C, 0xF9, 0x45, 0x81, 0x2C, 0xBC, 0xEB, 0x2C,
        0x98, 0x77, 0x90, 0x72, 0xAA, 0x09, 0x2E, 0x72, 0xD9, 0x39, 0xB4, 0x1F, 0x6F, 0x5C, 0xDC,
        0x8E, 0x40, 0x20, 0xCF, 0x7D, 0x51, 0x60, 0x35, 0x6A, 0xDD, 0x5D, 0x96, 0x40, 0x0A, 0x62,
        0x5D, 0x67, 0xA4, 0x45, 0xFB, 0x87, 0xC5, 0x7F, 0x4A, 0x20, 0xBF, 0x45, 0x04, 0xE2, 0x69,
        0x17, 0x7C, 0x45, 0x60, 0x11, 0xD4, 0xED, 0x89, 0x77, 0x04, 0xD2, 0x5E, 0x12, 0x58, 0xF6,
        0x07, 0xC1, 0x47, 0xAC, 0x4D, 0xC2, 0x6A, 0x73, 0xB7, 0x66, 0x36, 0x49, 0xB0, 0xED, 0x89,
        0xB7, 0x02, 0x69, 0x1A, 0x6D, 0xD1, 0x05, 0x32, 0x7F, 0xE4, 0x53, 0xE0, 0x90, 0x99, 0x41,
        0x5A, 0x5B, 0x5F, 0xE0, 0x8C, 0xCB, 0xC7, 0xCC, 0xC5, 0xCF, 0x97, 0x3C, 0x29, 0xCC, 0x8D,
        0xB6, 0xCD, 0x0E, 0xB7, 0x83, 0x7A, 0xF5, 0xFF, 0x87, 0x69, 0x03, 0x56, 0xB4, 0x78, 0x2E,
        0xA1, 0xAC, 0x63, 0x8F, 0x7F, 0x94, 0xB5, 0xC4, 0xBC, 0xD3, 0x58, 0xBB, 0xEC, 0xD7, 0x4F,
        0x03, 0x32, 0x54, 0xEB, 0x1F, 0xD8, 0x01, 0x15, 0x36, 0xAB, 0xC0, 0xD3, 0x73, 0xEE, 0x8C,
        0xF5, 0x4E, 0x73, 0x0B, 0xF6, 0x98, 0x51, 0x86, 0x01, 0xCB, 0xAD, 0xFA, 0x85, 0xC6, 0xA5,
        0xDB, 0xB0, 0x45, 0x3C, 0xC0, 0x6A, 0x71, 0x5F, 0x63, 0xDE, 0x41, 0x26, 0x52, 0xB3, 0x09,
        0x32, 0xCB, 0xA7, 0x4B, 0xD0, 0x80, 0xB1, 0xCA, 0xC0, 0xFE, 0xCA, 0x0E, 0x09, 0xD8, 0x50,
        0x7C, 0x69, 0x5A, 0xDC, 0x97, 0x58, 0x02, 0x99, 0x48, 0xAC, 0x2D, 0xE3, 0xAA, 0x0D, 0x88,
        0xB1, 0xA2, 0x69, 0xFE, 0xC6, 0xA5, 0xBB, 0x60, 0x8C, 0xAF, 0x0F, 0xD4, 0xE2, 0xBE, 0xC2,
        0x07, 0x02, 0xF1, 0x80, 0xD2, 0x96, 0x01, 0x6B, 0x5B, 0xFD, 0x03, 0x47, 0x77, 0xC1, 0x10,
        0xCF, 0x4B, 0xCB, 0x8D, 0xFC, 0x4F, 0xB0, 0x36, 0x09, 0xAB, 0xCD, 0x5D, 0xE6, 0x3A, 0x52,
        0x25, 0x0D, 0x28, 0xE8, 0xA5, 0x2F, 0xFE, 0x89, 0xAB, 0x17, 0xF4, 0x78, 0xE6, 0xAF, 0xF9,
        0xBF, 0xCE, 0xCC, 0x20, 0xAD, 0xAD, 0x81, 0x3D, 0x64, 0x1F, 0xB0, 0x00, 0x85, 0xE1, 0x77,
        0x5C, 0x65, 0xBB, 0x7B, 0xFC, 0xC5, 0x73, 0xEE, 0x8C, 0x97, 0xE7, 0xA0, 0x47, 0x94, 0x25,
        0xF4, 0x83, 0x57, 0x3D, 0x8A, 0x33, 0x6C, 0xBF, 0x86, 0xB5, 0xC4, 0x71, 0xDE, 0x70, 0x3F,
        0xB1, 0x67, 0xC2, 0xFF, 0xF4, 0x51, 0x08, 0xBB, 0xFA, 0xA1, 0x80, 0x6E, 0xB8, 0x5A, 0xDE,
        0x77, 0xF3, 0xF2, 0x1C, 0x6C, 0x33, 0x48, 0x00, 0x4F, 0xD2, 0x04, 0xDA, 0x0F, 0xF8, 0x1D,
        0x4D, 0x6B, 0xFB, 0x45, 0xCC, 0x3B, 0xC8, 0xF0, 0xD2, 0x00, 0x46, 0x56, 0xBF, 0x80, 0x40,
        0x9E, 0xA3, 0x0B, 0x7C, 0xE7, 0xED, 0x2F, 0x9F, 0x03, 0xE9, 0xE2, 0xC7, 0x36, 0x4F, 0x54,
        0x58, 0xCF, 0xA6, 0x16, 0xA8, 0x8E, 0xF0, 0x0C, 0x4B, 0x20, 0x02, 0xB0, 0x12, 0xD3, 0x1F,
        0xD8, 0x11, 0x88, 0x30, 0xC4, 0x48, 0x1D, 0x3F, 0xB6, 0x91, 0x55, 0xB8, 0x0B, 0x24, 0x8F,
        0xF6, 0x49, 0xBE, 0x22, 0x90, 0x76, 0x10, 0x88, 0x51, 0x1A, 0xE4, 0xD0, 0x19, 0xB7, 0x34,
        0xE0, 0x25, 0xAC, 0x4D, 0xC2, 0x6A, 0x73, 0x97, 0xB8, 0x0B, 0xA4, 0x21, 0xAE, 0x10, 0x76,
        0xF5, 0x23, 0x30, 0x60, 0x9B, 0x8E, 0x97, 0x30, 0x33, 0x48, 0x6B, 0x6B, 0xE0, 0xB6, 0x49,
        0xB8, 0x84, 0x26, 0xB0, 0xFA, 0x0D, 0xDB, 0xAF, 0x62, 0x66, 0x06, 0xE6, 0x46, 0x43, 0xDD,
        0x91, 0xB5, 0xC4, 0x75, 0x99, 0xEE, 0xC9, 0xCB, 0x73, 0xF0, 0x2E, 0xCC, 0x3B, 0xC8, 0x44,
        0x6A, 0x36, 0xC1, 0x1D, 0x59, 0x02, 0x99, 0x48, 0xAC, 0x9B, 0xF2, 0xFF, 0x40, 0xA0, 0x48,
        0x73, 0x79, 0x53, 0x66, 0x06, 0x69, 0x6D, 0xDD, 0x90, 0xFF, 0x03, 0xB5, 0xFC, 0x7F, 0x07,
        0xEA, 0x4F, 0xEE, 0xe4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60,
        0x82,
    ];

    assert_eq!(img.to_png().unwrap(), img_data);
}