use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;

pub struct SlotID<'a> {
    property: &'a Property,
}

pub struct SlotIDMut<'a> {
    property: &'a mut Property,
}
impl_storage_wrapper!(SlotID, "SlotID");
impl_storage_wrapper_mut!(SlotIDMut, "SlotID");
impl<'a> SlotID<'a> {
    pub fn at_index(self, index: usize) -> Option<&'a i32> {
        let list = get_slot_list(self.property)?;
        get_slot_at(list, index)
    }

    pub fn as_list(&self) -> Option<&ArrayProperty> {
        get_slot_list(self.property)
    }
}

impl<'a> SlotIDMut<'a> {}

fn get_slot_list(prop: &Property) -> Option<&ArrayProperty> {
    match prop {
        Property::ArrayProperty(a) => Some(a),
        _ => None,
    }
}

fn get_slot_at(array: &ArrayProperty, index: usize) -> Option<&i32> {
    match array {
        ArrayProperty::Ints { ints } => ints.get(index),
        _ => None,
    }
}
