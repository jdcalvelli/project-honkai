use crate::*;

pub fn update(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState)) -> () {
	// *** UPDATE *** //

    if player_state_deserialized.faction != enums::Factions::NoFaction {
        local_state.game_scene = enums::GameScenes::IdleGameScene;
    }

    if tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState)) -> () {
    // *** DRAW *** //

    // background
    sprite!("background_layer", x = 0, y = 0);

    sprite!("outerframe_layer", x = 0, y = 0);

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
    
    // not on the computer screen
    sprite!("bg_keyboard", x = 0, y = 210);
    if local_state.egghead_state {
        sprite!("spacebar_02", x = 126, y = 269);
        sprite!("hand_02", x = 48, y = 266);
    }
    else {
        sprite!("spacebar_01", x = 126, y = 268);
        sprite!("hand_01", x = 47, y = 263);
    }
}

pub fn input(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState)) -> () {
	// *** INPUT *** //

	if gamepad(0).left.just_pressed() {
	  // move selector left
	    if local_state.selector_pos != 0
	    {
	        local_state.selector_pos -= 1;
	    }
	}
	else if gamepad(0).right.just_pressed() {
	    // move selector right
	    if local_state.selector_pos != 2
	    {
	        local_state.selector_pos += 1;
	    }
	}            
	else if gamepad(0).start.just_pressed() {
        local_state.egghead_state = true;
        
	    match local_state.selector_pos {
	        0 => {
	            os::client::exec(PROGRAM_ID, "update_player_faction", &borsh::to_vec(&enums::Factions::Orange).unwrap());
	        },
	        1 => {
	            os::client::exec(PROGRAM_ID, "update_player_faction", &borsh::to_vec(&enums::Factions::Green).unwrap());
	        },
	        2 => {
	            os::client::exec(PROGRAM_ID, "update_player_faction", &borsh::to_vec(&enums::Factions::Purple).unwrap());
	        },
	        _ => ()
	    }
	    local_state.game_scene = enums::GameScenes::IdleGameScene;
	}
    else if gamepad(0).start.just_released() {
        local_state.egghead_state = false;
    }

}