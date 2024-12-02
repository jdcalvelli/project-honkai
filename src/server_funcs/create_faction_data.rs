use crate::*;

// this will get passed through the name of the faction to create

#[export_name = "turbo/create_faction_data"]
unsafe extern "C" fn on_create_faction_data() -> usize {

	// get the function data deserialized
	let function_data_deserialized = os::server::command!(enums::Factions);
	
	match function_data_deserialized {
		enums::Factions::NoFaction => {
			// error, we dont want to write a file for no faction
			os::server::CANCEL
		}
		enums::Factions::Green => {
			write_faction_file_if_not_there("green")
		},
		enums::Factions::Purple => {
			write_faction_file_if_not_there("orange")
		},
		enums::Factions::Orange => {
			write_faction_file_if_not_there("purple")
		}
	}
}

fn write_faction_file_if_not_there(faction_name: &str) -> usize {
	// check if the data exists
	let read_result = os::server::read_file(&format!("factions/{faction_name}"));
	if read_result.is_ok() {
		// data already exists so cancel
		return os::server::CANCEL
	}
	// data doesnt exist so write it
	let write_result = os::server::write_file(&format!("factions/{faction_name}"), 
		&states::FactionState::new().try_to_vec().unwrap());
	if write_result.is_err() {
		// write error
		return os::server::CANCEL
	}
	
	os::server::COMMIT
}