use crate::*;

// this will get passed through the name of the faction to increment

#[export_name = "turbo/increment_faction_level"]
unsafe extern "C" fn on_increment_faction_level() -> usize {
	// get the player's faction from the function data
	let function_input = os::server::get_command_data();

	// translate function data to string
	let function_input_as_string = String::from_utf8(function_input);

	match function_input_as_string {
		Ok(faction_string) => {
			// should prob have some sort of check that the string is sanitized

			// read the state of the current faction file
			let read_data = os::server::read_file(&format!("factions/{faction_string}"));

			match read_data {
				Ok(data) => {
					// if the remote data exists, deserialize
					let mut current_faction_deserialized = states::FactionState::try_from_slice(&data).unwrap();
					current_faction_deserialized.current_level += 1;

					// write the update to file
					let write_result = os::server::write_file(&format!("factions/{faction_string}"), 
						&current_faction_deserialized.try_to_vec().unwrap());

					match write_result {
						Ok(_) => {
							// no write error, so commit
							os::server::COMMIT
						},
						Err(err) => {
							// write error, log and cancel
							os::server::log(&err.to_string());
							os::server::CANCEL
						}
					}

				},
				Err(err) => {
					// if there is no read data for some reason, cancel
					os::server::log(&err.to_string());
					os::server::CANCEL
				}
			}
		},
		Err(_) => {
			// the data passed through was not valid utf8
			os::server::CANCEL
		}
	}
}