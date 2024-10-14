use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct GlobalState {
	pub total: u64
}

impl GlobalState {
	pub fn new() -> Self {
		Self {
			total: 0
		}
	}
}