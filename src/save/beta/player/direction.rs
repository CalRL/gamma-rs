use gvas::GvasFile;
use gvas::properties::struct_property::StructPropertyValue;
use gvas::properties::struct_types::Vector2D;

pub fn get_direction(gvas_file: &GvasFile) -> Option<&Vector2D> {
    let val = &gvas_file
        .properties
        .get("PlayerDirection")?
        .get_struct()?
        .value;

    match val {
        StructPropertyValue::Vector2D(vec) => Some(vec),
        _ => None,
    }
}

pub fn get_direction_mut(gvas_file: &mut GvasFile) -> Option<&mut Vector2D> {
    let val = &mut gvas_file
        .properties
        .get_mut("PlayerDirection")?
        .get_struct_mut()?
        .value;

    match val {
        StructPropertyValue::Vector2D(vec) => Some(vec),
        _ => None,
    }
}
