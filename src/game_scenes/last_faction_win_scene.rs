use crate::*;

pub fn update(local_state: &mut LocalState) -> () {
    let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

    // *** UPDATE *** //  

    if player_state_deserialized.did_accept_last_faction_winner {
        local_state.game_scene = enums::GameScenes::FactionSelectScene;
    }

    if time::tick() % 16 == 0 {
        local_state.view_flip = !local_state.view_flip;
    }
}

pub fn draw(local_state: &mut LocalState) -> () {
    let metastate_deserialized = utils::deserialize_metastate().unwrap();

    // *** DRAW *** //

    sprite!("on_top_background", x = 88, y = 25);

    match metastate_deserialized.last_faction_win {
        enums::Factions::Green => sprite!("green_on_top", x = 96, y = 30),
        enums::Factions::Purple => sprite!("purple_on_top", x = 96, y = 30),
        enums::Factions::Orange => sprite!("orange_on_top", x = 96, y = 30),
        enums::Factions::NoFaction => (),
    }

    if local_state.view_flip {
        sprite!("red_gogo_01", x = 150, y = 146);
    }
    else {
        sprite!("red_gogo_02", x = 150, y = 146);
    }

    rect!(x = 150, y = 172, w = (86 / 4) * (local_state.num_presses % 4), h = 1, color = 0xffd700ff);

    if !audio::is_playing("egg_on_top") {
        audio::play("egg_on_top");
    }
}

pub fn input(local_state: &mut LocalState) -> () {
    if gamepad::get(0).start.just_pressed() || mouse::screen().left.just_pressed() {
        audio::play("button_hit");
        local_state.egghead_state = true;
        // now i need a transaction to set flag back
        local_state.num_presses += 1;
        if local_state.num_presses == 4 {
            os::client::command::exec_raw(PROGRAM_ID, "acknowledge_last_faction_winner", &[]);
        }
    }
    else if gamepad::get(0).start.just_released() || mouse::screen().left.just_released() {
        audio::play("button_release");
        local_state.egghead_state = false;
    }
}