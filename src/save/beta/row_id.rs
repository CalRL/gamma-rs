use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;

pub struct RowID<'a> {
    property: &'a Property,
}

pub struct RowIDMut<'a> {
    property: &'a mut Property,
}

impl_storage_wrapper!(RowID, "RowID");
impl_storage_wrapper_mut!(RowIDMut, "RowID");

impl<'a> RowID<'a> {
    pub fn at_index(self, index: usize) -> Option<&'a i32> {
        let list = get_row_list(self.property)?;
        get_row_at(list, index)
    }

    pub fn as_list(&self) -> Option<&ArrayProperty> {
        get_row_list(self.property)
    }
}

impl<'a> RowIDMut<'a> {}

fn get_row_list(prop: &Property) -> Option<&ArrayProperty> {
    match prop {
        Property::ArrayProperty(a) => Some(a),
        _ => None,
    }
}

fn get_row_at(array: &ArrayProperty, index: usize) -> Option<&i32> {
    match array {
        ArrayProperty::Ints { ints } => ints.get(index),
        _ => None,
    }
}
