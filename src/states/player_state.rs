use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerState {
	pub xp: u64
}

impl PlayerState {
	pub fn new() -> Self {
		Self {
			xp: 0
		}
	}
}