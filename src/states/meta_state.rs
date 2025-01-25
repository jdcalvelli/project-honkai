use crate::*;



#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MetaState {
	pub last_faction_win: enums::Factions,
	pub player_list: Vec<String>
}

impl MetaState {
	pub fn new() -> Self {
		Self {
			last_faction_win: enums::Factions::NoFaction,
			player_list: vec![]
		}
	}
}