use crate::*;

pub fn update(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** UPDATE *** //  

    if player_state_deserialized.did_accept_level_up {
        local_state.game_scene = enums::GameScenes::IdleGameScene;
    }

    if tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** DRAW *** //

    sprite!("level_up", x = 79, y = 20);

    match player_state_deserialized.items[0].item_type {
        enums::ItemTypes::NoItem => (),
        enums::ItemTypes::Stapler => sprite!("item_stapler", x = 180, y = 92),
        enums::ItemTypes::BendedFolder => sprite!("item_bended_folder", x = 181, y = 88),
        enums::ItemTypes::YogurtCup => sprite!("item_yogurt", x = 183, y = 88),
        enums::ItemTypes::UsedNapkins => sprite!("item_used_napkin", x = 182, y = 88),
        enums::ItemTypes::Eggs => sprite!("item_eggs", x = 180, y = 89),
        enums::ItemTypes::Books => sprite!("item_books", x = 180, y = 82),
        enums::ItemTypes::Box => sprite!("item_box", x = 180, y = 88),
    }

    if local_state.view_flip {
        sprite!("red_claim_01", x = 150, y = 148);
    }
    else {
        sprite!("red_claim_02", x = 150, y = 148);
    }
}

pub fn input(local_state: &mut LocalState, _player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
	if gamepad(0).start.just_pressed() {
        local_state.egghead_state = true;
        // now i need a transaction to set flag back
        os::client::exec(PROGRAM_ID, "acknowledge_level_up", &[]);
	}
    else if gamepad(0).start.just_released() {
        local_state.egghead_state = false;
    }
}