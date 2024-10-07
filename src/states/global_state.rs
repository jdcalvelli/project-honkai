use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct GlobalState {
	total: u64
}

impl GlobalState {
	pub fn new() -> Self {
		Self {
			total: 0
		}
	}

	pub fn get_total(&mut self) -> u64 {
		self.total
	}

	pub fn increment_total(&mut self) {
		self.total += 1;
	}
}