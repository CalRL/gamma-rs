use crate::common;

use gamma_rs::save::beta::StorageType;
use gamma_rs::save::beta::shiny_list::{ShinyList, ShinyListMut, get_shiny_at};
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;

struct ShinyCase<'a> {
    property_name: &'a str,
    storage_type: StorageType,
    idx: usize,
}

fn generate_case(property_name: &str, storage_type: StorageType, idx: usize) -> ShinyCase<'_> {
    ShinyCase {
        property_name,
        storage_type,
        idx,
    }
}

fn raw_shiny_at(gvas_file: &GvasFile, property_name: &str, idx: usize) -> bool {
    let property = gvas_file
        .properties
        .get(property_name)
        .expect("shiny list property exists");
    let array = property
        .get_array()
        .expect("shiny list property is an array");

    match array {
        ArrayProperty::Bools { bools } => *bools.get(idx).expect("shiny value exists at index"),
        _ => panic!("shiny list property has unexpected array type"),
    }
}

fn assert_shiny_value(gvas_file: &GvasFile, case: &ShinyCase<'_>) {
    let shiny_list =
        ShinyList::new(gvas_file, case.storage_type.clone()).expect("shiny list wrapper exists");
    let expected = raw_shiny_at(gvas_file, case.property_name, case.idx);
    let actual = shiny_list
        .get_shiny_at(case.idx)
        .expect("shiny value exists at index");

    assert_eq!(*actual, expected);
}

fn assert_raw_shiny_value(gvas_file: &GvasFile, case: &ShinyCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.property_name)
        .expect("shiny list property exists");
    let array = property
        .get_array()
        .expect("shiny list property is an array");
    let expected = raw_shiny_at(gvas_file, case.property_name, case.idx);

    assert_eq!(get_shiny_at(array, case.idx).copied(), Some(expected));
}

fn assert_sets_shiny(gvas_file: &GvasFile, case: &ShinyCase<'_>, value: bool) {
    let mut cloned_gvas_file = gvas_file.clone();
    let mut shiny_list = ShinyListMut::new(&mut cloned_gvas_file, case.storage_type.clone())
        .expect("shiny list mut wrapper exists");

    assert!(shiny_list.set_shiny_at(case.idx, value).is_ok());
    drop(shiny_list);

    let shiny_list = ShinyList::new(&cloned_gvas_file, case.storage_type.clone())
        .expect("shiny list wrapper exists");
    let actual = shiny_list
        .get_shiny_at(case.idx)
        .expect("shiny value exists at index");

    assert_eq!(*actual, value);
}

#[test]
fn reads_shiny_lists_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        generate_case("PartyShinyList", StorageType::PARTY, 0),
        generate_case("Box1ShinyList", StorageType::BOXES(1), 0),
    ];

    for case in cases {
        assert_shiny_value(&gvas_file, case);
    }
}

#[test]
fn reads_raw_shiny_values_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        generate_case("PartyShinyList", StorageType::PARTY, 0),
        generate_case("Box1ShinyList", StorageType::BOXES(1), 0),
    ];

    for case in cases {
        assert_raw_shiny_value(&gvas_file, case);
    }
}

#[test]
fn sets_shiny_values_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        (generate_case("PartyShinyList", StorageType::PARTY, 0), true),
        (
            generate_case("Box1ShinyList", StorageType::BOXES(1), 0),
            true,
        ),
    ];

    for (case, value) in cases {
        assert_sets_shiny(&gvas_file, case, *value);
    }
}
