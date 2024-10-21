use crate::*;

#[export_name = "turbo/create_player_data"]
unsafe extern "C" fn on_create_player_data() -> usize {
	// get the user id
	let user_id = program::get_user_id();

	// get the function data, which in this case is which faction the player chose to join
	let function_input = program::get_input_data();

	let function_input_as_string = String::from_utf8(function_input);

	// try to read player state data from file, which returns Result
	let read_result = program::read_file(&format!("players/{user_id}"));

	match read_result {
		Ok(_data) => {
			// if there is data there already, then cancel the creation of the info
			program::CANCEL
		},
		Err(_err) => {
			// if there is NOT a file there already, then create a file
			let mut current_player_deserialized: states::PlayerState = states::PlayerState::new();

			// change the faction value based on function input
			match function_input_as_string{
				Ok(faction_string) => {
					// i might want to put another match statement in here to make sure the right string came through
					current_player_deserialized.faction = faction_string;
					// write the new data to the file
					let write_result = program::write_file(&format!("players/{user_id}"), 
						&current_player_deserialized.try_to_vec().unwrap());

					match write_result {
						Ok(_) => {
							// commit the change if theres no issue in the write
							program::COMMIT
						}
						Err(err) => {
							// cancel the change if there is an error in the write
							program::log(err);
							program::CANCEL
						}
					}
				},
				Err(_) => {
					// what came through was not valid utf8, so cancel
					program::CANCEL
				},

			}
		}
	}
}