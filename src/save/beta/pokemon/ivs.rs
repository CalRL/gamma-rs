use crate::save::beta::pokemon::stats::Stats;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum IVs {
    HP,
    ATK,
    DEF,
    SATK,
    SDEF,
    SPEED,
}

impl IVs {
    pub fn as_str(&self) -> &str {
        match self {
            IVs::HP => "HP",
            IVs::ATK => "ATK",
            IVs::DEF => "DEF",
            IVs::SATK => "SATK",
            IVs::SDEF => "SDEF",
            IVs::SPEED => "SPEED",
        }
    }
    pub fn iter() -> impl Iterator<Item = IVs> {
        [
            IVs::HP,
            IVs::ATK,
            IVs::DEF,
            IVs::SATK,
            IVs::SDEF,
            IVs::SPEED,
        ]
        .into_iter()
    }
}

impl TryFrom<Stats> for IVs {
    type Error = ();

    fn try_from(value: Stats) -> Result<Self, Self::Error> {
        let val = match value {
            Stats::MaxHp => IVs::HP,
            Stats::ATK => IVs::ATK,
            Stats::DEF => IVs::DEF,
            Stats::SATK => IVs::SATK,
            Stats::SDEF => IVs::SDEF,
            Stats::SPEED => IVs::SPEED,
            _ => return Err(()),
        };

        Ok(val)
    }
}

#[derive(Clone, Debug)]
pub struct IVSpread {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub satk: i32,
    pub sdef: i32,
    pub speed: i32,
}

impl IVs {
    pub fn get_index(self) -> usize {
        match self {
            IVs::HP => 0,
            IVs::ATK => 1,
            IVs::DEF => 2,
            IVs::SATK => 3,
            IVs::SDEF => 4,
            IVs::SPEED => 5,
        }
    }
}

impl FromStr for IVs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iv: IVs = match s {
            "HP" => IVs::HP,
            "ATK" => IVs::ATK,
            "DEF" => IVs::DEF,
            "SATK" => IVs::SATK,
            "SDEF" => IVs::SDEF,
            "SPEED" => IVs::SPEED,
            _ => return Err(()),
        };
        Ok(iv)
    }
}
