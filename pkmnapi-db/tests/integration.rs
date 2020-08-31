use pkmnapi_db::header::*;
use pkmnapi_db::*;

#[test]
fn new_success() {
    let rom = vec![0u8; 1024];

    PkmnapiDB::new(&rom).unwrap();
}

#[test]
fn new_failure() {
    let rom = vec![];

    match PkmnapiDB::new(&rom) {
        Err(e) => assert_eq!(e, "Header too small"),
        _ => {}
    };
}

#[test]
fn header_success() {
    let rom = [
        vec![0u8; 0x100],             // padding
        vec![0x00, 0xC3, 0x50, 0x01], // entry_point
        vec![
            // logo
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ],
        "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
        vec![0u8; 5],                                                // title padding
        vec![0x30u8, 0x31u8],                                        // new_licensee_code
        vec![0u8],                                                   // sgb_flag
        vec![0u8],                                                   // cartridge_type
        vec![0x05u8],                                                // rom_size
        vec![0u8],                                                   // ram_size
        vec![0x01u8],                                                // destination_code
        vec![0x01u8],                                                // old_licensee_code
        vec![0x01u8],                                                // mask_rom_version_number
        vec![0x60],                                                  // header_checksum
        vec![0x1A, 0x41],                                            // global_checksum
    ]
    .concat();
    let db = PkmnapiDB::new(&rom).unwrap();
    let header = Header::from(&rom).unwrap();

    assert_eq!(db.header, header);
}

#[test]
fn header_verify_success() {
    let rom = [
        vec![0u8; 0x100],             // padding
        vec![0x00, 0xC3, 0x50, 0x01], // entry_point
        vec![
            // logo
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ],
        "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
        vec![0u8; 5],                                                // title padding
        vec![0x30u8, 0x31u8],                                        // new_licensee_code
        vec![0u8],                                                   // sgb_flag
        vec![0u8],                                                   // cartridge_type
        vec![0x05u8],                                                // rom_size
        vec![0u8],                                                   // ram_size
        vec![0x01u8],                                                // destination_code
        vec![0x01u8],                                                // old_licensee_code
        vec![0x01u8],                                                // mask_rom_version_number
        vec![0x60],                                                  // header_checksum
        vec![0x1A, 0x41],                                            // global_checksum
    ]
    .concat();
    let db = PkmnapiDB::new(&rom).unwrap();

    assert_eq!(db.header.verify_checksum(), true);
}

#[test]
fn header_verify_fail() {
    let rom = [
        vec![0u8; 0x100],             // padding
        vec![0x00, 0xC3, 0x50, 0x01], // entry_point
        vec![
            // logo
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ],
        "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
        vec![0u8; 5],                                                // title padding
        vec![0x30u8, 0x31u8],                                        // new_licensee_code
        vec![0u8],                                                   // sgb_flag
        vec![0u8],                                                   // cartridge_type
        vec![0x05u8],                                                // rom_size
        vec![0u8],                                                   // ram_size
        vec![0x01u8],                                                // destination_code
        vec![0x01u8],                                                // old_licensee_code
        vec![0x01u8],                                                // mask_rom_version_number
        vec![0x42],                                                  // (wrong) header_checksum
        vec![0x1A, 0x41],                                            // global_checksum
    ]
    .concat();
    let db = PkmnapiDB::new(&rom).unwrap();

    assert_eq!(db.header.verify_checksum(), false);
}

#[test]
fn verify_checksum_success() {
    let rom = [
        vec![0u8; 0x100],             // padding
        vec![0x00, 0xC3, 0x50, 0x01], // entry_point
        vec![
            // logo
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ],
        "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
        vec![0u8; 5],                                                // title padding
        vec![0x30u8, 0x31u8],                                        // new_licensee_code
        vec![0u8],                                                   // sgb_flag
        vec![0u8],                                                   // cartridge_type
        vec![0x05u8],                                                // rom_size
        vec![0u8],                                                   // ram_size
        vec![0x01u8],                                                // destination_code
        vec![0x01u8],                                                // old_licensee_code
        vec![0x01u8],                                                // mask_rom_version_number
        vec![0x60],                                                  // header_checksum
        vec![0x1A, 0x41],                                            // global_checksum
    ]
    .concat();
    let mut db = PkmnapiDB::new(&rom).unwrap();

    db.header.global_checksum = 0x1A41;

    assert_eq!(db.verify_checksum(), true);
}

#[test]
fn verify_checksum_fail() {
    let rom = [
        vec![0u8; 0x100],             // padding
        vec![0x00, 0xC3, 0x50, 0x01], // entry_point
        vec![
            // logo
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ],
        "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
        vec![0u8; 5],                                                // title padding
        vec![0x30u8, 0x31u8],                                        // new_licensee_code
        vec![0u8],                                                   // sgb_flag
        vec![0u8],                                                   // cartridge_type
        vec![0x05u8],                                                // rom_size
        vec![0u8],                                                   // ram_size
        vec![0x01u8],                                                // destination_code
        vec![0x01u8],                                                // old_licensee_code
        vec![0x01u8],                                                // mask_rom_version_number
        vec![0x60],                                                  // header_checksum
        vec![0x1A, 0x41],                                            // global_checksum
    ]
    .concat();
    let mut db = PkmnapiDB::new(&rom).unwrap();

    db.header.global_checksum = 0x1234;

    assert_eq!(db.verify_checksum(), false);
}

#[test]
fn verify_hash_success() {
    let rom = [
        vec![0u8; 0x100],             // padding
        vec![0x00, 0xC3, 0x50, 0x01], // entry_point
        vec![
            // logo
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ],
        "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
        vec![0u8; 5],                                                // title padding
        vec![0x30u8, 0x31u8],                                        // new_licensee_code
        vec![0u8],                                                   // sgb_flag
        vec![0u8],                                                   // cartridge_type
        vec![0x05u8],                                                // rom_size
        vec![0u8],                                                   // ram_size
        vec![0x01u8],                                                // destination_code
        vec![0x01u8],                                                // old_licensee_code
        vec![0x01u8],                                                // mask_rom_version_number
        vec![0x60],                                                  // header_checksum
        vec![0x1A, 0x41],                                            // global_checksum
    ]
    .concat();
    let db = PkmnapiDB::new(&rom).unwrap();

    assert_eq!(db.verify_hash("b933af3d953bedd6ed3911ef6724cfa2"), true);
}

#[test]
fn verify_hash_fail() {
    let rom = [
        vec![0u8; 0x100],             // padding
        vec![0x00, 0xC3, 0x50, 0x01], // entry_point
        vec![
            // logo
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ],
        "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
        vec![0u8; 5],                                                // title padding
        vec![0x30u8, 0x31u8],                                        // new_licensee_code
        vec![0u8],                                                   // sgb_flag
        vec![0u8],                                                   // cartridge_type
        vec![0x05u8],                                                // rom_size
        vec![0u8],                                                   // ram_size
        vec![0x01u8],                                                // destination_code
        vec![0x01u8],                                                // old_licensee_code
        vec![0x01u8],                                                // mask_rom_version_number
        vec![0x60],                                                  // header_checksum
        vec![0x1A, 0x41],                                            // global_checksum
    ]
    .concat();
    let db = PkmnapiDB::new(&rom).unwrap();

    assert_eq!(db.verify_hash("0123456789abcdef0123456789abcdef"), false);
}
