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

	// faction win, cause reset
	if current_faction_deserialized.current_level == current_faction_deserialized.max_level {

		// read the meta state file
		let read_result = os::server::read_file("metastate");
		if read_result.is_err() {
			return os::server::CANCEL
		}

		let mut current_meta_state_deserialized = states::MetaState::try_from_slice(&read_result.unwrap()).unwrap();
		current_meta_state_deserialized.last_faction_win = function_input_deserialized;

		match function_input_deserialized {
		    enums::Factions::Green => current_meta_state_deserialized.green_total_wins += 1,
		    enums::Factions::Orange => current_meta_state_deserialized.orange_total_wins += 1,
		    enums::Factions::Purple => current_meta_state_deserialized.purple_total_wins += 1,
		    enums::Factions::NoFaction => (),
		}

		// set faction win 
		let write_result = os::server::write_file("metastate", &current_meta_state_deserialized.try_to_vec().unwrap());
		if write_result.is_err() {
			return os::server::CANCEL
		}

		// this resets factions, but not players
		// just write empty vec to all the factions
		let write_result = os::server::write_file("factions/green", &[]);
		if write_result.is_err() {
			return os::server::CANCEL
		}

		let write_result = os::server::write_file("factions/orange", &[]);
		if write_result.is_err() {
			return os::server::CANCEL
		}

		let write_result = os::server::write_file("factions/purple", &[]);
		if write_result.is_err() {
			return os::server::CANCEL
		}

		// reset all players
		for player in current_meta_state_deserialized.player_list.iter() {
			let read_result = os::server::read_file(&format!("players/{player}"));
			if read_result.is_err() {
				return os::server::CANCEL
			}
			let mut current_player_data_deserialized = states::PlayerState::try_from_slice(&read_result.unwrap()).unwrap();
			// this is basically the playerstate::new just i dont want to reset the mf uh faction
			let faction_to_keep = current_player_data_deserialized.faction;
			current_player_data_deserialized = states::PlayerState::new();
			current_player_data_deserialized.faction = faction_to_keep;
			current_player_data_deserialized.did_accept_last_faction_winner = false;
			// write
			let write_result = os::server::write_file(&format!("players/{player}"), 
				&current_player_data_deserialized.try_to_vec().unwrap());
			if write_result.is_err() {
				return os::server::CANCEL
			}
		}

		os::server::alert!("RESET");

		os::server::COMMIT
	}
	else {
		// write change
		let write_result = os::server::write_file(&format!("factions/{faction_in_question_as_str}"), 
			&current_faction_deserialized.try_to_vec().unwrap());
		if write_result.is_err() {
			return os::server::CANCEL
		}
	
		os::server::COMMIT
	}
}