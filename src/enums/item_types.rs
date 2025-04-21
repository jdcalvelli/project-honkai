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
    Box,
    Pantaloons,
    PuzzlePiece,
    BowlingBall,
    GOGOBucks,
    CreditCard,
    GOGOCard,
    GOGOSticker,
    InGameTokens,
    Sock,
    BakedBeans,
    Cube
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
            Self::Box => "Box".to_string(),
            Self::Pantaloons => "Pantaloons".to_string(),
            Self::PuzzlePiece => "Puzzle Piece".to_string(),
            Self::BowlingBall => "Bowling Ball".to_string(),
            Self::GOGOBucks => "GOGO! Bucks".to_string(),
            Self::GOGOCard => "GOGO! Card".to_string(),
            Self::CreditCard => "Credit Card".to_string(),
            Self::GOGOSticker => "GOGO! Sticker".to_string(),
            Self::InGameTokens => "In-Game Tokens".to_string(),
            Self::Sock => "Sock".to_string(),
            Self::BakedBeans => "Baked Beans".to_string(),
            Self::Cube => "Cube".to_string()
        }
    }
}