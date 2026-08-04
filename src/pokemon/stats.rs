#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Stats {
    CurrentHp,
    MaxHp,
    ATK,
    DEF,
    SATK,
    SDEF,
    SPEED,
}

impl Stats {
    pub fn iter() -> impl Iterator<Item = Stats> {
        [
            Stats::CurrentHp,
            Stats::MaxHp,
            Stats::ATK,
            Stats::DEF,
            Stats::SATK,
            Stats::SDEF,
            Stats::SPEED,
        ]
        .into_iter()
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Stats::CurrentHp => "CurrentHP",
            Stats::MaxHp => "MaxHP",
            Stats::ATK => "ATK",
            Stats::DEF => "DEF",
            Stats::SATK => "SATK",
            Stats::SDEF => "SDEF",
            Stats::SPEED => "SPEED",
        }
    }
}

/// Will only error if wrong str is provided
impl TryFrom<&str> for Stats {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let stat: Self = match value {
            "CurrentHP" => Stats::CurrentHp,
            "MaxHP" => Stats::MaxHp,
            "ATK" => Stats::ATK,
            "DEF" => Stats::DEF,
            "SATK" => Stats::SATK,
            "SDEF" => Stats::SDEF,
            "SPEED" => Stats::SPEED,
            _ => return Err(()),
        };
        Ok(stat)
    }
}
