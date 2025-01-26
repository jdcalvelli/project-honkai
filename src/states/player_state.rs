use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerState {
	pub faction: enums::Factions,
	pub items: Vec<structs::Item>,
	pub current_tier: u64,
	pub current_level_in_tier: u64,
	pub current_xp: u64,
	pub xp_needed_for_prev_level: u64,
	pub xp_needed_for_next_level: u64,
	pub did_accept_level_up: bool,
	pub did_accept_tier_up: bool,
	pub did_accept_last_faction_winner: bool,
}

impl PlayerState {
	pub fn new() -> Self {
		Self {
			faction: enums::Factions::NoFaction,
			items: vec![structs::Item::new(enums::ItemTypes::NoItem, 0xffffffff)],
			current_tier: 0,
			current_level_in_tier: 0,
			current_xp: 0,
			xp_needed_for_prev_level: 0,
			// this val is ultimately dependant on the equation used level to level, so be sure to change here too
			// if equation changes
			xp_needed_for_next_level: 3,
			did_accept_level_up: true,
			did_accept_tier_up: true,
			did_accept_last_faction_winner: true,
		}
	}
}