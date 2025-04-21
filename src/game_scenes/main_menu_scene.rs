use crate::*;

// ADD A CHECK FOR PLAYER ACKNOWLEDGE LAST WINNER

pub fn update(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** UPDATE *** //  

    if tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** DRAW *** //

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

    text!("v1.1.1", x = 315, y = 165);
}

pub fn input(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
	if gamepad(0).start.just_pressed() || mouse(0).left.just_pressed() {
        audio::play("button_hit");
        local_state.egghead_state = true;
        // just go to the next scene
        local_state.game_scene = enums::GameScenes::FactionSelectScene;
	}
    else if gamepad(0).start.just_released() || mouse(0).left.just_released() {
        audio::play("button_release");
        local_state.egghead_state = false;
    }
}