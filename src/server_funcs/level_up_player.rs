use crate::*;

#[export_name = "turbo/level_up_player"]
unsafe extern "C" fn on_level_up_player() -> usize {
	// get the user id
	let user_id = program::get_user_id();

	// create holder for player state
	let mut current_player_state: states::PlayerState;

	// try to read player state data from file, which returns a result
	let remote_data = program::read_file(&format!("players/{user_id}"));

	// check based on result next steps
	match remote_data {
		Ok(data) => {
			// we know data exists, so lets deserialize into player struct
			current_player_state = states::PlayerState::try_from_slice(&data).unwrap();

			// now we can do whatever we want to the state
			// first we increase player level!
			current_player_state.current_level += 1;
			// then we need to set the prev needed level xp to whatever currently next needed level xp is
			current_player_state.xp_needed_for_prev_level = current_player_state.xp_needed_for_next_level;
			// then we calculate what the next level xp needs to be
			// test purpose, just assume its new level + 1 to the 3th power + 5
			current_player_state.xp_needed_for_next_level = (current_player_state.current_level + 1).pow(3) + 5;

			// now we write the new state, get a result option
			let result = program::write_file(&format!("players/{user_id}"), &current_player_state.try_to_vec().unwrap());

			// check based on result opt
			match result {
				Ok(_) => {
					// commit the change if theres no write error
					program::COMMIT
				},
				Err(err) => {
					// cancel the change if there is a write error
					program::log(err);
					program::CANCEL
				}
			}
		},
		Err(_err) => {
			// if there is no remote data, cancel execution
			program::CANCEL
		}
	}
}