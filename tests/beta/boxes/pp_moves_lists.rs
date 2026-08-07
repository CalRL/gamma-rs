use crate::common;

use gamma_rs::save::beta::pp_moves_lists::{
    current_pp_at, max_pp_at, moves_array, moves_at,
};
use gvas::GvasFile;

struct PpMoveCase<'a> {
    array_property_name: &'a str,
    pokemon_index: usize,
    move_idx: usize,
    expected_current_pp: i32,
    expected_max_pp: i32,
}

fn generate_case(
    array_property_name: &str,
    pokemon_index: usize,
    move_idx: usize,
    expected_current_pp: i32,
    expected_max_pp: i32,
) -> PpMoveCase<'_> {
    PpMoveCase {
        array_property_name,
        pokemon_index,
        move_idx,
        expected_current_pp,
        expected_max_pp,
    }
}

fn assert_pp_values(gvas_file: &GvasFile, case: &PpMoveCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_property_name)
        .expect("array property exists");
    let array = moves_array(property).expect("property is an array");
    let moves = moves_at(array, case.pokemon_index).expect("pokemon has a moves array");

    assert_eq!(
        current_pp_at(moves, case.move_idx).copied(),
        Some(case.expected_current_pp),
        "current pp mismatch for {}[{}][{}]",
        case.array_property_name,
        case.pokemon_index,
        case.move_idx
    );
    assert_eq!(
        max_pp_at(moves, case.move_idx).copied(),
        Some(case.expected_max_pp),
        "max pp mismatch for {}[{}][{}]",
        case.array_property_name,
        case.pokemon_index,
        case.move_idx
    );
}

#[test]
fn reads_pp_move_lists_from_gvas_file() {
    let gvas_file = common::load_slot1();

    let cases = &[
        generate_case("Box1PPMovesLists", 1, 0, 15, 15),
        generate_case("PartyPPMovesLists", 0, 0, 10, 15),
    ];

    assert!(!cases.is_empty(), "add PP move list test cases");
    for case in cases {
        assert_pp_values(&gvas_file, case);
    }
}
