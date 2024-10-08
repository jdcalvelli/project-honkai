use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct IncrementXp {
	pub amt: u64,
}

impl IncrementXp {
	pub fn new() -> Self{
		Self {
			amt: 1
		}
	}
}