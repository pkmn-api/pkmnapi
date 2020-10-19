use pkmnapi_db::patch::*;
use pkmnapi_db::*;

mod common;

macro_rules! set_mart_items_test {
    (
        $test_name:ident,
        $mart_id:expr,
        $mart_items:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_mart_items(&$mart_id, &$mart_items) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for mart ID: {}",
                    $mart_id
                ),
                Err(_) => panic!(format!("Could not find mart ID: {}", $mart_id)),
            };
        }
    };
}

set_mart_items_test!(
    set_mart_items_0,
    0,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03)
    ],
    0x2444,
    vec![0x00, 0x01, 0x02, 0x03]
);
set_mart_items_test!(
    set_mart_items_1,
    1,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05),
        MartItem::ITEM(0x06)
    ],
    0x244B,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
);
set_mart_items_test!(
    set_mart_items_2,
    2,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05),
        MartItem::ITEM(0x06)
    ],
    0x2455,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
);
set_mart_items_test!(
    set_mart_items_3,
    3,
    vec![MartItem::ITEM(0x00)],
    0x245F,
    vec![0x00]
);
set_mart_items_test!(
    set_mart_items_4,
    4,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05)
    ],
    0x2463,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]
);
set_mart_items_test!(
    set_mart_items_5,
    5,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05),
        MartItem::ITEM(0x06),
        MartItem::ITEM(0x07),
        MartItem::ITEM(0x08)
    ],
    0x246C,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
);
set_mart_items_test!(
    set_mart_items_6,
    6,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05),
        MartItem::ITEM(0x06),
        MartItem::ITEM(0x07),
        MartItem::ITEM(0x08)
    ],
    0x2478,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
);
set_mart_items_test!(
    set_mart_items_7,
    7,
    vec![
        MartItem::TM(0x01),
        MartItem::TM(0x02),
        MartItem::TM(0x03),
        MartItem::TM(0x04),
        MartItem::TM(0x05),
        MartItem::TM(0x06),
        MartItem::TM(0x07),
        MartItem::TM(0x08),
        MartItem::TM(0x09)
    ],
    0x2484,
    vec![0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE, 0xCF, 0xD0, 0xD1]
);
set_mart_items_test!(
    set_mart_items_8,
    8,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04)
    ],
    0x2490,
    vec![0x00, 0x01, 0x02, 0x03, 0x04]
);
set_mart_items_test!(
    set_mart_items_9,
    9,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05),
        MartItem::ITEM(0x06)
    ],
    0x2498,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
);
set_mart_items_test!(
    set_mart_items_10,
    10,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04)
    ],
    0x24A2,
    vec![0x00, 0x01, 0x02, 0x03, 0x04]
);
set_mart_items_test!(
    set_mart_items_11,
    11,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05)
    ],
    0x24AA,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]
);
set_mart_items_test!(
    set_mart_items_12,
    12,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04)
    ],
    0x24B3,
    vec![0x00, 0x01, 0x02, 0x03, 0x04]
);
set_mart_items_test!(
    set_mart_items_13,
    13,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05),
        MartItem::ITEM(0x06)
    ],
    0x24BB,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
);
set_mart_items_test!(
    set_mart_items_14,
    14,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05)
    ],
    0x24C5,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]
);
set_mart_items_test!(
    set_mart_items_15,
    15,
    vec![
        MartItem::ITEM(0x00),
        MartItem::ITEM(0x01),
        MartItem::ITEM(0x02),
        MartItem::ITEM(0x03),
        MartItem::ITEM(0x04),
        MartItem::ITEM(0x05),
        MartItem::ITEM(0x06)
    ],
    0x24CE,
    vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
);
