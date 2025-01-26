use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum ItemTypes {
	NoItem,
    Stapler,
    BendedFolder,
    YogurtCup,
    UsedNapkins,
    Eggs,
    Books,
    Box
}

impl ItemTypes {
    pub fn to_string(&self) -> String {
        match self {
            Self::NoItem => "Nothing".to_string(),
            Self::Stapler => "Stapler".to_string(),
            Self::BendedFolder => "Bended Folder".to_string(),
            Self::YogurtCup => "Yogurt Cup".to_string(),
            Self::UsedNapkins => "Used Napkins".to_string(),
            Self::Eggs => "Eggs".to_string(),
            Self::Books => "Books".to_string(),
            Self::Box => "Box".to_string()
        }
    }
}