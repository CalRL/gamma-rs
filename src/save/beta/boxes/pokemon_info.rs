use crate::save::beta::pokemon::stats::{StatStruct, Stats};
use crate::traits::StartsWith;
use crate::utils::custom_struct::{get_struct_property_at_idx};
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::text_property::FTextHistory;
use std::collections::HashMap;
use crate::save::beta::StorageType;

pub fn get_is_fainted(struct_property: &StructProperty) -> Option<bool> {
    let is_fainted: &Vec<Property> = struct_property.get_starts_with("isFainted")?;
    let first: &Property = is_fainted.first()?;
    match first {
        Property::BoolProperty(bool) => Some(bool.value),
        _ => None,
    }
}

// contains:
// is_fainted - bool
// name - string
// character_icon - object
// level - int
// current_hp - double
// max_hp - double
// atk - double
// def - double
// satk - double
// sdef - double
// speed - double
// PrimaryType - byte (enum)
// SecondaryType - byte (enum)
// nature - byte (enum)

#[derive(Default, Clone, Debug)]
pub struct InfoStruct {
    pub is_fainted: Option<bool>,
    pub name: Option<String>,
    pub character_icon: Option<String>,
    pub level: Option<i32>,
    pub current_hp: Option<f64>,
    pub max_hp: Option<f64>,
    pub atk: Option<f64>,
    pub def: Option<f64>,
    pub satk: Option<f64>,
    pub speed: Option<f64>,
    // These 3 are actually byte property values.
    // We won't store a ByteProperty, but rather the value as a string and convert later.
    pub primary_type: Option<String>,
    pub secondary_type: Option<String>,
    pub nature: Option<String>,
}

/// Takes the custom struct indexmap.
// properties: The properties inside the custom struct
// e.g.
// "CustomStruct": {
// "type_name": "STRUCT_CharacterAttributes",
// "properties": { <- **THIS**
// must not be casted to structproperty, get_starts_with handles that...
pub fn get_stat(property: &StructPropertyValue, stat: Stats) -> Option<f64> {
    let stat_str: &str = stat.as_str();
    let stat_property = get_first(property, stat_str)?;
    match &stat_property {
        Property::DoubleProperty(double) => Some(double.value.0),
        _ => None,
    }
}

pub fn get_stat_mut(property: &mut StructPropertyValue, stat: Stats) -> Option<&mut f64> {
    let stat_str: &str = stat.as_str();
    let stat_property = get_first_mut(property, stat_str)?;

    match stat_property {
        Property::DoubleProperty(double) => Some(&mut double.value.0),
        _ => None,
    }
}

// todo()! fix this
pub fn get_stats(properties: &StructProperty) -> Option<StatStruct> {
    fn get_value(props: &StructProperty, stat: Stats) -> Option<f64> {
        let name = stat.as_str();
        let property = props.get_starts_with(name)?.first()?;
        match property {
            Property::DoubleProperty(double) => Some(double.value.0),
            _ => None,
        }
    }

    let mut map: HashMap<Stats, f64> = HashMap::new();
    for stat in Stats::iter() {
        map.insert(stat.clone(), get_value(properties, stat)?);
    }

    Some(StatStruct { values: map })
}

pub fn get_level(property: &StructPropertyValue) -> Option<&i32> {
    let property = get_first(property, "Level")?;
    match property {
        Property::IntProperty(val) => Some(&val.value),
        _ => None
    }
}

pub fn get_name(property: &StructPropertyValue) -> Option<&String> {
    let prop = get_first(property, "Name")?;
    let text = match prop {
        Property::TextProperty(text) => Some(text),
        _ => None
    }?;

    match &text.value.history {
        FTextHistory::Base { source_string: name, .. } => Some(name),
        _ => None
    }?.as_ref()
}

pub fn get_name_mut(property: &mut StructPropertyValue) -> Option<&mut String> {
    let prop = get_first_mut(property, "Name")?;
    let text = match prop {
        Property::TextProperty(text) => Some(text),
        _ => None
    }?;

    match &mut text.value.history {
        FTextHistory::Base { source_string: name, .. } => Some(name),
        _ => None
    }?.as_mut()
}

fn get_namespaced<'a>(property: &'a StructPropertyValue, key_prefix: &str) -> Option<&'a String>{
    let prop = get_first(property, key_prefix)?;
    let val = match prop {
        Property::ByteProperty(prop) => Some(&prop.value),
        _ => None
    }?;

    match val {
        BytePropertyValue::Byte(_) => None,
        BytePropertyValue::Namespaced(val) => Some(val)
    }
}

/// Returns a namespaced string
pub fn get_primary_type(property: &StructPropertyValue) -> Option<&String> {
    get_namespaced(property, "PrimaryType")
}

pub fn get_secondary_type(property: &StructPropertyValue) -> Option<&String> {
    get_namespaced(property, "SecondaryType")
}

pub fn get_nature(property: &StructPropertyValue) -> Option<&String> {
    get_namespaced(property, "Nature")
}

pub struct PokemonInfo<'a> {
    /// The actual property containing isFainted, IVs, name, etc.
    property: &'a Property,
}

fn get_first<'a>(property: &'a StructPropertyValue, key_prefix: &str) -> Option<&'a Property> {
    property
        .get_custom_struct()?
        .0
        .iter()
        .find(|(key, _)| key.starts_with(key_prefix))?
        .1
        .first()
}

fn get_first_mut<'a>(property: &'a mut StructPropertyValue, key_prefix: &str) -> Option<&'a mut Property> {
    property
        .get_custom_struct_mut()?
        .0
        .iter_mut()
        .find(|(key, _)| key.starts_with(key_prefix))?
        .1
        .first_mut()
}

impl<'a> PokemonInfo<'a> {
    /// Todo: turn this into a trait
    pub fn new_party(gvas_file: &'a GvasFile) -> Option<Self> {
        let prop = gvas_file.properties.get("PartyPokemonInfo")?;
        Some(Self { property: prop })
    }

    pub fn get_name(&self, index: usize) -> Option<&String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;

        get_name(struct_at)
    }

    // pub fn get_nature(&self, index: usize) -> Option<String> {
    //     let struct_at = get_struct_property_at_idx(self.property, index)?;
    //     get_nature_string(struct_at).cloned()
    // }
    //
    // pub fn get_primary_type(&self, index: usize) -> Option<String> {
    //     let struct_at = get_struct_property_at_idx(self.property, index)?;
    //     get_primary_type_string(struct_at).cloned()
    // }
    //
    // pub fn get_secondary_type(&self, index: usize) -> Option<String> {
    //     let struct_at = get_struct_property_at_idx(self.property, index)?;
    //     get_secondary_type_string(struct_at).cloned()
    // }
    // pub fn get_stats(&self, index: usize) -> Option<StatStruct> {
    //     let struct_at = get_struct_property_at_idx(self.property, index)?;
    //     get_stats(struct_at)
    // }
    // pub fn get_stat(&self, index: usize, stat: Stats) -> Option<f64> {
    //     let struct_at = get_struct_property_at_idx(self.property, index)?;
    //     get_stat(struct_at, stat)
    // }
}

pub struct PokemonInfoMut<'a> {
    property: &'a mut Property,
    storage_type: StorageType
}

impl<'a> PokemonInfoMut<'a> {
    pub fn new_party(gvas_file: &'a mut GvasFile) -> Option<Self> {
        let prop = gvas_file.properties.get_mut("PartyPokemonInfo")?;
        Some(Self { property: prop, storage_type: StorageType::PARTY })
    }

    pub fn set_stat(&mut self, index: usize, stat: Stats, value: f64) -> Result<(), set_stat::Error> {
        let arr = self.property.get_array_mut().ok_or(set_stat::Error::InvalidArrayProperty)?;
        let structs = match arr {
            ArrayProperty::Structs { structs, .. } => Ok(structs.get_mut(index).ok_or(set_stat::Error::InvalidIndex)?),
            _ => Err(set_stat::Error::InvalidStructArray)
        }?;

        let stat = get_stat_mut(structs, stat).ok_or(set_stat::Error::StatNotFound)?;
        *stat = value;
        Ok(())
    }

    pub fn set_name(&mut self, index: usize, name: String) -> Result<(), set_name::Error>{
        let arr = self.property.get_array_mut().ok_or(set_name::Error::InvalidArrayProperty)?;
        let structs = match arr {
            ArrayProperty::Structs { structs, .. } => Ok(structs.get_mut(index).ok_or(set_name::Error::InvalidIndex)?),
            _ => Err(set_name::Error::InvalidStructArray)
        }?;

        let stat = get_name_mut(structs).ok_or(set_name::Error::NameNotFound)?;
        *stat = name;
        Ok(())
    }
}

pub mod set_stat {
    pub enum Error {
        InvalidArrayProperty,
        InvalidStructArray,
        InvalidIndex,
        StatNotFound,
    }
}

pub mod set_name {
    pub enum Error {
        InvalidArrayProperty,
        InvalidStructArray,
        InvalidIndex,
        NameNotFound,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gvas::game_version::GameVersion;
    use std::fs::File;
    use crate::save::beta::BetaEnumStr;
    use crate::save::beta::pokemon::types::Types;

    const SLOT1_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/saves/Slot1.sav");

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

    fn load_slot1() -> GvasFile {
        let mut file = File::open(SLOT1_PATH).expect("save file exists");
        GvasFile::read(&mut file, GameVersion::Default).expect("gvas file reads")
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

        assert_eq!(
            primary_type, case.expected_primary_type,
            "primary type mismatch for {}[{}]",
            case.array_name, case.idx
        );
        assert_eq!(
            secondary_type, case.expected_secondary_type,
            "secondary type mismatch for {}[{}]",
            case.array_name, case.idx
        );
    }

    #[test]
    fn reads_names_from_gvas_file() {
        let gvas_file = load_slot1();

        let cases = &[
            generate_case("Box1PokemonInfo", 1, "SALAMENCE"),
            generate_case("Box1PokemonInfo", 0, "NAME"),
            generate_case("PartyPokemonInfo", 0, "METAGROSS"),
        ];

        assert!(!cases.is_empty(), "add pokemon info name test cases");
        for case in cases {
            assert_name(&gvas_file, case);
        }
    }

    #[test]
    fn reads_natures_from_gvas_file() {
        let gvas_file = load_slot1();

        let cases = &[generate_nature_case(
            "Box1PokemonInfo",
            0,
            "ENUM_Natures::NewEnumerator0",
        )];

        assert!(!cases.is_empty(), "add pokemon info nature test cases");
        for case in cases {
            assert_nature(&gvas_file, case);
        }
    }

    #[test]
    fn reads_types_from_gvas_file() {
        let gvas_file = load_slot1();

        let cases = &[generate_type_case(
            "Box1PokemonInfo",
            2,
            "ENUM_PokemonTypePrimary::NewEnumerator13",
            "ENUM_PokemonTypePrimary::NewEnumerator17",
        )];

        assert!(!cases.is_empty(), "add pokemon info type test cases");
        for case in cases {
            assert_types(&gvas_file, case);
        }
    }
}
