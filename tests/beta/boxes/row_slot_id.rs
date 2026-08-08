use crate::common;

use gamma_rs::save::beta::StorageType;
use gamma_rs::save::beta::row_id::RowID;
use gamma_rs::save::beta::slot_id::SlotID;
use gvas::properties::array_property::ArrayProperty;

#[test]
fn reads_box_row_ids_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = [(0, 0), (1, 2), (2, 2)];

    for (idx, expected_row) in cases {
        let row_id = RowID::new(&gvas_file, StorageType::BOXES(1)).expect("row id wrapper exists");
        let actual = row_id.at_index(idx).expect("row id exists at index");

        assert_eq!(*actual, expected_row);
    }
}

#[test]
fn reads_box_row_id_list_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let row_id = RowID::new(&gvas_file, StorageType::BOXES(1)).expect("row id wrapper exists");
    let rows = match row_id.as_list().expect("row id list exists") {
        ArrayProperty::Ints { ints } => ints,
        _ => panic!("row id list has unexpected array type"),
    };

    assert_eq!(rows, &[0, 2, 2]);
}

#[test]
fn reads_box_slot_ids_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = [(0, 1), (1, 1), (2, 2)];

    for (idx, expected_slot) in cases {
        let slot_id = SlotID::new(&gvas_file, StorageType::BOXES(1)).expect("slot id wrapper exists");
        let actual = slot_id.at_index(idx).expect("slot id exists at index");

        assert_eq!(*actual, expected_slot);
    }
}

#[test]
fn reads_box_slot_id_list_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let slot_id = SlotID::new(&gvas_file, StorageType::BOXES(1)).expect("slot id wrapper exists");
    let slots = match slot_id.as_list().expect("slot id list exists") {
        ArrayProperty::Ints { ints } => ints,
        _ => panic!("slot id list has unexpected array type"),
    };

    assert_eq!(slots, &[1, 1, 2]);
}
