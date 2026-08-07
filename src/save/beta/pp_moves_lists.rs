use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::struct_property::StructPropertyValue;
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
