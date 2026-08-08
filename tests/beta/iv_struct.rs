use crate::common;

use gamma_rs::save::beta::StorageType;
use gamma_rs::save::beta::iv_struct::{IV, IVMut, get_ivs};
use gamma_rs::save::beta::pokemon::ivs::IVs;
use gamma_rs::utils::custom_struct::{get_struct_property_at_idx, get_struct_property_at_idx_mut};
use gvas::GvasFile;

struct IvCase<'a> {
    property_name: &'a str,
    storage_type: StorageType,
    idx: usize,
    expected_ivs: [i32; 6],
}

struct IvMutCase {
    storage_type: StorageType,
    idx: usize,
    iv: IVs,
    new_value: i32,
}

fn generate_case(
    property_name: &str,
    storage_type: StorageType,
    idx: usize,
    expected_ivs: [i32; 6],
) -> IvCase<'_> {
    IvCase {
        property_name,
        storage_type,
        idx,
        expected_ivs,
    }
}

fn generate_mut_case(storage_type: StorageType, idx: usize, iv: IVs, new_value: i32) -> IvMutCase {
    IvMutCase {
        storage_type,
        idx,
        iv,
        new_value,
    }
}

fn assert_raw_ivs(gvas_file: &GvasFile, case: &IvCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.property_name)
        .expect("iv property exists");
    let iv_struct = get_struct_property_at_idx(property, case.idx).expect("iv struct exists");
    let actual: Vec<i32> = get_ivs(iv_struct)
        .expect("iv values exist")
        .into_iter()
        .copied()
        .collect();

    assert_eq!(actual, case.expected_ivs);
}

fn assert_wrapper_ivs(gvas_file: &GvasFile, case: &IvCase<'_>) {
    let ivs = IV::new(gvas_file, case.storage_type.clone()).expect("iv wrapper exists");
    let actual: Vec<i32> = ivs
        .get_ivs_at(case.idx)
        .expect("iv values exist")
        .into_iter()
        .copied()
        .collect();

    assert_eq!(actual, case.expected_ivs);
}

fn assert_single_iv(gvas_file: &GvasFile, case: &IvCase<'_>, iv: IVs, expected_value: i32) {
    let ivs = IV::new(gvas_file, case.storage_type.clone()).expect("iv wrapper exists");
    let actual = ivs.get_iv_at(case.idx, iv).expect("single iv value exists");

    assert_eq!(*actual, expected_value);
}

fn assert_sets_iv(gvas_file: &GvasFile, case: &IvMutCase) {
    let mut cloned_gvas_file = gvas_file.clone();
    let mut ivs = IVMut::new(&mut cloned_gvas_file, case.storage_type.clone())
        .expect("iv mut wrapper exists");

    assert!(
        ivs.set_iv_at(case.idx, case.iv.clone(), case.new_value)
            .is_ok()
    );
    drop(ivs);

    let ivs = IV::new(&cloned_gvas_file, case.storage_type.clone()).expect("iv wrapper exists");
    let actual = ivs
        .get_iv_at(case.idx, case.iv.clone())
        .expect("single iv value exists");

    assert_eq!(*actual, case.new_value);
}

fn assert_struct_property_at_idx(gvas_file: &GvasFile, case: &IvCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.property_name)
        .expect("iv property exists");
    let iv_struct = get_struct_property_at_idx(property, case.idx).expect("iv struct exists");
    let actual: Vec<i32> = get_ivs(iv_struct)
        .expect("iv values exist")
        .into_iter()
        .copied()
        .collect();

    assert_eq!(actual, case.expected_ivs);
}

fn assert_struct_property_at_idx_mut(gvas_file: &GvasFile, case: &IvMutCase) {
    let mut cloned_gvas_file = gvas_file.clone();
    let property_name = match case.storage_type {
        StorageType::PARTY => "PartyIVstruct",
        StorageType::BOXES(_) => "Box1IV",
    };
    let property = cloned_gvas_file
        .properties
        .get_mut(property_name)
        .expect("iv property exists");
    let iv_struct = get_struct_property_at_idx_mut(property, case.idx).expect("iv struct exists");

    match iv_struct.get_custom_struct_mut() {
        Some(map) => {
            let values = map
                .iter_mut()
                .find(|(key, _)| key.starts_with(case.iv.as_str()))
                .expect("iv entry exists")
                .1;
            let property = values.first_mut().expect("iv value exists");
            property.get_int_mut().expect("iv value is an int").value = case.new_value;
        }
        None => panic!("iv struct has unexpected type"),
    }

    let property = cloned_gvas_file
        .properties
        .get(property_name)
        .expect("iv property exists");
    let iv_struct = get_struct_property_at_idx(property, case.idx).expect("iv struct exists");
    let actual: Vec<i32> = get_ivs(iv_struct)
        .expect("iv values exist")
        .into_iter()
        .copied()
        .collect();

    assert_eq!(actual[case.iv.clone().get_index()], case.new_value);
}

#[test]
fn reads_raw_ivs_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = [
        generate_case(
            "PartyIVstruct",
            StorageType::PARTY,
            2,
            [24, 15, 23, 31, 16, 24],
        ),
        generate_case("Box1IV", StorageType::BOXES(1), 0, [31, 31, 31, 31, 31, 31]),
    ];

    for case in cases {
        assert_raw_ivs(&gvas_file, &case);
    }
}

#[test]
fn gets_struct_property_at_idx() {
    let gvas_file = common::load_slot1();
    let cases = [
        generate_case(
            "PartyIVstruct",
            StorageType::PARTY,
            2,
            [24, 15, 23, 31, 16, 24],
        ),
        generate_case("Box1IV", StorageType::BOXES(1), 0, [31, 31, 31, 31, 31, 31]),
    ];

    for case in cases {
        assert_struct_property_at_idx(&gvas_file, &case);
    }
}

#[test]
fn gets_struct_property_at_idx_mut() {
    let gvas_file = common::load_slot1();
    let cases = [
        generate_mut_case(StorageType::PARTY, 2, IVs::ATK, 30),
        generate_mut_case(StorageType::BOXES(1), 0, IVs::SPEED, 12),
    ];

    for case in cases {
        assert_struct_property_at_idx_mut(&gvas_file, &case);
    }
}

#[test]
fn reads_ivs_from_wrapper() {
    let gvas_file = common::load_slot1();
    let cases = [
        generate_case(
            "PartyIVstruct",
            StorageType::PARTY,
            2,
            [24, 15, 23, 31, 16, 24],
        ),
        generate_case("Box1IV", StorageType::BOXES(1), 0, [31, 31, 31, 31, 31, 31]),
    ];

    for case in cases {
        assert_wrapper_ivs(&gvas_file, &case);
    }
}

#[test]
fn reads_single_ivs_from_wrapper() {
    let gvas_file = common::load_slot1();
    let party_case = generate_case(
        "PartyIVstruct",
        StorageType::PARTY,
        2,
        [24, 15, 23, 31, 16, 24],
    );
    let box_case = generate_case("Box1IV", StorageType::BOXES(1), 0, [31, 31, 31, 31, 31, 31]);

    assert_single_iv(&gvas_file, &party_case, IVs::SPEED, 24);
    assert_single_iv(&gvas_file, &box_case, IVs::HP, 31);
}

#[test]
fn sets_ivs_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = [
        generate_mut_case(StorageType::PARTY, 2, IVs::ATK, 30),
        generate_mut_case(StorageType::BOXES(1), 0, IVs::SPEED, 12),
    ];

    for case in cases {
        assert_sets_iv(&gvas_file, &case);
    }
}
