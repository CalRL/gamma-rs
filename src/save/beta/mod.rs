use crate::pokemon::types::Types;

pub struct BetaEnumStr<'a>(&'a str);

pub enum Error {
    InvalidNumber,
    UnknownType(i32)
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
            _ => Err(Error::UnknownType(num))
        }
    }
}

fn get_enum_number(enum_str: &str) -> Option<i32> {
    enum_str.to_string().split("::")
        .last()
        .and_then(|part| part.strip_prefix("NewEnumerator"))
        .and_then(|x| x.parse::<i32>().ok())
}

pub fn from_enum_str(enum_str: &str) -> Option<&str> {
    let num = get_enum_number(enum_str)?;

    let val = match num {
        0 => "BUG",
        2 => "FLYING",
        4 => "GROUND",
        5 => "NORMAL",
        6 => "POISON",
        7 => "ROCK",
        8 => "STEEL",
        9 => "DARK",
        10 => "STEEL",
        12 => "FIRE",
        13 => "GRASS",
        15 => "PSYCHIC",
        16 => "WATER",
        17 => "NONE",
        18 => "FAIRY",
        _ => return None,
    };

    Some(val)
}


