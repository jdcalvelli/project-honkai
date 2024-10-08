use crate::*;

#[export_name = "turbo/update_player_data"]
unsafe extern "C" fn on_update_player_data() -> usize {
	// get the user id
	let user_id = program::get_user_id();
	// get the data passed into the function
	let input_data = program::get_input_data();
	
	// figure out some sort of naming structure for this?
	let increment_xp = commands::IncrementXp::try_from_slice(&input_data).unwrap();

	let remote_data = program::read_file(&format!("players/{user_id}"));
		// check if data exists
		let mut player_state = match remote_data {
			Ok(file) => {
				program::log("getting global state");
				// if data exists, deserialize the struct and return
				states::PlayerState::try_from_slice(&file).unwrap()
			},
			Err(_err) => {
				// BECAUSE OF AN ASYNC OPERATION?
				program::log("erroring out");
				// if data doesnt exist, just return new player state
				states::PlayerState::new()
			},
		};

	player_state.increment_xp(increment_xp.amt);

	// write the data to the file
	let result = program::write_file(&format!("players/{user_id}"), &player_state.try_to_vec().unwrap());

	match result {
		Ok(_data) => {
			// commit the change if theres no issue
			program::COMMIT
		}
		Err(_err) => {
			// cancel the change if there is an error
			program::CANCEL
		}
	}
}
