use crate::*;

// this will get passed through the name of the faction to increment

#[export_name = "turbo/increment_faction_level"]
unsafe extern "C" fn on_increment_faction_level() -> usize {
	// get function input deserialized
	let function_input_deserialized = os::server::command!(enums::Factions);
	
	// create var for which faction we're talking about
	// in the event that the faction that came through is nofaction cancel
	let faction_in_question_as_str: &str;
	match function_input_deserialized {
		enums::Factions::Green => faction_in_question_as_str = "green",
		enums::Factions::Orange => faction_in_question_as_str = "orange",
		enums::Factions::Purple => faction_in_question_as_str = "purple",
		enums::Factions::NoFaction => return os::server::CANCEL
	}
	
	// read based on what we got
	let read_result = os::server::read_file(&format!("factions/{faction_in_question_as_str}"));
	if read_result.is_err() {
		// no data, error
		return os::server::CANCEL
	}
	
	let mut current_faction_deserialized = states::FactionState::try_from_slice(&read_result.unwrap()).unwrap();
	current_faction_deserialized.current_level += 1;
	
	// write change
	let write_result = os::server::write_file(&format!("factions/{faction_in_question_as_str}"), 
		&current_faction_deserialized.try_to_vec().unwrap());
	if write_result.is_err() {
		return os::server::CANCEL
	}
	
	os::server::COMMIT
}