use crate::*;

// this will get passed through the name of the faction to create

#[export_name = "turbo/create_faction_data"]
unsafe extern "C" fn on_create_faction_data() -> usize {
	// first get the function data which will be the faction name
	let function_data = program::get_input_data();

	// translate function data to string
	let function_data_as_string = String::from_utf8(function_data);

	// first make sure the function data was acceptable utf8
	match function_data_as_string {
		Ok(faction_string) => {
			program::log(&faction_string);
			// try to read the remote data for associated faction
			// should prob have a check of whether the string is acceptable lol
			let remote_data = program::read_file(&format!("factions/{faction_string}"));

			// check for presense of that data
			match remote_data {
				Ok(_data) => {
					// if the data already exists, just cancel the transaction
					program::CANCEL
				},
				Err(_err) => {
					// if the data doesn't already exist, then create the data based on faction string
					let result = program::write_file(&format!("factions/{faction_string}"), 
						&states::FactionState::new().try_to_vec().unwrap());

					match result {
						Ok(_) => {
							program::COMMIT
						},
						Err(err) => {
							// write error
							program::log(err);
							program::CANCEL
						},
					}
				},
			}

		},
		Err(_) => {
			// utf8 passed through was not acceptable
			program::CANCEL
		}
	}
}