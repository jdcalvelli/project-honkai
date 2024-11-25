use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerState {
	pub faction: String,
	pub current_tier: u64,
	pub current_level_in_tier: u64,
	pub current_xp: u64,
	pub xp_needed_for_prev_level: u64,
	pub xp_needed_for_next_level: u64,
	pub did_accept_level_up: bool,
	pub did_accept_tier_up: bool,
}

impl PlayerState {
	pub fn new() -> Self {
		Self {
			faction: "none".to_string(),
			current_tier: 0,
			current_level_in_tier: 0,
			current_xp: 0,
			xp_needed_for_prev_level: 0,
			// this val is ultimately dependant on the equation used level to level, so be sure to change here too
			// if equation changes
			xp_needed_for_next_level: 6,
			did_accept_level_up: true,
			did_accept_tier_up: true,
		}
	}
}