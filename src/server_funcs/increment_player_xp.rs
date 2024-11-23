use crate::*;

#[export_name = "turbo/increment_player_xp"]
unsafe extern "C" fn on_increment_player_xp() -> usize {
	// get the user id
	let user_id = os::server::get_user_id();

	// try to read player state data from file, which returns Result
	let read_result = os::server::read_file(&format!("players/{user_id}"));

	// check based on result what we should do next
	match read_result {
		Ok(data) => {
			// if data exists, deserialize the struct and set holder to it
			let mut current_player_deserialized = states::PlayerState::try_from_slice(&data).unwrap();

			// this is the increase!
			current_player_deserialized.current_xp += 1;

			// if players xp is high enough, level up
            if current_player_deserialized.current_xp == current_player_deserialized.xp_needed_for_next_level {
				// first we increase player level!
				current_player_deserialized.current_level_in_tier += 1;
				// then we need to set the prev needed level xp to whatever currently next needed level xp is
				current_player_deserialized.xp_needed_for_prev_level = current_player_deserialized.xp_needed_for_next_level;
				// then we calculate what the next level xp needs to be
				// currently, the equation is new level + 1 to the 3th power + 5
				current_player_deserialized.xp_needed_for_next_level = 
					(current_player_deserialized.current_level_in_tier + 1).pow(3) + 5;
            }

			// if the players tier is high enough to tier up, also tier up the player
			if current_player_deserialized.current_level_in_tier == 10 && current_player_deserialized.current_tier != 9 {
				// this is the tier up, so we need to increment tier, and return all else to zero?
				current_player_deserialized.current_tier += 1;
				current_player_deserialized.current_level_in_tier = 0;
				current_player_deserialized.current_xp = 0;
				current_player_deserialized.xp_needed_for_prev_level = 0;
				// this 6 value is based on the current equation which can be found in player_state.rs
				// assumption rn is that it will be the same xp curve regardless of tier
				current_player_deserialized.xp_needed_for_next_level = 6;
			}

			// write the data to the file
			let write_result = os::server::write_file(&format!("players/{user_id}"), 
				&current_player_deserialized.try_to_vec().unwrap());

			match write_result {
				Ok(_) => {
					// commit the change if theres no issue writing data
					os::server::COMMIT
				}
				Err(err) => {
					// cancel the change if there is an error in writing for some reason
					os::server::log(&err.to_string());
					os::server::CANCEL
				}
			}
		},
		Err(_err) => {
			// if there is no read data, cancel the execution
			os::server::CANCEL

		},
	}
}
