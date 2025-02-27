use crate::*;

pub fn update(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** UPDATE *** //  

    if player_state_deserialized.did_accept_tier_up {
        local_state.game_scene = enums::GameScenes::IdleGameScene;
    }

    if tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** DRAW *** //

    sprite!("tier_up", x = 88, y = 25);

    sprite!(&format!("ui_tier_{}", player_state_deserialized.current_tier - 1), x = 119, y = 43);
    sprite!(&format!("ui_tier_{}", player_state_deserialized.current_tier), x = 218, y = 43);

    if local_state.view_flip {
        sprite!("red_gogo_01", x = 149, y = 144);
    }
    else {
        sprite!("red_gogo_02", x = 149, y = 144);
    }
}

pub fn input(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
	if gamepad(0).start.just_pressed() || mouse(0).left.just_pressed() {
        local_state.egghead_state = true;
        // now i need a transaction to set flag back
        os::client::exec(PROGRAM_ID, "acknowledge_tier_up", &[]);
	}
    else if gamepad(0).start.just_released() || mouse(0).left.just_released() {
        local_state.egghead_state = false;
    }
}