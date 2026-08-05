use core::fmt;
use std::fmt::Formatter;

pub enum Gender {
    Male,
    Female,
    Genderless,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Genderless => "Genderless",
        };

        f.write_str(s)
    }
}
///
/// Will error out if invalid gender string is provided
impl TryFrom<&str> for Gender {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            v if v.eq_ignore_ascii_case("MALE")
                || v.eq_ignore_ascii_case("ENUM_Gender::NewEnumerator0") =>
            {
                Ok(Gender::Male)
            }

            v if v.eq_ignore_ascii_case("FEMALE")
                || v.eq_ignore_ascii_case("ENUM_Gender::NewEnumerator1") =>
            {
                Ok(Gender::Female)
            }

            v if v.eq_ignore_ascii_case("FEMALE")
                || v.eq_ignore_ascii_case("ENUM_Gender::NewEnumerator1") =>
            {
                Ok(Gender::Genderless)
            }
            _ => Err(()),
        }
    }
}

impl Gender {
    pub fn as_game_enum(&self) -> &str {
        match self {
            Gender::Male => "ENUM_Gender::NewEnumerator0",
            Gender::Female => "ENUM_Gender::NewEnumerator1",
            Gender::Genderless => "ENUM_Gender::NewEnumerator2",
        }
    }

    pub fn from_enum(string: &str) -> Option<Gender> {
        let gender = match string {
            "ENUM_Gender::NewEnumerator0" => Gender::Male,
            "ENUM_Gender::NewEnumerator1" => Gender::Female,
            "ENUM_Gender::NewEnumerator2" => Gender::Genderless,
            _ => return None,
        };

        Some(gender)
    }

    /// Converts to UE 4/5 ENUM
    pub fn as_enum(&self) -> String {
        let string = match &self {
            Gender::Male => "ENUM_Gender::NewEnumerator0",
            Gender::Female => "ENUM_Gender::NewEnumerator1",
            Gender::Genderless => "ENUM_Gender::NewEnumerator2",
        };

        string.to_string()
    }
}
