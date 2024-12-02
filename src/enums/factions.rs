use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Factions {
	Green,
	Orange,
	Purple,
	NoFaction
}