use crate::*;

pub fn update(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState)) -> () {
    // *** UPDATE *** //  

    if player_state_deserialized.did_accept_level_up {
        if !player_state_deserialized.did_accept_tier_up {
            local_state.game_scene = enums::GameScenes::TierUpScene;
        }
        else {
            local_state.game_scene = enums::GameScenes::IdleGameScene;   
        }
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

    sprite!("level_up", x = 79, y = 20);

    if local_state.view_flip {
        sprite!("red_claim_01", x = 150, y = 148);
    }
    else {
        sprite!("red_claim_02", x = 150, y = 148);
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
	if gamepad(0).start.just_pressed() {
        local_state.egghead_state = true;
        // now i need a transaction to set flag back
        os::client::exec("project_honkai", "acknowledge_level_up", &[]);
	}
    else if gamepad(0).start.just_released() {
        local_state.egghead_state = false;
    }
}