use crate::*;

#[export_name = "turbo/create_player_data"]
unsafe extern "C" fn on_create_player_data() -> usize {
	// get the user id
	let user_id = program::get_user_id();

	// try to read player state data from file, which returns Result
	let remote_data = program::read_file(&format!("players/{user_id}"));

	match remote_data {
		Ok(_file) => {
			// if there is a file there already, then cancel the creation of the info
			program::CANCEL
		},
		Err(_err) => {
			// if there is NOT a file there already, then create a file
			let current_player_state: states::PlayerState = states::PlayerState::new();

			// write the new data to the file
			let result = program::write_file(&format!("players/{user_id}"), &current_player_state.try_to_vec().unwrap());

			match result {
				Ok(_data) => {
					// commit the change if theres no issue in the write
					program::COMMIT
				}
				Err(_err) => {
					// cancel the change if there is an error in the write
					program::CANCEL
				}
			}
		}
	}
}