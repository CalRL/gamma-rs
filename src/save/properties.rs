use gvas::properties::Property;
use gvas::types::map::HashableIndexMap;

pub struct Properties<'a>(&'a HashableIndexMap<String, Property>);

pub struct PropertiesMut<'a>(&'a mut HashableIndexMap<String, Property>);
