use crate::*;

pub fn update(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // read the player state to see if the bool is flipped basically
    if !player_state_deserialized.did_accept_level_up {
        local_state.game_scene = enums::GameScenes::LevelUpScene;
    }
}

pub fn draw(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), metastate_deserialized: &states::MetaState) -> () {  
    let (green_faction_deserialized, orange_faction_deserialized, purple_faction_deserialized) = faction_states_deserialized;
    // *** DRAW *** //

    // background
    sprite!("background_layer", x = 0, y = 0);
    sprite!("outerframe_layer", x = 0, y = 0);

    // ui
    sprite!("ui_faction_bar", x = 38, y = 21);
    sprite!("ui_xp_bar", x = 39, y = 65);
    // light sprite moving
    sprite!("lights_overlay", x = {98 + (tick() % 36 / 4) * 24}, y = 63);

    // draw correct button based on winning faction
    match metastate_deserialized.last_faction_win {
        enums::Factions::Green => sprite!("green_badge", x = 326, y = 29),
        enums::Factions::Purple => sprite!("purple_badge", x = 326, y = 29),
        enums::Factions::Orange => sprite!("orange_badge", x = 326, y = 29),
        enums::Factions::NoFaction => ()
    }

    // draw correct UI based on player faction
    match player_state_deserialized.faction {
        enums::Factions::Green => {
            // draw the green bar at top
            sprite!("ui_faction_bars_green", x = 43, y = 26);
            rect!(
                x = 87,
                y = 31,
                w = utils::range_map(
                    green_faction_deserialized.current_level as f64, 
                    0., 
                    green_faction_deserialized.max_level as f64, 
                    0., 
                    234.
                ),
                h = 4,
                color = 0x008000ff
            );
            // smaller purple bar
            rect!(
                x = 87,
                y = 44,
                w = utils::range_map(
                    purple_faction_deserialized.current_level as f64,
                    0.,
                    purple_faction_deserialized.max_level as f64,
                    0.,
                    234.
                ),
                h = 2,
                color = 0x800080ff
            );
            // smaller orange bar
            rect!(
                x = 87,
                y = 52,
                w = utils::range_map(
                    orange_faction_deserialized.current_level as f64,
                    0.,
                    orange_faction_deserialized.max_level as f64,
                    0.,
                    234.
                ),
                h = 2,
                color = 0xffa500ff
            );

            // green faction card
            sprite!("ui_faction_profile_green", x = 272, y = 121);
        },
        enums::Factions::Orange => {
            // draw the orange bar at top
            sprite!("ui_faction_bars_orange", x = 43, y = 26);
            rect!(
                x = 87,
                y = 30,
                w = utils::range_map(
                    orange_faction_deserialized.current_level as f64, 
                    0., 
                    orange_faction_deserialized.max_level as f64, 
                    0., 
                    234.
                ),
                h = 6,
                color = 0xffa500ff
            );
            // smaller green bar
            rect!(
                x = 87,
                y = 44,
                w = utils::range_map(
                    green_faction_deserialized.current_level as f64,
                    0.,
                    green_faction_deserialized.max_level as f64,
                    0.,
                    234.
                ),
                h = 2,
                color = 0x008000ff
            );
            // smaller purple bar
            rect!(
                x = 87,
                y = 52,
                w = utils::range_map(
                    purple_faction_deserialized.current_level as f64,
                    0.,
                    purple_faction_deserialized.max_level as f64,
                    0.,
                    234.
                ),
                h = 2,
                color = 0x800080ff
            );

            // orange faction card
            sprite!("ui_faction_profile_orange", x = 272, y = 121);
        },
        enums::Factions::Purple => {
            // draw the purple bar at top
            sprite!("ui_faction_bars_purple", x = 43, y = 26);
            rect!(
                x = 87,
                y = 30,
                w = utils::range_map(
                    purple_faction_deserialized.current_level as f64, 
                    0., 
                    purple_faction_deserialized.max_level as f64, 
                    0., 
                    234.
                ),
                h = 6,
                color = 0x800080ff
            );
            // smaller orange bar
            rect!(
                x = 87,
                y = 44,
                w = utils::range_map(
                    orange_faction_deserialized.current_level as f64,
                    0.,
                    orange_faction_deserialized.max_level as f64,
                    0.,
                    234.
                ),
                h = 2,
                color = 0xffa500ff
            );
            // smaller green bar
            rect!(
                x = 87,
                y = 52,
                w = utils::range_map(
                    green_faction_deserialized.current_level as f64,
                    0.,
                    green_faction_deserialized.max_level as f64,
                    0.,
                    234.
                ),
                h = 2,
                color = 0x008000ff
            );

            // purple faction card
            sprite!("ui_faction_profile_purple", x = 272, y = 121);
        },
        enums::Factions::NoFaction => (),
    }

    // non faction specific draws
    // tier related (draw both the block and the question mark circle)
    sprite!(&format!("ui_tier_{}", player_state_deserialized.current_tier), x = 39, y = 67);
    sprite!(&format!("ui_qm_{}", player_state_deserialized.current_tier), x = 304, y = 71);

    // intra tier level related
    match player_state_deserialized.current_level_in_tier {
        0 => sprite!("ui_lvl_num_0", x = 54, y = 78),
        1 => sprite!("ui_lvl_num_1", x = 59, y = 78),
        2 => sprite!("ui_lvl_num_2", x = 56, y = 78),
        3 => sprite!("ui_lvl_num_3", x = 55, y = 78),
        4 => sprite!("ui_lvl_num_4", x = 55, y = 78),
        5 => sprite!("ui_lvl_num_5", x = 56, y = 78),
        6 => sprite!("ui_lvl_num_6", x = 56, y = 78),
        7 => sprite!("ui_lvl_num_7", x = 56, y = 78),
        8 => sprite!("ui_lvl_num_8", x = 56, y = 78),
        9 => sprite!("ui_lvl_num_9", x = 55, y = 78),
        _ => {
            // if we're on max tier, display infinity
            if player_state_deserialized.current_tier == 9 {
                sprite!("ui_lvl_num_infinite", x = 52, y = 82);
            }
            // if we are not on max tier do nothing so we can pass forward
        }
    }

    // button press rect overlay
    // interpolate btw player xp to 215 always - my range map func is like the map() function in processing
    rect!(
        x = 86, 
        y = 88, 
        w = utils::range_map(
            player_state_deserialized.current_xp as f64, 
            player_state_deserialized.xp_needed_for_prev_level as f64, 
            player_state_deserialized.xp_needed_for_next_level as f64, 
            0., 
            215.
        ),
        h = 6, 
        color = 0xffd700ff
    );

    // pedestal
    sprite!("platform_layer", x = 50, y = 150);

    // foreground
    sprite!("seat", x = 174, y = 163);

    // i could squash this match
    match local_state.egghead_state {
        true => {
            // draw button pressed
            sprite!("button_press", x = 209, y = 152);
            // match the egghead based on player faction
            match player_state_deserialized.faction {
                enums::Factions::Green => {
                    sprite!("egghead_green_press", x = 168, y = 121);
                    if metastate_deserialized.last_faction_win == enums::Factions::Green {
                        // CROWN
                        sprite!("jester_hat_02", x = 160, y = 107);
                    }
                },
                enums::Factions::Orange => {
                    sprite!("egghead_orange_press", x = 168, y = 121);
                    if metastate_deserialized.last_faction_win == enums::Factions::Orange {
                        // CROWN
                        sprite!("jester_hat_02", x = 160, y = 107);
                    }
                },
                enums::Factions::Purple => {
                    sprite!("egghead_purple_press", x = 168, y = 121);
                    if metastate_deserialized.last_faction_win == enums::Factions::Purple {
                        // CROWN
                        sprite!("jester_hat_02", x = 160, y = 107);
                    } 
                },
                enums::Factions::NoFaction => ()
            }
        },
        false => {
            // draw button released
            sprite!("button_release", x = 209, y = 152);
            // draw right egg
            match player_state_deserialized.faction {
                enums::Factions::Green => {
                    sprite!("egghead_green_release", x = 168, y = 121);
                    if metastate_deserialized.last_faction_win == enums::Factions::Green {
                        // CROWN
                        sprite!("jester_hat_01", x = 161, y = 107);
                    } 
                },
                enums::Factions::Orange => {
                    sprite!("egghead_orange_release", x = 168, y = 121);
                    if metastate_deserialized.last_faction_win == enums::Factions::Orange {
                        // CROWN
                        sprite!("jester_hat_01", x = 161, y = 107);
                    } 
                },
                enums::Factions::Purple => {
                    sprite!("egghead_purple_release", x = 168, y = 121);
                    if metastate_deserialized.last_faction_win == enums::Factions::Purple {
                        // CROWN
                        sprite!("jester_hat_01", x = 161, y = 107);
                    } 
                },
                enums::Factions::NoFaction => ()
            }
        },
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

pub fn input(local_state: &mut LocalState, player_state_deserialized: &states::PlayerState, _faction_states_deserialized: &(states::FactionState, states::FactionState, states::FactionState), _metastate_deserialized: &states::MetaState) -> () {
    // *** INPUT *** //

    if gamepad(0).start.just_pressed() {
        local_state.egghead_state = true;
        if player_state_deserialized.current_xp == player_state_deserialized.xp_needed_for_next_level - 1 {
            os::client::exec(PROGRAM_ID, "increment_player_xp", &[]);
            os::client::exec(PROGRAM_ID, "increment_faction_level", &borsh::to_vec(&player_state_deserialized.faction).unwrap());
        }
        else {
            os::client::exec(PROGRAM_ID, "increment_player_xp", &[]);
        }
    }
    else if gamepad(0).start.just_released() {
        local_state.egghead_state = false;
    }
}