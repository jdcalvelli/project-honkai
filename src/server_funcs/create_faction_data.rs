use crate::*;

// this will get passed through the name of the faction to create

#[export_name = "turbo/create_faction_data"]
unsafe extern "C" fn on_create_faction_data() -> usize {
	// first get the function data which will be the faction name
	let function_data = os::server::get_command_data();

	// translate function data to string
	let function_data_as_string = String::from_utf8(function_data);

	// first make sure the function data was acceptable utf8
	match function_data_as_string {
		Ok(faction_string) => {
			// try to read the remote data for associated faction
			// should prob have a check of whether the string is acceptable lol
			let read_result = os::server::read_file(&format!("factions/{faction_string}"));

			// check for presense of that data
			match read_result {
				Ok(_data) => {
					// if the data already exists, just cancel the transaction
					os::server::CANCEL
				},
				Err(_err) => {
					// if the data doesn't already exist, then create the data based on faction string
					let write_result = os::server::write_file(&format!("factions/{faction_string}"), 
						&states::FactionState::new().try_to_vec().unwrap());

					match write_result {
						Ok(_) => {
							os::server::COMMIT
						},
						Err(err) => {
							// write error
							os::server::log(&err.to_string());
							os::server::CANCEL
						},
					}
				},
			}

		},
		Err(_) => {
			// utf8 passed through was not acceptable
			os::server::CANCEL
		}
	}
}