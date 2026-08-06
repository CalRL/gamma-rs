use crate::save::beta::BetaEnumStr;

#[derive(PartialEq, Debug, Clone)]
pub enum PokeBall {
    PokeBall,
    GreatBall,
    UltraBall,
}

impl PokeBall {
    pub fn from_enum(string: &str) -> Option<PokeBall> {
        let res = match string {
            "/Game/SPRITES/UI/ITEMS/SPR_Item_POKEBALL.SPR_Item_POKEBALL" => Self::PokeBall,
            "/Game/SPRITES/UI/ITEMS/SPR_Item_GREATBALL.SPR_Item_GREATBALL" => Self::GreatBall,
            "/Game/SPRITES/UI/ITEMS/SPR_Item_ULTRABALL.SPR_Item_ULTRABALL" => Self::UltraBall,
            _ => return None,
        };
        Some(res)
    }

    pub fn as_enum(&self) -> &str {
        match self {
            PokeBall::PokeBall => "/Game/SPRITES/UI/ITEMS/SPR_Item_POKEBALL.SPR_Item_POKEBALL",
            PokeBall::GreatBall => "/Game/SPRITES/UI/ITEMS/SPR_Item_GREATBALL.SPR_Item_GREATBALL",
            PokeBall::UltraBall => "/Game/SPRITES/UI/ITEMS/SPR_Item_ULTRABALL.SPR_Item_ULTRABALL",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PokeBall::PokeBall => "Poke Ball",
            PokeBall::GreatBall => "Great Ball",
            PokeBall::UltraBall => "Ultra Ball",
        }
    }
}

impl<'a> TryFrom<BetaEnumStr<'a>> for PokeBall {
    type Error = ();

    fn try_from(value: BetaEnumStr<'a>) -> Result<PokeBall, Self::Error> {
        Ok(match value.0 {
            "/Game/SPRITES/UI/ITEMS/SPR_Item_POKEBALL.SPR_Item_POKEBALL" => PokeBall::PokeBall,
            "/Game/SPRITES/UI/ITEMS/SPR_Item_GREATBALL.SPR_Item_GREATBALL" => PokeBall::GreatBall,
            "/Game/SPRITES/UI/ITEMS/SPR_Item_ULTRABALL.SPR_Item_ULTRABALL" => PokeBall::UltraBall,
            _ => return Err(()),
        })
    }
}
