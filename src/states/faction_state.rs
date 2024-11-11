use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FactionState {
	pub current_level: u64,
	pub max_level: u64,
}

impl FactionState {
	pub fn new() -> Self {
		Self {
			current_level: 0,
			max_level: 1000,
		}
	}
}