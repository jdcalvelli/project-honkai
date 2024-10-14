use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerState {
	pub current_level: u64,
	pub current_xp: u64,
	pub xp_needed_for_prev_level: u64,
	pub xp_needed_for_next_level: u64,
}

impl PlayerState {
	pub fn new() -> Self {
		Self {
			current_level: 0,
			current_xp: 0,
			xp_needed_for_prev_level: 0,
			// this val is ultimately dependant on the equation used level to level, so be sure to change here too
			// if equation changes
			xp_needed_for_next_level: 6,
		}
	}
}