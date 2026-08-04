use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Types {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Unknown,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

impl From<&str> for Types {
    fn from(value: &str) -> Types {
        match value {
            "Normal" => Types::Normal,
            "Fighting" => Types::Fighting,
            "Flying" => Types::Flying,
            "Poison" => Types::Poison,
            "Ground" => Types::Ground,
            "Rock" => Types::Rock,
            "Bug" => Types::Bug,
            "Ghost" => Types::Ghost,
            "Steel" => Types::Steel,
            "Fire" => Types::Fire,
            "Water" => Types::Water,
            "Grass" => Types::Grass,
            "Electric" => Types::Electric,
            "Psychic" => Types::Psychic,
            "Ice" => Types::Ice,
            "Dragon" => Types::Dragon,
            "Dark" => Types::Dark,
            "Fairy" => Types::Fairy,
            _ => Types::Unknown,
        }
    }
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Types::Normal => "NORMAL",
            Types::Fighting => "FIGHTING",
            Types::Flying => "FLYING",
            Types::Poison => "POISON",
            Types::Ground => "GROUND",
            Types::Rock => "ROCK",
            Types::Bug => "BUG",
            Types::Ghost => "GHOST",
            Types::Steel => "STEEL",
            Types::Unknown => "UNKNOWN",
            Types::Fire => "FIRE",
            Types::Water => "WATER",
            Types::Grass => "GRASS",
            Types::Electric => "ELECTRIC",
            Types::Psychic => "PSYCHIC",
            Types::Ice => "ICE",
            Types::Dragon => "DRAGON",
            Types::Dark => "DARK",
            Types::Fairy => "FAIRY",
        };

        f.write_str(value)
    }
}