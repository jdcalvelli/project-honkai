use crate::*;

pub fn update(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState)) -> () {
    // *** UPDATE *** //  

    if tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState)) -> () {
    // *** DRAW *** //

    // background
    sprite!("background_layer", x = 0, y = 0);
    sprite!("outerframe_layer", x = 0, y = 0);

    sprite!("gofirst", x = 71, y = 21);
    sprite!("gosecond", x = 199, y = 21);

    sprite!("MVG", x = 48, y = 82);

    // light sprite moving
    sprite!("light_anim_title", x = {57 + (tick() % 48 / 4) * 24}, y = 80);

    if local_state.view_flip {
        sprite!("red_gogo_01", x = 150, y = 148);
    }
    else {
        sprite!("red_gogo_02", x = 150, y = 148);
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
        // just go to the next scene
        local_state.game_scene = enums::GameScenes::FactionSelectScene;
	}
    else if gamepad(0).start.just_released() {
        local_state.egghead_state = false;
    }
}