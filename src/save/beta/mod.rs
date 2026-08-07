pub mod boxes;
pub mod pokemon;

use crate::utils::get_enum_number;
use pokemon::types::Types;
use std::str::FromStr;

pub struct BetaEnumStr<'a>(&'a str);

impl<'a> TryFrom<&'a str> for BetaEnumStr<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value.contains("::NewEnumerator") {
            true => Ok(BetaEnumStr(value)),
            false => Err(()),
        }
    }
}

pub enum Error {
    InvalidNumber,
    UnknownType(i32),
}
impl TryFrom<BetaEnumStr<'_>> for Types {
    type Error = Error;

    fn try_from(value: BetaEnumStr<'_>) -> Result<Self, Self::Error> {
        let num = get_enum_number(value.0).ok_or(Error::InvalidNumber)?;
        match num {
            0 => Ok(Types::Bug),
            2 => Ok(Types::Flying),
            4 => Ok(Types::Ground),
            5 => Ok(Types::Normal),
            6 => Ok(Types::Poison),
            7 => Ok(Types::Rock),
            8 => Ok(Types::Steel),
            9 => Ok(Types::Dark),
            10 => Ok(Types::Steel),
            12 => Ok(Types::Fire),
            13 => Ok(Types::Grass),
            15 => Ok(Types::Psychic),
            16 => Ok(Types::Water),
            17 => Ok(Types::Unknown),
            18 => Ok(Types::Fairy),
            _ => Err(Error::UnknownType(num)),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum StorageType {
    PARTY,
    BOXES(i32),
}
