use crate::common;
use gamma_rs::save::beta::boxes::attack_lists::{
    attack_array, attack_at, attacks_at, parse_attack,
};
use gvas::GvasFile;

struct AttackCase<'a> {
    array_property_name: &'a str,
    pokemon_index: usize,
    move_idx: usize,
    expected_attack: &'a str,
}

fn generate_case<'a>(
    array_property_name: &'a str,
    pokemon_index: usize,
    move_idx: usize,
    expected_attack: &'a str,
) -> AttackCase<'a> {
    AttackCase {
        array_property_name,
        pokemon_index,
        move_idx,
        expected_attack,
    }
}

fn assert_attack_value(gvas_file: &GvasFile, case: &AttackCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_property_name)
        .expect("array property exists");
    let array = attack_array(property).expect("property is an array");
    let attacks = attacks_at(array, case.pokemon_index).expect("pokemon has an attacks array");
    let attack = attack_at(attacks, case.move_idx).expect("move has an attack value");
    let parsed_attack = parse_attack(attack).expect("attack value parses");

    assert_eq!(
        parsed_attack, case.expected_attack,
        "attack mismatch for {}[{}][{}]",
        case.array_property_name, case.pokemon_index, case.move_idx
    );
}

#[test]
fn reads_attack_lists_from_gvas_file() {
    let gvas_file = common::load_slot1();

    let cases = &[
        generate_case("Box1AttackLists", 1, 0, "Growl"),
        generate_case("PartyAttackLists", 0, 0, "Astonish"),
    ];

    assert!(!cases.is_empty(), "add attack list test cases");
    for case in cases {
        assert_attack_value(&gvas_file, case);
    }
}
