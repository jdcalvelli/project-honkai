use crate::*;

#[export_name = "turbo/create_player_data"]
unsafe extern "C" fn on_create_player_data() -> usize {
	// get the user id
	let user_id = os::server::get_user_id();

	// when i create a player i need to add them to the meta list
	let read_result = os::server::read_file("metastate");
	if read_result.is_err() {
		return os::server::CANCEL
	}
	let mut current_meta_state_deserialized = states::MetaState::try_from_slice(&read_result.unwrap()).unwrap();

	if !current_meta_state_deserialized.player_list.contains(&user_id) {
		current_meta_state_deserialized.player_list.push(user_id.clone());
	}
	
	let write_result = os::server::write_file("metastate",
		&current_meta_state_deserialized.try_to_vec().unwrap());
	if write_result.is_err() {
		return os::server::CANCEL
	}

	// get the function data deserialized
	let function_input_deserialized = os::server::command!(enums::Factions);
	if function_input_deserialized != enums::Factions::NoFaction {
		// if the wrong input came in, cancel
		return os::server::CANCEL
	}

	// below is what i had when i cared if data was already there lol
	// try to read player state data from file, which returns Result
	let read_result = os::server::read_file(&format!("players/{user_id}"));
	if read_result.is_ok() {
		// if there is data there already, then cancel info creation
		return os::server::CANCEL
	}
	
	// there is not currently a player file, lets create one
	let mut current_player_deserialized: states::PlayerState = states::PlayerState::new();
	// set the faction to be what i want, which is no faction at this point
	current_player_deserialized.faction = function_input_deserialized;
	// write the file
	let write_result = os::server::write_file(&format!("players/{user_id}"),
		&current_player_deserialized.try_to_vec().unwrap());
	if write_result.is_err() {
		// write error
		return os::server::CANCEL
	}
	
	os::server::COMMIT
}