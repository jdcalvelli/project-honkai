use crate::*;

#[export_name = "turbo/increment_player_xp"]
unsafe extern "C" fn on_increment_player_xp() -> usize {
	// get the user id
	let user_id = program::get_user_id();
	
	// create holder for player state
	let mut current_player_state: states::PlayerState;

	// try to read player state data from file, which returns Result
	let remote_data = program::read_file(&format!("players/{user_id}"));

	// check based on result what we should do next
	match remote_data {
		Ok(file) => {
			// if data exists, deserialize the struct and set holder to it
			current_player_state = states::PlayerState::try_from_slice(&file).unwrap();

			// this is the increase!
			current_player_state.xp += 1;

			// write the data to the file
			let result = program::write_file(&format!("players/{user_id}"), &current_player_state.try_to_vec().unwrap());

			match result {
				Ok(_data) => {
					// commit the change if theres no issue writing data
					program::COMMIT
				}
				Err(_err) => {
					// cancel the change if there is an error in writing for some reason
					program::CANCEL
				}
			}
		},
		Err(_err) => {
			// if there is no remote data, cancel the execution
			//current_player_state = states::PlayerState::new();
			program::CANCEL

		},
	}
}
