use crate::*;

#[export_name = "turbo/tier_up_player"]
unsafe extern "C" fn on_tier_up_player() -> usize {
	// get the user id
	let user_id = os::server::get_user_id();

	// try to read player state data from file, returns result
	let read_result = os::server::read_file(&format!("players/{user_id}"));

	// check based on result
	match read_result {
		Ok(data) => {
			// data exists, time to deserialize
			let mut current_player_deserialized = states::PlayerState::try_from_slice(&data).unwrap();

			// now we edit the state according to our needs
			// this is the tier up, so we need to increment tier, and return all else to zero?
			current_player_deserialized.current_tier += 1;
			current_player_deserialized.current_level_in_tier = 0;
			current_player_deserialized.current_xp = 0;
			current_player_deserialized.xp_needed_for_prev_level = 0;
			// this 6 value is based on the current equation which can be found in player_state.rs
			// assumption rn is that it will be the same xp curve regardless of tier
			current_player_deserialized.xp_needed_for_next_level = 6;

			// now we write the new state, and confirm execution based on result
			let write_result = os::server::write_file(&format!("players/{user_id}"), 
				&current_player_deserialized.try_to_vec().unwrap());

			match write_result {
				Ok(_) => {
					// commit the change if theres no write error
					os::server::COMMIT
				},
				Err(err) => {
					// cancel the change if there is a write error
					os::server::log(&err.to_string());
					os::server::CANCEL
				}
			}
		},
		Err(_err) => {
			// if there is no remote data on the server, cancel
			os::server::CANCEL
		}
	}
}