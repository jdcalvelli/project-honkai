use crate::*;

#[export_name = "turbo/increment_player_xp"]
unsafe extern "C" fn on_increment_player_xp() -> usize {
	// get the user id
	let user_id = os::server::get_user_id();

	// try to read player state data from file, which returns Result
	let read_result = os::server::read_file(&format!("players/{user_id}"));
	if read_result.is_err() {
		// read error
		return os::server::CANCEL
	}
	
	let mut current_player_deserialized = states::PlayerState::try_from_slice(&read_result.unwrap()).unwrap();
	current_player_deserialized.current_xp +=1;
	// if players xp is high enough, level up
	if current_player_deserialized.current_xp == current_player_deserialized.xp_needed_for_next_level {
		// first we increase player level!
		current_player_deserialized.current_level_in_tier += 1;
		// then we need to set the prev needed level xp to whatever currently next needed level xp is
		current_player_deserialized.xp_needed_for_prev_level = current_player_deserialized.xp_needed_for_next_level;
		// then we calculate what the next level xp needs to be
		// currently, the equation is new level + 1 to the 3th power + 5
		current_player_deserialized.xp_needed_for_next_level = 
			(current_player_deserialized.current_level_in_tier + 1).pow(1) + 5;
	
		current_player_deserialized.did_accept_level_up = false;
	}
	// if the players level is high enough to tier up, also tier up the player
	if current_player_deserialized.current_level_in_tier == 10 && current_player_deserialized.current_tier != 9 {
		// this is the tier up, so we need to increment tier, and return all else to zero?
		current_player_deserialized.current_tier += 1;
		current_player_deserialized.current_level_in_tier = 0;
		current_player_deserialized.current_xp = 0;
		current_player_deserialized.xp_needed_for_prev_level = 0;
		// this 6 value is based on the current equation which can be found in player_state.rs
		// assumption rn is that it will be the same xp curve regardless of tier
		current_player_deserialized.xp_needed_for_next_level = 6;
	
		current_player_deserialized.did_accept_tier_up = false;
	}
	
	// try write
	let write_result = os::server::write_file(&format!("players/{user_id}"), 
		&current_player_deserialized.try_to_vec().unwrap());
	if write_result.is_err() {
		// write error
		return os::server::CANCEL
	}
	
	os::server::COMMIT
}
