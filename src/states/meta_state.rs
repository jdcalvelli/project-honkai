use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MetaState {
	pub last_faction_win: enums::Factions,
	pub orange_total_wins: u32,
	pub green_total_wins: u32,
	pub purple_total_wins: u32,
	pub player_list: Vec<String>
}

impl MetaState {
	pub fn new() -> Self {
		Self {
			last_faction_win: enums::Factions::NoFaction,
			orange_total_wins: 0,
			green_total_wins: 0,
			purple_total_wins: 0,
			player_list: vec![]
		}
	}
}