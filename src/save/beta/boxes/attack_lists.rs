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
