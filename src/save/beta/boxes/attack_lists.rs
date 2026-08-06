use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::struct_property::StructPropertyValue;

// Takes the "PartyAttackLists" property
pub fn attack_array(property: &Property) -> Option<&ArrayProperty> {
    let cs: &ArrayProperty = match &property {
        Property::ArrayProperty(arr) => Some(arr),
        _ => return None,
    }?;

    Some(cs)
}
/// Returns the custom struct
// TODO: test this
pub fn attacks_at(array: &ArrayProperty, index: usize) -> Option<&ArrayProperty> {
    let property: &StructPropertyValue = match &array {
        ArrayProperty::Structs { structs, .. } => structs.get(index)?,
        _ => return None,
    };

    let custom_struct = property.get_custom_struct()?;
    for (key, val) in custom_struct.iter() {
        if key.starts_with("Attacks_") {
            let first = val.first()?;
            return match &first {
                Property::ArrayProperty(arr) => Some(&arr),
                _ => None,
            };
        }
    }
    None
}

/// Takes attacks_at custom struct. Returns the attack string
pub fn attack_at(array: &ArrayProperty, index: usize) -> Option<&String> {
    let property = match &array {
        ArrayProperty::Properties { properties, .. } => properties.get(index),
        _ => None,
    }?;

    let object = match &property {
        Property::ObjectProperty(obj) => obj,
        _ => return None,
    };

    Some(&object.value)
}

/// Returns the attack name, from the attack class path.
pub fn parse_attack(attack: &str) -> Option<String> {
    let string = String::from(attack);
    let class = string.split(".").last()?.to_string();
    let name = class.replace("BP_", "").replace("_C", "").to_string();
    Some(name)
}

pub struct AttackLists;

#[cfg(test)]
mod tests {
    use super::*;
    use gvas::GvasFile;
    use gvas::game_version::GameVersion;
    use std::fs::File;

    const SLOT1_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/saves/Slot1.sav");

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

    fn load_slot1() -> GvasFile {
        let mut file = File::open(SLOT1_PATH).expect("save file exists");
        GvasFile::read(&mut file, GameVersion::Default).expect("gvas file reads")
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
        let gvas_file = load_slot1();

        let cases = &[
            generate_case("Box1AttackLists", 1, 0, "Growl"),
            generate_case("PartyAttackLists", 0, 0, "Astonish"),
        ];

        assert!(!cases.is_empty(), "add attack list test cases");
        for case in cases {
            assert_attack_value(&gvas_file, case);
        }
    }
}
