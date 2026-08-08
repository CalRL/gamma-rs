use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;

pub fn class_at(array: &ArrayProperty, idx: usize) -> Option<&String> {
    let class_property = match &array {
        ArrayProperty::Properties { properties, .. } => properties.get(idx)?,
        _ => return None,
    };

    match &class_property {
        Property::ObjectProperty(prop) => Some(&prop.value),
        _ => None,
    }
}

/// Probably shouldn't be used, at least not until an enum for every class is written...
pub fn class_at_mut(array: &mut ArrayProperty, idx: usize) -> Option<&mut String> {
    let class_property: &mut Property = match array {
        ArrayProperty::Properties { properties, .. } => properties.get_mut(idx)?,
        _ => return None,
    };

    match class_property {
        Property::ObjectProperty(prop) => Some(&mut prop.value),
        _ => None,
    }
}

/// Returns the name, from the class path.
pub fn parse_class(class: &str) -> Option<String> {
    let string: String = String::from(class);
    let class: String = string.split(".").last()?.to_string();
    let name: String = class
        .replace("BP_", "")
        .replace("_C", "")
        .replace("Player_", "")
        .to_string();

    Some(name)
}

pub struct PokemonClasses<'a> {
    property: &'a Property,
}

pub struct PokemonClassesMut<'a> {
    property: &'a mut Property,
}

impl_storage_wrapper!(PokemonClasses, "PokemonClasses");
impl_storage_wrapper_mut!(PokemonClassesMut, "PokemonClasses");

impl<'a> PokemonClasses<'a> {
    pub fn class_at(&self, idx: usize) -> Option<&String> {
        let arr = self.property.get_array()?;
        

        class_at(arr, idx)
    }

    pub fn classes(&self) -> Option<Vec<&String>> {
        let array: &ArrayProperty = self.property.get_array()?;
        let mut strings: Vec<&String> = Vec::new();
        if let ArrayProperty::Properties { properties, .. } = array {
            for i in properties.iter() {
                if let Property::ObjectProperty(prop) = i {
                    let val = &prop.value;
                    strings.push(val);
                }
            }
        };

        Some(strings)
    }
}
