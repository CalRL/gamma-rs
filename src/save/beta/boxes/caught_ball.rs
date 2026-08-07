use crate::save::beta::{BetaEnumStr, StorageType};
use crate::save::beta::pokemon::ball::PokeBall;
use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;

pub struct CaughtBall<'a> {
    property: &'a Property,
}
impl<'a> CaughtBall<'a> {
    pub fn new(gvas_file: &'a GvasFile, storage_type: StorageType) -> Option<Self> {
        let key = crate::utils::property_key(storage_type, "CaughtBall");
        Some(Self {
            property: gvas_file.properties.get(key.as_str())?,
        })
    }

    pub fn get_caught_ball_at(&self, index: usize) -> Option<PokeBall> {
        let arr: &ArrayProperty = self.property.get_array()?;
        let ball_enum: String = get_caught_ball_at(arr, index)?;

        PokeBall::try_from(BetaEnumStr::try_from(ball_enum.as_str()).ok()?).ok()
    }
}

pub struct CaughtBallMut<'a> {
    property: &'a mut Property,
}
impl<'a> CaughtBallMut<'a> {
    pub fn new(gvas_file: &'a mut GvasFile, storage_type: StorageType) -> Option<Self> {
        let key = crate::utils::property_key(storage_type, "CaughtBall");
        Some(CaughtBallMut {
            property: gvas_file.properties.get_mut(key.as_str())?,
        })
    }

    pub fn set_ball_at(&mut self, poke_ball: PokeBall, index: usize) -> Result<(), String> {
        if let Some(arr) = self.property.get_array_mut() {
            if let Some(ball_enum) = get_caught_ball_at_mut(arr, index) {
                *ball_enum = poke_ball.as_enum().to_string()
            }
        }

        Ok(())
    }
}

fn get_caught_ball_at(array: &ArrayProperty, index: usize) -> Option<String> {
    let property = match array {
        ArrayProperty::Properties { properties, .. } => properties.get(index)?,
        _ => return None,
    };

    match property {
        Property::ObjectProperty(object) => Some(object.value.clone()),
        _ => None,
    }
}

fn get_caught_ball_at_mut(array: &mut ArrayProperty, index: usize) -> Option<&mut String> {
    let property = match array {
        ArrayProperty::Properties { properties, .. } => properties.get_mut(index)?,
        _ => return None,
    };

    match property {
        Property::ObjectProperty(object) => Some(&mut object.value),
        _ => None,
    }
}
