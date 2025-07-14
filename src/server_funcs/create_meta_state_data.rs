use crate::*;

#[export_name = "turbo_program:command_handler/ggmvgxdev3/create_meta_state_data"]
unsafe extern "C" fn on_create_meta_state_data() -> usize {
	log!("test");
	let mut current_meta_state_deserialized = states::MetaState::new();
		
	// if not already present
	let read_result = os::server::fs::read_bytes("metastate");
	if read_result.is_ok() {
		log!("READ ERROR");
		return os::server::command::CANCEL
	}

	current_meta_state_deserialized.last_faction_win = enums::Factions::NoFaction;
	let write_result = os::server::fs::write_bytes("metastate", 
		&borsh::to_vec(&current_meta_state_deserialized).unwrap());
	if write_result.is_err() {
		log!("WRITE ERROR");
		return os::server::command::CANCEL
	}

	os::server::command::COMMIT
}