use crate::*;

#[export_name = "turbo/increment_player_xp"]
unsafe extern "C" fn on_increment_player_xp() -> usize {
	// get the user id
	let user_id = program::get_user_id();

	// try to read player state data from file, which returns Result
	let read_result = program::read_file(&format!("players/{user_id}"));

	// check based on result what we should do next
	match read_result {
		Ok(data) => {
			// if data exists, deserialize the struct and set holder to it
			let mut current_player_deserialized = states::PlayerState::try_from_slice(&data).unwrap();

			// this is the increase!
			current_player_deserialized.current_xp += 1;

			// write the data to the file
			let write_result = program::write_file(&format!("players/{user_id}"), 
				&current_player_deserialized.try_to_vec().unwrap());

			match write_result {
				Ok(_) => {
					// commit the change if theres no issue writing data
					program::COMMIT
				}
				Err(err) => {
					// cancel the change if there is an error in writing for some reason
					program::log(err);
					program::CANCEL
				}
			}
		},
		Err(_err) => {
			// if there is no read data, cancel the execution
			program::CANCEL

		},
	}
}
