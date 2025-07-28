use crate::*;

// ADD A CHECK FOR PLAYER ACKNOWLEDGE LAST WINNER

pub fn update(local_state: &mut LocalState) -> () {
    // *** UPDATE *** //

    if time::tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }

    // delay until all things are in, and we click play once
    if utils::deserialize_metastate().is_some()
        && utils::deserialize_factions().is_some()
        && utils::deserialize_player(&local_state.user_id).is_some()
        && local_state.start_game
    {
        // go to the next scene
        local_state.game_scene = enums::GameScenes::FactionSelectScene;
    }
}

pub fn draw(local_state: &mut LocalState) -> () {
    // *** DRAW *** //

    sprite!("gofirst", x = 71, y = 21);
    sprite!("gosecond", x = 199, y = 21);

    sprite!("MVG", x = 48, y = 82);

    // light sprite moving
    sprite!(
        "light_anim_title",
        x = { 57 + (time::tick() % 48 / 4) * 24 },
        y = 80
    );

    if local_state.view_flip {
        sprite!("red_gogo_01", x = 150, y = 148);
    } else {
        sprite!("red_gogo_02", x = 150, y = 148);
    }

    text!("v2.1.0", x = 315, y = 165);
}

pub fn input(local_state: &mut LocalState) -> () {
    if gamepad::get(0).start.just_pressed() && local_state.start_game == false {
        audio::play("button_hit");
        local_state.egghead_state = true;
        // create metastate
        log!("CREATE METASTATE");
        os::client::command::exec_raw(PROGRAM_ID, "create_meta_state_data", &[]);
        log!("CREATE FACTIONS");
        os::client::command::exec_raw(
            PROGRAM_ID,
            "create_faction_data",
            &borsh::to_vec(&enums::Factions::Green).unwrap(),
        );
        os::client::command::exec_raw(
            PROGRAM_ID,
            "create_faction_data",
            &borsh::to_vec(&enums::Factions::Orange).unwrap(),
        );
        os::client::command::exec_raw(
            PROGRAM_ID,
            "create_faction_data",
            &borsh::to_vec(&enums::Factions::Purple).unwrap(),
        );
        log!("CREATE PLAYER");
        os::client::command::exec_raw(
            PROGRAM_ID,
            "create_player_data",
            &borsh::to_vec(&enums::Factions::NoFaction).unwrap(),
        );
        local_state.start_game = true;
    } else if gamepad::get(0).start.just_released() {
        audio::play("button_release");
        local_state.egghead_state = false;
    }
}
