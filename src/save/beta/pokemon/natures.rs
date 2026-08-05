use crate::save::beta::BetaEnumStr;

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

impl From<BetaEnumStr<'_>> for Nature {
    fn from(value: BetaEnumStr) -> Self {
        todo!()
    }
}
