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