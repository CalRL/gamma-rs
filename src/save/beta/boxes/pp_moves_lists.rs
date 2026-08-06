use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::types::map::HashableIndexMap;
use indexmap::IndexMap;

pub fn moves_array(property: &Property) -> Option<&ArrayProperty> {
    property.get_array()
}

/// Returns moves index at
/// Takes moves_array result
pub fn moves_at(array: &ArrayProperty, index: usize) -> Option<&ArrayProperty> {
    let values: &Vec<StructPropertyValue> = match array {
        ArrayProperty::Structs { structs, .. } => structs,
        _ => return None,
    };

    let properties: &HashableIndexMap<String, Vec<Property>> = match &values.get(index)? {
        StructPropertyValue::CustomStruct { 0: map, .. } => map,
        _ => return None,
    };

    properties.values().find_map(|v| match v.first()? {
        Property::ArrayProperty(arr) => Some(arr),
        _ => None,
    })
}

/// Gets max pp at an index
/// Takes moves_at result
pub fn max_pp_at(moves: &ArrayProperty, index: usize) -> Option<&i32> {
    let pp_struct: &StructPropertyValue = match &moves {
        ArrayProperty::Structs { structs, .. } => structs.get(index)?,
        _ => return None,
    };
    let map: &IndexMap<String, Vec<Property>> = &pp_struct.get_custom_struct()?.0;
    let prop: &Property = map
        .iter()
        .find(|(k, _)| k.starts_with("MaxPP"))
        .and_then(|(_, v)| v.first())?;
    Some(&prop.get_int()?.value)
}

pub fn current_pp_at(moves: &ArrayProperty, index: usize) -> Option<&i32> {
    let pp_struct: &StructPropertyValue = match &moves {
        ArrayProperty::Structs { structs, .. } => structs.get(index)?,
        _ => return None,
    };
    let map: &IndexMap<String, Vec<Property>> = &pp_struct.get_custom_struct()?.0;
    // Typo "Curremt" is intentional, it's like that in the file...
    let prop: &Property = map
        .iter()
        .find(|(k, _)| k.starts_with("CurremtPP"))
        .and_then(|(_, v)| v.first())?;
    Some(&prop.get_int()?.value)
}
pub struct PPMovesLists;

#[cfg(test)]
mod tests {
    use super::*;
    use gvas::GvasFile;
    use gvas::game_version::GameVersion;
    use std::fs::File;

    const SLOT1_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/saves/Slot1.sav");

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

    fn load_slot1() -> GvasFile {
        let mut file = File::open(SLOT1_PATH).expect("save file exists");
        GvasFile::read(&mut file, GameVersion::Default).expect("gvas file reads")
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
        let gvas_file = load_slot1();

        let cases = &[
            generate_case("Box1PPMovesLists", 1, 0, 15, 15),
            generate_case("PartyPPMovesLists", 0, 0, 10, 15),
        ];

        assert!(!cases.is_empty(), "add PP move list test cases");
        for case in cases {
            assert_pp_values(&gvas_file, case);
        }
    }
}
