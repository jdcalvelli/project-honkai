use crate::*;

#[export_name = "turbo/create_meta_state_data"]
unsafe extern "C" fn on_create_meta_state_data() -> usize {
	let mut current_meta_state_deserialized = states::MetaState::new();
		
	// if not already present
	let read_result = os::server::read_file("metastate");
	if read_result.is_ok() {
		os::server::log("READ ERROR");
		return os::server::CANCEL
	}

	current_meta_state_deserialized.last_faction_win = enums::Factions::NoFaction;
	let write_result = os::server::write_file("metastate", 
		&current_meta_state_deserialized.try_to_vec().unwrap());
	if write_result.is_err() {
		os::server::log("WRITE ERROR");
		return os::server::CANCEL
	}

	os::server::COMMIT
}