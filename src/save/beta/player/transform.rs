use gvas::GvasFile;
use gvas::properties::struct_types::VectorD;

pub fn get_transform(gvas_file: &GvasFile) -> Option<&VectorD> {
    gvas_file
        .properties
        .get("PlayerTransform")?
        .get_struct()?
        .value
        .get_custom_struct()?
        .0
        .get("Translation")?
        .first()?
        .get_struct()?
        .value
        .get_vector_d()
}

pub fn get_transform_mut(gvas_file: &mut GvasFile) -> Option<&mut VectorD> {
    gvas_file
        .properties
        .get_mut("PlayerTransform")?
        .get_struct_mut()?
        .value
        .get_custom_struct_mut()?
        .0
        .get_mut("Translation")?
        .first_mut()?
        .get_struct_mut()?
        .value
        .get_vector_d_mut()
}
