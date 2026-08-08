use crate::common;

use gamma_rs::save::beta::StorageType;
use gamma_rs::save::beta::pokemon::stats::Stats;
use gamma_rs::save::beta::pokemon_info::{
    PokemonInfo, PokemonInfoMut, get_is_fainted, get_level, get_name, get_nature, get_primary_type,
    get_secondary_type, get_stat, get_stat_mut, get_stats,
};
use gamma_rs::utils::custom_struct::get_struct_property_at_idx;
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::struct_property::StructPropertyValue;

struct NameCase<'a> {
    array_name: &'a str,
    idx: usize,
    expected_name: &'a str,
}

struct NatureCase<'a> {
    array_name: &'a str,
    idx: usize,
    expected_nature: &'a str,
}

struct TypeCase<'a> {
    array_name: &'a str,
    idx: usize,
    expected_primary_type: &'a str,
    expected_secondary_type: &'a str,
}

struct FaintedCase<'a> {
    array_name: &'a str,
    idx: usize,
    expected_is_fainted: bool,
}

struct LevelCase<'a> {
    array_name: &'a str,
    idx: usize,
    expected_level: i32,
}

struct PartyLevelCase {
    idx: usize,
    expected_level: i32,
}

struct StatCase<'a> {
    array_name: &'a str,
    idx: usize,
    stat: Stats,
    expected_value: f64,
}

struct StatMutCase<'a> {
    array_name: &'a str,
    idx: usize,
    stat: Stats,
    new_value: f64,
}

struct StatsCase<'a> {
    array_name: &'a str,
    idx: usize,
    stat: Stats,
    expected_value: f64,
}

struct SetStatCase {
    idx: usize,
    stat: Stats,
    new_value: f64,
}

struct SetNameCase<'a> {
    idx: usize,
    new_name: &'a str,
}

fn generate_case<'a>(array_name: &'a str, idx: usize, expected_name: &'a str) -> NameCase<'a> {
    NameCase {
        array_name,
        idx,
        expected_name,
    }
}

fn generate_nature_case<'a>(
    array_name: &'a str,
    idx: usize,
    expected_nature: &'a str,
) -> NatureCase<'a> {
    NatureCase {
        array_name,
        idx,
        expected_nature,
    }
}

fn generate_type_case<'a>(
    array_name: &'a str,
    idx: usize,
    expected_primary_type: &'a str,
    expected_secondary_type: &'a str,
) -> TypeCase<'a> {
    TypeCase {
        array_name,
        idx,
        expected_primary_type,
        expected_secondary_type,
    }
}

fn generate_fainted_case(
    array_name: &str,
    idx: usize,
    expected_is_fainted: bool,
) -> FaintedCase<'_> {
    FaintedCase {
        array_name,
        idx,
        expected_is_fainted,
    }
}

fn generate_level_case(array_name: &str, idx: usize, expected_level: i32) -> LevelCase<'_> {
    LevelCase {
        array_name,
        idx,
        expected_level,
    }
}

fn generate_party_level_case(idx: usize, expected_level: i32) -> PartyLevelCase {
    PartyLevelCase {
        idx,
        expected_level,
    }
}

fn generate_stat_case(
    array_name: &str,
    idx: usize,
    stat: Stats,
    expected_value: f64,
) -> StatCase<'_> {
    StatCase {
        array_name,
        idx,
        stat,
        expected_value,
    }
}

fn generate_stat_mut_case(
    array_name: &str,
    idx: usize,
    stat: Stats,
    new_value: f64,
) -> StatMutCase<'_> {
    StatMutCase {
        array_name,
        idx,
        stat,
        new_value,
    }
}

fn generate_stats_case(
    array_name: &str,
    idx: usize,
    stat: Stats,
    expected_value: f64,
) -> StatsCase<'_> {
    StatsCase {
        array_name,
        idx,
        stat,
        expected_value,
    }
}

fn generate_set_stat_case(idx: usize, stat: Stats, new_value: f64) -> SetStatCase {
    SetStatCase {
        idx,
        stat,
        new_value,
    }
}

fn generate_set_name_case(idx: usize, new_name: &str) -> SetNameCase<'_> {
    SetNameCase { idx, new_name }
}

fn get_pokemon_info_mut<'a>(
    gvas_file: &'a mut GvasFile,
    array_name: &str,
    idx: usize,
) -> &'a mut StructPropertyValue {
    let property = gvas_file
        .properties
        .get_mut(array_name)
        .expect("array property exists");
    let array = property.get_array_mut().expect("property is an array");
    match array {
        ArrayProperty::Structs { structs, .. } => structs.get_mut(idx),
        _ => None,
    }
    .expect("pokemon info exists")
}

fn assert_name(gvas_file: &GvasFile, case: &NameCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let name = get_name(pokemon_info).expect("pokemon name exists");

    assert_eq!(
        name, case.expected_name,
        "name mismatch for {}[{}]",
        case.array_name, case.idx
    );
}

fn assert_nature(gvas_file: &GvasFile, case: &NatureCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let nature = get_nature(pokemon_info).expect("pokemon nature exists");

    assert_eq!(
        nature, case.expected_nature,
        "nature mismatch for {}[{}]",
        case.array_name, case.idx
    );
}

fn assert_types(gvas_file: &GvasFile, case: &TypeCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let primary_type = get_primary_type(pokemon_info).expect("pokemon primary type exists");
    let secondary_type = get_secondary_type(pokemon_info).expect("pokemon secondary type exists");

    assert_eq!(primary_type, case.expected_primary_type);
    assert_eq!(secondary_type, case.expected_secondary_type);
}

fn assert_is_fainted(gvas_file: &GvasFile, case: &FaintedCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let is_fainted = get_is_fainted(pokemon_info).expect("pokemon isFainted exists");

    assert_eq!(is_fainted, case.expected_is_fainted);
}

fn assert_level(gvas_file: &GvasFile, case: &LevelCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let level = get_level(pokemon_info).expect("pokemon level exists");

    assert_eq!(*level, case.expected_level);
}

fn assert_party_level(gvas_file: &GvasFile, case: &PartyLevelCase) {
    let pokemon_info =
        PokemonInfo::new(gvas_file, StorageType::PARTY).expect("party pokemon info exists");
    let level = pokemon_info
        .get_level(case.idx)
        .expect("pokemon level exists");

    assert_eq!(*level, case.expected_level);
}

fn assert_stat(gvas_file: &GvasFile, case: &StatCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let value = get_stat(pokemon_info, case.stat.clone()).expect("pokemon stat exists");

    assert!((value - case.expected_value).abs() < f64::EPSILON);
}

fn assert_stat_mut(case: &StatMutCase<'_>) {
    let mut gvas_file = common::load_slot1();
    let pokemon_info = get_pokemon_info_mut(&mut gvas_file, case.array_name, case.idx);
    let value = get_stat_mut(pokemon_info, case.stat.clone()).expect("pokemon stat exists");
    *value = case.new_value;

    assert!((*value - case.new_value).abs() < f64::EPSILON);
}

fn assert_stats(gvas_file: &GvasFile, case: &StatsCase<'_>) {
    let property = gvas_file
        .properties
        .get(case.array_name)
        .expect("array property exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let stats = get_stats(pokemon_info).expect("pokemon stats exist");
    let value = stats
        .values
        .get(&case.stat)
        .expect("stat exists in stat struct");

    assert!((*value - case.expected_value).abs() < f64::EPSILON);
}

fn assert_set_stat(gvas_file: &GvasFile, case: &SetStatCase) {
    let mut cloned_gvas_file = gvas_file.clone();
    let mut pokemon_info = PokemonInfoMut::new(&mut cloned_gvas_file, StorageType::PARTY)
        .expect("party pokemon info exists");
    assert!(
        pokemon_info
            .set_stat(case.idx, case.stat.clone(), case.new_value)
            .is_ok()
    );
    drop(pokemon_info);

    let property = cloned_gvas_file
        .properties
        .get("PartyPokemonInfo")
        .expect("party pokemon info exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let value = get_stat(pokemon_info, case.stat.clone()).expect("pokemon stat exists");

    assert!((value - case.new_value).abs() < f64::EPSILON);
}

fn assert_set_box_stat(gvas_file: &GvasFile, case: &SetStatCase) {
    let mut cloned_gvas_file = gvas_file.clone();
    let mut pokemon_info = PokemonInfoMut::new(&mut cloned_gvas_file, StorageType::BOXES(1))
        .expect("box pokemon info exists");
    assert!(
        pokemon_info
            .set_stat(case.idx, case.stat.clone(), case.new_value)
            .is_ok()
    );
    drop(pokemon_info);

    let property = cloned_gvas_file
        .properties
        .get("Box1PokemonInfo")
        .expect("box pokemon info exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let value = get_stat(pokemon_info, case.stat.clone()).expect("pokemon stat exists");

    assert!((value - case.new_value).abs() < f64::EPSILON);
}

fn assert_set_name(gvas_file: &GvasFile, case: &SetNameCase<'_>) {
    let mut cloned_gvas_file = gvas_file.clone();
    let mut pokemon_info = PokemonInfoMut::new(&mut cloned_gvas_file, StorageType::PARTY)
        .expect("party pokemon info exists");
    assert!(
        pokemon_info
            .set_name(case.idx, case.new_name.to_string())
            .is_ok()
    );
    drop(pokemon_info);

    let property = cloned_gvas_file
        .properties
        .get("PartyPokemonInfo")
        .expect("party pokemon info exists");
    let pokemon_info = get_struct_property_at_idx(property, case.idx).expect("pokemon info exists");
    let name = get_name(pokemon_info).expect("pokemon name exists");

    assert_eq!(name, case.new_name);
}

#[test]
fn reads_names_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        generate_case("Box1PokemonInfo", 1, "SALAMENCE"),
        generate_case("Box1PokemonInfo", 0, "NAME"),
        generate_case("PartyPokemonInfo", 0, "METAGROSS"),
    ];
    for case in cases {
        assert_name(&gvas_file, case);
    }
}

#[test]
fn reads_natures_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_nature_case(
        "Box1PokemonInfo",
        0,
        "ENUM_Natures::NewEnumerator0",
    )];
    for case in cases {
        assert_nature(&gvas_file, case);
    }
}

#[test]
fn reads_types_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_type_case(
        "Box1PokemonInfo",
        2,
        "ENUM_PokemonTypePrimary::NewEnumerator13",
        "ENUM_PokemonTypePrimary::NewEnumerator17",
    )];
    for case in cases {
        assert_types(&gvas_file, case);
    }
}

#[test]
fn reads_is_fainted_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_fainted_case("Box1PokemonInfo", 0, false)];
    for case in cases {
        assert_is_fainted(&gvas_file, case);
    }
}

#[test]
fn reads_levels_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_level_case("Box1PokemonInfo", 0, 10)];
    for case in cases {
        assert_level(&gvas_file, case);
    }
}

#[test]
fn reads_party_levels_from_wrapper() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_party_level_case(0, 48)];
    for case in cases {
        assert_party_level(&gvas_file, case);
    }
}

#[test]
fn reads_stats_by_name_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_stat_case(
        "Box1PokemonInfo",
        0,
        Stats::CurrentHp,
        11.0,
    )];
    for case in cases {
        assert_stat(&gvas_file, case);
    }
}

#[test]
fn mutates_stats_from_gvas_file() {
    let cases = &[generate_stat_mut_case(
        "Box1PokemonInfo",
        0,
        Stats::MaxHp,
        123.0,
    )];
    for case in cases {
        assert_stat_mut(case);
    }
}

#[test]
fn sets_party_stat_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_set_stat_case(0, Stats::MaxHp, 123.0)];
    for case in cases {
        assert_set_stat(&gvas_file, case);
    }
}

#[test]
fn sets_box_stat_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_set_stat_case(0, Stats::MaxHp, 123.0)];
    for case in cases {
        assert_set_box_stat(&gvas_file, case);
    }
}

#[test]
fn sets_party_name_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_set_name_case(0, "TEST_NAME")];
    for case in cases {
        assert_set_name(&gvas_file, case);
    }
}

#[test]
fn reads_all_stats_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[generate_stats_case("Box1PokemonInfo", 0, Stats::ATK, 6.0)];
    for case in cases {
        assert_stats(&gvas_file, case);
    }
}
