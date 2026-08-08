use crate::save::beta::BetaEnumStr;
use crate::utils::get_enum_number;

// ENUM_Natures::NewEnumerator0 hardy
// 1 lonely
// 2 brave
// 3 adamant
// ENUM_Natures::NewEnumerator6 docile
// ENUM_Natures::NewEnumerator7 relaxed
// 8 impish
// 9 lax
// ENUM_Natures::NewEnumerator10 timid
// 11 hasty
// ENUM_Natures::NewEnumerator12 serious
// 13 jolly
// 14 naive
// 15 modest
// ENUM_Natures::NewEnumerator16 mild
// ENUM_Natures::NewEnumerator17 quiet
// 18 bashful
// 19 rash
// 20 calm
// 21 gentle
// 22 sassy
// 23 careful
// ENUM_Natures::NewEnumerator24 quirky

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nature {
    Hardy,   // 0
    Lonely,  // 1
    Brave,   // 2
    Adamant, // 3
    Docile,  // 6
    Relaxed, // 7
    Impish,  // 8
    Lax,     // 9
    Timid,   // 10
    Hasty,   // 11
    Serious, // 12
    Jolly,   // 13
    Naive,   // 14
    Modest,  // 15
    Mild,    // 16
    Quiet,   // 17
    Bashful, // 18
    Rash,    // 19
    Calm,    // 20
    Gentle,  // 21
    Sassy,   // 22
    Careful, // 23
    Quirky,  // 24
}

impl From<Nature> for i32 {
    fn from(value: Nature) -> Self {
        match value {
            Nature::Hardy => 0,
            Nature::Lonely => 1,
            Nature::Brave => 2,
            Nature::Adamant => 3,
            Nature::Docile => 6,
            Nature::Relaxed => 7,
            Nature::Impish => 8,
            Nature::Lax => 9,
            Nature::Timid => 10,
            Nature::Hasty => 11,
            Nature::Serious => 12,
            Nature::Jolly => 13,
            Nature::Naive => 14,
            Nature::Modest => 15,
            Nature::Mild => 16,
            Nature::Quiet => 17,
            Nature::Bashful => 18,
            Nature::Rash => 19,
            Nature::Calm => 20,
            Nature::Gentle => 21,
            Nature::Sassy => 22,
            Nature::Careful => 23,
            Nature::Quirky => 24,
        }
    }
}

impl TryFrom<BetaEnumStr<'_>> for Nature {
    type Error = ();

    fn try_from(value: BetaEnumStr<'_>) -> Result<Self, Self::Error> {
        match get_enum_number(value.0).ok_or(())? {
            0 => Ok(Nature::Hardy),
            1 => Ok(Nature::Lonely),
            2 => Ok(Nature::Brave),
            3 => Ok(Nature::Adamant),
            6 => Ok(Nature::Docile),
            7 => Ok(Nature::Relaxed),
            8 => Ok(Nature::Impish),
            9 => Ok(Nature::Lax),
            10 => Ok(Nature::Timid),
            11 => Ok(Nature::Hasty),
            12 => Ok(Nature::Serious),
            13 => Ok(Nature::Jolly),
            14 => Ok(Nature::Naive),
            15 => Ok(Nature::Modest),
            16 => Ok(Nature::Mild),
            17 => Ok(Nature::Quiet),
            18 => Ok(Nature::Bashful),
            19 => Ok(Nature::Rash),
            20 => Ok(Nature::Calm),
            21 => Ok(Nature::Gentle),
            22 => Ok(Nature::Sassy),
            23 => Ok(Nature::Careful),
            24 => Ok(Nature::Quirky),
            _ => Err(()),
        }
    }
}
