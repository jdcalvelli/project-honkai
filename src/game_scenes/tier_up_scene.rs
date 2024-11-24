use crate::*;

pub fn update(local_state: &mut LocalState, player_state_deserialized: &PlayerState, _faction_states_deserialized: &(FactionState, FactionState, FactionState)) -> () {
    // *** UPDATE *** //  

    if player_state_deserialized.did_accept_tier {
        local_state.game_scene = enums::GameScenes::IdleGameScene;
    }

    if tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState, player_state_deserialized: &PlayerState, _faction_states_deserialized: &(FactionState, FactionState, FactionState)) -> () {
    // *** DRAW *** //

    // background
    sprite!("background_layer", x = 0, y = 0);
    sprite!("outerframe_layer", x = 0, y = 0);

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

pub fn input(_local_state: &mut LocalState, _player_state_deserialized: &PlayerState, _faction_states_deserialized: &(FactionState, FactionState, FactionState)) -> () {
	if gamepad(0).start.just_pressed() {
        // now i need a transaction to set flag back
        os::client::exec("project_honkai", "acknowledge_tier_up", &[]);
	}
}