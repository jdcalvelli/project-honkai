use crate::*;

#[export_name = "turbo/level_up_player"]
unsafe extern "C" fn on_level_up_player() -> usize {
	// get the user id
	let user_id = os::server::get_user_id();

	// try to read player state data from file, which returns a result
	let read_result = os::server::read_file(&format!("players/{user_id}"));

	// check based on result next steps
	match read_result {
		Ok(data) => {
			// we know data exists, so lets deserialize into player struct
			let mut current_player_deserialized = states::PlayerState::try_from_slice(&data).unwrap();

			// now we can do whatever we want to the state
			// first we increase player level!
			current_player_deserialized.current_level_in_tier += 1;
			// then we need to set the prev needed level xp to whatever currently next needed level xp is
			current_player_deserialized.xp_needed_for_prev_level = current_player_deserialized.xp_needed_for_next_level;
			// then we calculate what the next level xp needs to be
			// currently, the equation is new level + 1 to the 3th power + 5
			current_player_deserialized.xp_needed_for_next_level = 
				(current_player_deserialized.current_level_in_tier + 1).pow(1) + 5;

			// now we write the new state, get a result option
			let write_result = os::server::write_file(&format!("players/{user_id}"), 
				&current_player_deserialized.try_to_vec().unwrap());

			// check based on result opt
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
			// if there is no remote data, cancel execution
			os::server::CANCEL
		}
	}
}