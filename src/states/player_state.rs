use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerState {
	xp: u64
}

impl PlayerState {
	pub fn new() -> Self {
		Self {
			xp: 0
		}
	}

	pub fn get_xp(&mut self) -> u64{
		self.xp
	}

	pub fn increment_xp(&mut self) {
		self.xp += 1;
	}
}