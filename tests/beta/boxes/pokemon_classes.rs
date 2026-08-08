use crate::common;

use gamma_rs::save::beta::StorageType;
use gamma_rs::save::beta::pokemon_classes::{PokemonClasses, class_at, class_at_mut, parse_class};
use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;

struct ClassCase<'a> {
    property_name: &'a str,
    storage_type: StorageType,
    idx: usize,
}

fn generate_case(property_name: &str, storage_type: StorageType, idx: usize) -> ClassCase<'_> {
    ClassCase {
        property_name,
        storage_type,
        idx,
    }
}

fn class_array<'a>(gvas_file: &'a GvasFile, property_name: &str) -> &'a ArrayProperty {
    gvas_file
        .properties
        .get(property_name)
        .expect("pokemon classes property exists")
        .get_array()
        .expect("pokemon classes property is an array")
}

fn class_array_mut<'a>(gvas_file: &'a mut GvasFile, property_name: &str) -> &'a mut ArrayProperty {
    gvas_file
        .properties
        .get_mut(property_name)
        .expect("pokemon classes property exists")
        .get_array_mut()
        .expect("pokemon classes property is an array")
}

fn raw_class_at<'a>(gvas_file: &'a GvasFile, property_name: &str, idx: usize) -> &'a String {
    let array = class_array(gvas_file, property_name);
    let property = match array {
        ArrayProperty::Properties { properties, .. } => properties
            .get(idx)
            .expect("pokemon class property exists at index"),
        _ => panic!("pokemon classes property has unexpected array type"),
    };

    match property {
        Property::ObjectProperty(object) => &object.value,
        _ => panic!("pokemon class entry has unexpected property type"),
    }
}

fn assert_class_value(gvas_file: &GvasFile, case: &ClassCase<'_>) {
    let pokemon_classes = PokemonClasses::new(gvas_file, case.storage_type.clone())
        .expect("pokemon classes wrapper exists");
    let expected = raw_class_at(gvas_file, case.property_name, case.idx);
    let actual = pokemon_classes
        .class_at(case.idx)
        .expect("pokemon class exists at index");

    assert_eq!(actual, expected);
}

fn assert_raw_class_value(gvas_file: &GvasFile, case: &ClassCase<'_>) {
    let array = class_array(gvas_file, case.property_name);
    let expected = raw_class_at(gvas_file, case.property_name, case.idx);

    assert_eq!(class_at(array, case.idx), Some(expected));
}

fn assert_mutates_class_value(gvas_file: &GvasFile, case: &ClassCase<'_>, new_class: &str) {
    let mut cloned_gvas_file = gvas_file.clone();
    let array = class_array_mut(&mut cloned_gvas_file, case.property_name);
    let class = class_at_mut(array, case.idx).expect("pokemon class exists at index");

    *class = new_class.to_string();

    assert_eq!(class.as_str(), new_class);
}

#[test]
fn reads_pokemon_classes_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        generate_case("PartyPokemonClasses", StorageType::PARTY, 0),
        generate_case("Box1PokemonClasses", StorageType::BOXES(1), 0),
    ];

    for case in cases {
        assert_class_value(&gvas_file, case);
    }
}

#[test]
fn reads_raw_pokemon_classes_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        generate_case("PartyPokemonClasses", StorageType::PARTY, 0),
        generate_case("Box1PokemonClasses", StorageType::BOXES(1), 0),
    ];

    for case in cases {
        assert_raw_class_value(&gvas_file, case);
    }
}

#[test]
fn mutates_pokemon_classes_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        generate_case("PartyPokemonClasses", StorageType::PARTY, 0),
        generate_case("Box1PokemonClasses", StorageType::BOXES(1), 0),
    ];

    for case in cases {
        assert_mutates_class_value(
            &gvas_file,
            case,
            "/Game/Blueprints/Pokemon/BP_Test.BP_Test_C",
        );
    }
}
