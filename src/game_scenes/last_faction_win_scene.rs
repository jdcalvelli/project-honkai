use crate::*;

pub fn update(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** UPDATE *** //  

    if player_state_deserialized.did_accept_last_faction_winner {
        local_state.game_scene = enums::GameScenes::FactionSelectScene;
    }

    if tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), metastate_deserialized: &states::MetaState) -> () {
    // *** DRAW *** //

    sprite!("on_top_background", x = 88, y = 25);

    match metastate_deserialized.last_faction_win {
        enums::Factions::Green => sprite!("green_on_top", x = 96, y = 30),
        enums::Factions::Purple => sprite!("purple_on_top", x = 96, y = 30),
        enums::Factions::Orange => sprite!("orange_on_top", x = 96, y = 30),
        enums::Factions::NoFaction => (),
    }

    if local_state.view_flip {
        sprite!("red_gogo_01", x = 150, y = 148);
    }
    else {
        sprite!("red_gogo_02", x = 150, y = 148);
    }
}

pub fn input(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    if gamepad(0).start.just_pressed() || mouse(0).left.just_pressed() {
        local_state.egghead_state = true;
        // now i need a transaction to set flag back
        os::client::exec(PROGRAM_ID, "acknowledge_last_faction_winner", &[]);
    }
    else if gamepad(0).start.just_released() || mouse(0).left.just_released() {
        local_state.egghead_state = false;
    }
}