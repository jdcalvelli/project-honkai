use crate::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerState {
	id: String,
	xp: u64
}

impl PlayerState {
	pub fn load_remote() -> Self {
		// get the user id
		let user_id = os::user_id().unwrap();
		// go try to load existing data at that user id
		let remote_data = os::read_file("project_honkai", &format!("players/{user_id}"));
		// check if data exists
		match remote_data {
			Ok(file) => {
				// if data exists, deserialize the struct and return
				PlayerState::try_from_slice(&file.contents).unwrap()
			},
			Err(_err) => {
				// if data doesnt exist, just return new player state
				PlayerState::new()
			},
		}
	}

	pub fn new() -> Self {
		Self {
			id: os::user_id().unwrap(),
			xp: 0
		}
	}

	pub fn get_xp(&mut self) -> u64{
		self.xp
	}

	pub fn increment_xp(&mut self, amt: u64) {
		// local increment
		self.xp += amt;
	}

	pub fn exec_increment_xp() {
		// push the change, so to speak
		os::exec("project_honkai", "update_player_data", &commands::IncrementXp::new().try_to_vec().unwrap());
	}
}