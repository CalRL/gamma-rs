use crate::save::beta::StorageType;
use crate::save::beta::pokemon::stats::{StatStruct, Stats};
use crate::utils::custom_struct::get_struct_property_at_idx;
use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::struct_property::StructPropertyValue;
use gvas::properties::text_property::FTextHistory;

pub struct PokemonInfo<'a> {
    /// The actual property containing isFainted, IVs, name, etc.
    property: &'a Property,
}

pub struct PokemonInfoMut<'a> {
    property: &'a mut Property,
}

impl_storage_wrapper!(PokemonInfo, "PokemonInfo");
impl_storage_wrapper_mut!(PokemonInfoMut, "PokemonInfo");


pub fn get_is_fainted(property: &StructPropertyValue) -> Option<bool> {
    Some(get_first(property, "isFainted")?.get_bool()?.value)
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
pub struct InfoSnapshot {
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
// TODO: make a infosnapshot::new func with storagetype and index!

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

pub fn get_stats(property: &StructPropertyValue) -> Option<StatStruct> {
    let mut map = std::collections::HashMap::new();
    for stat in Stats::iter() {
        map.insert(stat.clone(), get_stat(property, stat)?);
    }

    Some(StatStruct { values: map })
}

pub fn get_level(property: &StructPropertyValue) -> Option<&i32> {
    let property = get_first(property, "Level")?;
    match property {
        Property::IntProperty(val) => Some(&val.value),
        _ => None,
    }
}

pub fn get_name(property: &StructPropertyValue) -> Option<&String> {
    let prop = get_first(property, "Name")?;
    let text = match prop {
        Property::TextProperty(text) => Some(text),
        _ => None,
    }?;

    match &text.value.history {
        FTextHistory::Base {
            source_string: name,
            ..
        } => Some(name),
        _ => None,
    }?
    .as_ref()
}

pub fn get_name_mut(property: &mut StructPropertyValue) -> Option<&mut String> {
    let prop = get_first_mut(property, "Name")?;
    let text = match prop {
        Property::TextProperty(text) => Some(text),
        _ => None,
    }?;

    match &mut text.value.history {
        FTextHistory::Base {
            source_string: name,
            ..
        } => Some(name),
        _ => None,
    }?
    .as_mut()
}

fn get_namespaced<'a>(property: &'a StructPropertyValue, key_prefix: &str) -> Option<&'a String> {
    let prop = get_first(property, key_prefix)?;
    let val = match prop {
        Property::ByteProperty(prop) => Some(&prop.value),
        _ => None,
    }?;

    match val {
        BytePropertyValue::Byte(_) => None,
        BytePropertyValue::Namespaced(val) => Some(val),
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
fn get_first<'a>(property: &'a StructPropertyValue, key_prefix: &str) -> Option<&'a Property> {
    property
        .get_custom_struct()?
        .0
        .iter()
        .find(|(key, _)| key.starts_with(key_prefix))?
        .1
        .first()
}

fn get_first_mut<'a>(
    property: &'a mut StructPropertyValue,
    key_prefix: &str,
) -> Option<&'a mut Property> {
    property
        .get_custom_struct_mut()?
        .0
        .iter_mut()
        .find(|(key, _)| key.starts_with(key_prefix))?
        .1
        .first_mut()
}

impl<'a> PokemonInfo<'a> {
    pub fn get_is_fainted(&self, index: usize) -> Option<bool> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_is_fainted(struct_at)
    }

    pub fn get_level(&self, index: usize) -> Option<&i32> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_level(struct_at)
    }

    pub fn get_name(&self, index: usize) -> Option<&String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;

        get_name(struct_at)
    }

    pub fn get_nature(&self, index: usize) -> Option<String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_nature(struct_at).cloned()
    }

    pub fn get_primary_type(&self, index: usize) -> Option<String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_primary_type(struct_at).cloned()
    }

    pub fn get_secondary_type(&self, index: usize) -> Option<String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_secondary_type(struct_at).cloned()
    }
    pub fn get_stats(&self, index: usize) -> Option<StatStruct> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_stats(struct_at)
    }
    pub fn get_stat(&self, index: usize, stat: Stats) -> Option<f64> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_stat(struct_at, stat)
    }
}

impl<'a> PokemonInfoMut<'a> {
    pub fn set_stat(
        &mut self,
        index: usize,
        stat: Stats,
        value: f64,
    ) -> Result<(), set_stat::Error> {
        let arr = self
            .property
            .get_array_mut()
            .ok_or(set_stat::Error::InvalidArrayProperty)?;
        let structs = match arr {
            ArrayProperty::Structs { structs, .. } => Ok(structs
                .get_mut(index)
                .ok_or(set_stat::Error::InvalidIndex)?),
            _ => Err(set_stat::Error::InvalidStructArray),
        }?;

        let stat = get_stat_mut(structs, stat).ok_or(set_stat::Error::StatNotFound)?;
        *stat = value;
        Ok(())
    }

    pub fn set_name(&mut self, index: usize, name: String) -> Result<(), set_name::Error> {
        let arr = self
            .property
            .get_array_mut()
            .ok_or(set_name::Error::InvalidArrayProperty)?;
        let structs = match arr {
            ArrayProperty::Structs { structs, .. } => Ok(structs
                .get_mut(index)
                .ok_or(set_name::Error::InvalidIndex)?),
            _ => Err(set_name::Error::InvalidStructArray),
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