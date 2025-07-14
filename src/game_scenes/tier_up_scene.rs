use crate::*;

pub fn update(local_state: &mut LocalState) -> () {
    if utils::deserialize_player(&local_state.user_id).is_some() {
        let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

        // *** UPDATE *** //  

        if player_state_deserialized.did_accept_tier_up {
            local_state.game_scene = enums::GameScenes::IdleGameScene;
        }

        if time::tick() % 16 == 0 {
            local_state.view_flip = !local_state.view_flip;
        }
    }
}

pub fn draw(local_state: &mut LocalState) -> () {
    if utils::deserialize_player(&local_state.user_id).is_some() {
        let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

        // *** DRAW *** //

        sprite!("tier_up", x = 88, y = 25);

        let hold = &format!("ui_tier_{}", player_state_deserialized.current_tier - 1);
        sprite!(hold, x = 119, y = 43);
        let hold = &format!("ui_tier_{}", player_state_deserialized.current_tier);
        sprite!(hold, x = 218, y = 43);

        if local_state.view_flip {
            sprite!("red_gogo_01", x = 149, y = 146);
        }
        else {
            sprite!("red_gogo_02", x = 149, y = 146);
        }

        rect!(x = 150, y = 172, w = (86 / 4) * (local_state.num_presses % 4), h = 1, color = 0xffd700ff);

        if !audio::is_playing("tier_up") {
            audio::play("tier_up");
        }
    }
}

pub fn input(local_state: &mut LocalState) -> () {
	if gamepad::get(0).start.just_pressed() || mouse::screen().left.just_pressed() {
        audio::play("button_hit");
        local_state.egghead_state = true;
        local_state.num_presses += 1;
        // now i need a transaction to set flag back
        if local_state.num_presses == 8 {
            os::client::command::exec_raw(PROGRAM_ID, "acknowledge_tier_up", &[]);
        }
	}
    else if gamepad::get(0).start.just_released() || mouse::screen().left.just_released() {
        audio::play("button_release");
        local_state.egghead_state = false;
    }
}