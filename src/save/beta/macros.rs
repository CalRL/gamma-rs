macro_rules! impl_storage_wrapper {
    ($type:ident, $key:literal) => {
        impl<'a> $type<'a> {
            pub fn new(
                gvas_file: &'a gvas::GvasFile,
                storage_type: crate::save::beta::StorageType,
            ) -> Option<Self> {
                let key = crate::utils::property_key(storage_type, $key);
                Some(Self {
                    property: gvas_file.properties.get(key.as_str())?,
                })
            }
        }
    };
}

macro_rules! impl_storage_wrapper_mut {
    ($type:ident, $key:literal) => {
        impl<'a> $type<'a> {
            pub fn new(
                gvas_file: &'a mut gvas::GvasFile,
                storage_type: crate::save::beta::StorageType,
            ) -> Option<Self> {
                let key = crate::utils::property_key(storage_type, $key);
                Some(Self {
                    property: gvas_file.properties.get_mut(key.as_str())?,
                })
            }
        }
    };
}
