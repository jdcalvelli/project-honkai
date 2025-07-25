use crate::*;

pub fn update(local_state: &mut LocalState) -> () {
    if utils::deserialize_player(&local_state.user_id).is_some() {
        // *** UPDATE *** //

        let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

        if player_state_deserialized.faction != enums::Factions::NoFaction {
            local_state.game_scene = enums::GameScenes::IdleGameScene;
        }

        if time::tick() % 16 == 0 {
            local_state.view_flip = !local_state.view_flip;
        }
    }
}

pub fn draw(local_state: &mut LocalState) -> () {
    // *** DRAW *** //

    sprite!("txt_select_your_suit", x = 62, y = 23);

    sprite!("choose_orange", x = 48, y = 51);
    sprite!("choose_green", x = 150, y = 51);
    sprite!("choose_purple", x = 252, y = 51);

    match local_state.selector_pos {
        0 => {
            if local_state.view_flip {
                sprite!("selected_orange_01", x = 48, y = 101);
                sprite!("selected_overlay_01", x = 40, y = 43);
            }
            else {
                sprite!("selected_orange_02", x = 48, y = 101);
                sprite!("selected_overlay_02", x = 40, y = 43);
            }
        },
        1 => {
            if local_state.view_flip {
                sprite!("selected_green_01", x = 150, y = 101);
                sprite!("selected_overlay_01", x = 142, y = 43);
            }
            else {
                sprite!("selected_green_02", x = 150, y = 101);
                sprite!("selected_overlay_02", x = 142, y = 43);
            }
        },
        2 => {
            if local_state.view_flip {
                sprite!("selected_purple_01", x = 252, y = 101);
                sprite!("selected_overlay_01", x = 244, y = 43);
            }
            else {
                sprite!("selected_purple_02", x = 252, y = 101);
                sprite!("selected_overlay_02", x = 244, y = 43);
            }
        },
        _ => ()
    }

    if local_state.view_flip {
        sprite!("arrow_left", x = 34, y = 22);
        sprite!("arrow_right", x = 327, y = 22);
    }
    else {
        sprite!("arrow_left", x = 32, y = 22);
        sprite!("arrow_right", x = 329, y = 22);
    }
}

pub fn input(local_state: &mut LocalState) -> () {
	// *** INPUT *** //

	if gamepad::get(0).left.just_pressed() {
	  // move selector left
	    if local_state.selector_pos != 0
	    {
	        local_state.selector_pos -= 1;
	    }
	}
	else if gamepad::get(0).right.just_pressed() {
	    // move selector right
	    if local_state.selector_pos != 2
	    {
	        local_state.selector_pos += 1;
	    }
	}            
	else if gamepad::get(0).start.just_pressed() {
        audio::play("button_hit");
        local_state.egghead_state = true;
        
	    match local_state.selector_pos {
	        0 => {
	            os::client::command::exec_raw(PROGRAM_ID, "update_player_faction", &borsh::to_vec(&enums::Factions::Orange).unwrap());
	        },
	        1 => {
	            os::client::command::exec_raw(PROGRAM_ID, "update_player_faction", &borsh::to_vec(&enums::Factions::Green).unwrap());
	        },
	        2 => {
	            os::client::command::exec_raw(PROGRAM_ID, "update_player_faction", &borsh::to_vec(&enums::Factions::Purple).unwrap());
	        },
	        _ => ()
	    }
        audio::play("character_select");
	    local_state.game_scene = enums::GameScenes::IdleGameScene;
	}
    else if gamepad::get(0).start.just_released() || mouse::screen().left.just_released() {
        audio::play("button_release");
        local_state.egghead_state = false;
    }

}