use crate::common;

use gamma_rs::save::beta::StorageType;
use gamma_rs::save::beta::gender::gender_string_at;
use gamma_rs::save::beta::pokemon::gender::Gender;
use gamma_rs::save::beta::pokemon_gender::{PokemonGender, PokemonGenderMut};
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;

struct GenderCase<'a> {
    property_name: &'a str,
    storage_type: StorageType,
    idx: usize,
    expected_gender: &'a str,
}

fn generate_case<'a>(
    property_name: &'a str,
    storage_type: StorageType,
    idx: usize,
    expected_gender: &'a str,
) -> GenderCase<'a> {
    GenderCase {
        property_name,
        storage_type,
        idx,
        expected_gender,
    }
}

fn gender_array<'a>(gvas_file: &'a GvasFile, property_name: &str) -> &'a ArrayProperty {
    gvas_file
        .properties
        .get(property_name)
        .expect("pokemon gender property exists")
        .get_array()
        .expect("pokemon gender property is an array")
}

fn assert_gender_value(gvas_file: &GvasFile, case: &GenderCase<'_>) {
    let pokemon_gender = PokemonGender::new(gvas_file, case.storage_type.clone())
        .expect("pokemon gender wrapper exists");
    let actual = pokemon_gender
        .get_gender_at(case.idx)
        .expect("pokemon gender exists at index");

    assert_eq!(actual, case.expected_gender);
}

fn assert_raw_gender_value(gvas_file: &GvasFile, case: &GenderCase<'_>) {
    let array = gender_array(gvas_file, case.property_name);

    assert_eq!(
        gender_string_at(array, case.idx).map(String::as_str),
        Some(case.expected_gender)
    );
}

fn assert_sets_gender(gvas_file: &GvasFile, case: &GenderCase<'_>, gender: Gender) {
    let mut cloned_gvas_file = gvas_file.clone();
    let mut pokemon_gender =
        PokemonGenderMut::new(&mut cloned_gvas_file, case.storage_type.clone())
            .expect("pokemon gender mut wrapper exists");
    let expected_gender = gender.as_game_enum().to_string();

    assert!(pokemon_gender.set_gender_at(gender, case.idx).is_ok());
    drop(pokemon_gender);

    let pokemon_gender = PokemonGender::new(&cloned_gvas_file, case.storage_type.clone())
        .expect("pokemon gender wrapper exists");
    let actual = pokemon_gender
        .get_gender_at(case.idx)
        .expect("pokemon gender exists at index");

    assert_eq!(actual, &expected_gender);
}

#[test]
fn reads_pokemon_gender_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = [
        generate_case(
            "Box1Gender",
            StorageType::BOXES(1),
            0,
            "ENUM_Gender::NewEnumerator0",
        ),
        generate_case(
            "PartyGender",
            StorageType::PARTY,
            1,
            "ENUM_Gender::NewEnumerator1",
        ),
    ];

    for case in cases {
        assert_gender_value(&gvas_file, &case);
    }
}

#[test]
fn reads_raw_pokemon_gender_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = [
        generate_case(
            "Box1Gender",
            StorageType::BOXES(1),
            0,
            "ENUM_Gender::NewEnumerator0",
        ),
        generate_case(
            "PartyGender",
            StorageType::PARTY,
            1,
            "ENUM_Gender::NewEnumerator1",
        ),
    ];

    for case in cases {
        assert_raw_gender_value(&gvas_file, &case);
    }
}

#[test]
fn sets_pokemon_gender_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = [
        (
            generate_case(
                "Box1Gender",
                StorageType::BOXES(1),
                0,
                "ENUM_Gender::NewEnumerator0",
            ),
            Gender::Female,
        ),
        (
            generate_case(
                "PartyGender",
                StorageType::PARTY,
                1,
                "ENUM_Gender::NewEnumerator1",
            ),
            Gender::Male,
        ),
    ];

    for (case, gender) in cases {
        assert_sets_gender(&gvas_file, &case, gender);
    }
}
