use crate::common;

use gamma_rs::save::beta::boxes::pokemon_id::{id_array, id_array_mut, id_at, id_at_mut};
use gvas::GvasFile;

struct IdCase<'a> {
    array_name: &'a str,
    idx: usize,
    expected_id: i32,
}

struct IdMutCase<'a> {
    array_name: &'a str,
    idx: usize,
    new_id: i32,
}

fn generate_case(array_name: &str, idx: usize, expected_id: i32) -> IdCase<'_> {
    IdCase {
        array_name,
        idx,
        expected_id,
    }
}

fn generate_mut_case(array_name: &str, idx: usize, new_id: i32) -> IdMutCase<'_> {
    IdMutCase {
        array_name,
        idx,
        new_id,
    }
}

fn assert_id(gvas_file: &GvasFile, case: &IdCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let array = id_array(property).expect("property is an array");
    let id = id_at(array, case.idx).expect("pokemon ID exists");

    assert_eq!(
        *id, case.expected_id,
        "pokemon ID mismatch for {}[{}]",
        case.array_name, case.idx
    );
}

fn assert_id_mut(gvas_file: &GvasFile, case: &IdMutCase<'_>) {
    let mut cloned_gvas_file = gvas_file.clone();
    let property = cloned_gvas_file
        .properties
        .get_mut(case.array_name)
        .expect("array property exists");
    let array = id_array_mut(property).expect("property is an array");
    let id = id_at_mut(array, case.idx).expect("pokemon ID exists");
    *id = case.new_id;

    assert_eq!(
        *id, case.new_id,
        "pokemon ID mutation mismatch for {}[{}]",
        case.array_name, case.idx
    );
}

#[test]
fn reads_pokemon_ids_from_gvas_file() {
    let gvas_file = common::load_slot1();

    let cases = &[
        generate_case("Box1PokemonID", 0, 0),
        generate_case("Box1PokemonID", 1, 765312),
        generate_case("PartyPokemonID", 0, 981811),
    ];

    assert!(!cases.is_empty(), "add pokemon ID test cases");
    for case in cases {
        assert_id(&gvas_file, case);
    }
}

#[test]
fn mutates_pokemon_ids_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();

    let cases = &[generate_mut_case("PartyPokemonID", 0, 123456)];

    assert!(!cases.is_empty(), "add pokemon ID mutation test cases");
    for case in cases {
        assert_id_mut(&gvas_file, case);
    }
}
