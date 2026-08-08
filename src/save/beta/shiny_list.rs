use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;

pub fn get_shiny_list(array: &ArrayProperty) -> Option<&Vec<bool>> {
    match array {
        ArrayProperty::Bools { bools } => Some(bools),
        _ => None,
    }
}

pub fn get_shiny_at(array: &ArrayProperty, index: usize) -> Option<&bool> {
    match array {
        ArrayProperty::Bools { bools } => bools.get(index),
        _ => None,
    }
}

pub fn get_shiny_at_mut(array: &mut ArrayProperty, index: usize) -> Option<&mut bool> {
    match array {
        ArrayProperty::Bools { bools } => bools.get_mut(index),
        _ => None,
    }
}

pub fn set_shiny_at(array: &mut ArrayProperty, index: usize, value: bool) -> bool {
    match get_shiny_at_mut(array, index) {
        None => false,
        Some(shiny) => {
            *shiny = value;
            true
        }
    }
}

pub struct ShinyList<'a> {
    pub property: &'a Property,
}

pub struct ShinyListMut<'a> {
    property: &'a mut Property,
}

impl_storage_wrapper!(ShinyList, "ShinyList");
impl_storage_wrapper_mut!(ShinyListMut, "ShinyList");

impl<'a> ShinyList<'a> {
    fn get_array(&self) -> Option<&ArrayProperty> {
        self.property.get_array()
    }

    pub fn get_shiny_list(&self) -> Option<&Vec<bool>> {
        get_shiny_list(self.get_array()?)
    }

    pub fn get_shiny_at(&self, index: usize) -> Option<&bool> {
        get_shiny_at(self.get_array()?, index)
    }
}

impl<'a> ShinyListMut<'a> {
    pub fn set_shiny_at(&mut self, index: usize, value: bool) -> Result<(), String> {
        match self.property {
            Property::ArrayProperty(array) => {
                if let Some(shiny) = get_shiny_at_mut(array, index) {
                    *shiny = value;
                    Ok(())
                } else {
                    Err(format!(
                        "Failed to set shiny at index {} (does it exist?)",
                        index
                    ))
                }
            }
            _ => Err(format!("Failed to set shiny at index {}", index)),
        }
    }
}
