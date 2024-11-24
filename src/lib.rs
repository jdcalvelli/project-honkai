use states::{FactionState, PlayerState};

mod utils;
mod states;
mod server_funcs;

turbo::cfg! {r#"
    name = "project-honkai"
    version = "0.1.0"
    author = "jd calvelli and devinne moses"
    description = "a game about modern games"
    [settings]
    resolution = [384, 216]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

turbo::init! {
    // something only the player will see? menus, interpolated values, etc, local ui state eg, tweens, transitions, etc
    // dont need to round trip with the server at all!
    struct LocalState {
        game_scene: u16,
        view_flip: bool,
        selector_pos: u16,
        egghead_state: bool,
    } = {
        Self::new()
    }
}

impl LocalState {
    fn new() -> Self {
        Self {
            game_scene: 0,
            view_flip: true,
            selector_pos: 0,
            egghead_state: false
        }
    }
}

turbo::go! ({
    let mut local_state = LocalState::load();

    // get user id, for use across scenes
    let user_id = os::client::user_id().unwrap();

    match (local_state.game_scene, deserialize_player(&user_id), deserialize_factions()) {
        (_, None, _) => {
            os::client::exec("project_honkai", "create_player_data", "none".as_bytes());
        },
        (_, _, None) => {
            os::client::exec("project_honkai", "create_faction_data", "green".as_bytes());
            os::client::exec("project_honkai", "create_faction_data", "orange".as_bytes());
            os::client::exec("project_honkai", "create_faction_data", "purple".as_bytes());
        },
        (0, Some(player_state_deserialized), _) => {

            // *** UPDATE *** //

            if player_state_deserialized.faction != "none" {
                local_state.game_scene = 1;
            }

            if tick() % 16 == 0 {
                local_state.view_flip = !local_state.view_flip;
            }

            // *** DRAW *** //

            // background
            sprite!("background_layer", x = 0, y = 0);

            sprite!("outerframe_layer", x = 0, y = 0);

            sprite!("txt_select_your_suit", x = 62, y = 23);

            sprite!("choose_orange", x = 48, y = 51);
            sprite!("choose_green", x = 150, y = 51);
            sprite!("choose_purple", x = 252, y = 51);

            match local_state.selector_pos {
                0 => {
                    if local_state.view_flip {
                        sprite!("selected_orange_01", x = 48, y = 101);
                        sprite!("selected_overlay_01", x = 40, y = 43);
                    }
                    else {
                        sprite!("selected_orange_02", x = 48, y = 101);
                        sprite!("selected_overlay_02", x = 40, y = 43);
                    }
                },
                1 => {
                    if local_state.view_flip {
                        sprite!("selected_green_01", x = 150, y = 101);
                        sprite!("selected_overlay_01", x = 142, y = 43);
                    }
                    else {
                        sprite!("selected_green_02", x = 150, y = 101);
                        sprite!("selected_overlay_02", x = 142, y = 43);
                    }
                },
                2 => {
                    if local_state.view_flip {
                        sprite!("selected_purple_01", x = 252, y = 101);
                        sprite!("selected_overlay_01", x = 244, y = 43);
                    }
                    else {
                        sprite!("selected_purple_02", x = 252, y = 101);
                        sprite!("selected_overlay_02", x = 244, y = 43);
                    }
                },
                _ => ()
            }

            if local_state.view_flip {
                sprite!("arrow_left", x = 34, y = 22);
                sprite!("arrow_right", x = 327, y = 22);
            }
            else {
                sprite!("arrow_left", x = 32, y = 22);
                sprite!("arrow_right", x = 329, y = 22);
            }


            // *** INPUT *** //

            // THIS I NEED TO RECONSIDER COMPLETELY NOW THAT WE HAVE THE MOVING THING AND THE SELECT ON SPACE

            if gamepad(0).left.just_pressed() {
              // move selector left
                if local_state.selector_pos != 0
                {
                    local_state.selector_pos -= 1;
                }
            }
            else if gamepad(0).right.just_pressed() {
                // move selector right
                if local_state.selector_pos != 2
                {
                    local_state.selector_pos += 1;
                }
            }            
            else if gamepad(0).start.just_pressed() {
                match local_state.selector_pos {
                    0 => {
                        os::client::exec("project_honkai", "update_player_faction", "orange".as_bytes());
                    },
                    1 => {
                        os::client::exec("project_honkai", "update_player_faction", "green".as_bytes());
                    },
                    2 => {
                        os::client::exec("project_honkai", "update_player_faction", "purple".as_bytes());
                    },
                    _ => ()
                }
                local_state.game_scene = 1;
            }
        },
        (1, Some(player_state_deserialized), Some((green_faction_deserialized, orange_faction_deserialized, purple_faction_deserialized))) => {
            // this is the game scene
            // give me the factions

            // *** DRAW *** //

            // background
            sprite!("background_layer", x = 0, y = 0);
            sprite!("outerframe_layer", x = 0, y = 0);

            // ui
            sprite!("ui_faction_bar", x = 38, y = 21);
            sprite!("ui_xp_bar", x = 39, y = 65);
            // light sprite moving
            sprite!("lights_overlay", x = {98 + (tick() % 36 / 4) * 24}, y = 63);


            // draw correct UI based on player faction
            match player_state_deserialized.faction.as_str() {
                "green" => {
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
                "orange" => {
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
                "purple" => {
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
                _ => (),
            }

            // non faction specific draws
            // tier related (draw both the block and the question mark circle)
            match player_state_deserialized.current_tier {
                0 => {
                    sprite!("ui_tier_00_free", x = 42, y = 70);
                    sprite!("ui_qm_00_free", x = 304, y = 71);
                },
                1 => {
                    sprite!("ui_tier_01_basic", x = 42, y = 70);
                    sprite!("ui_qm_01_basic", x = 304, y = 71);
                },
                2 => {
                    sprite!("ui_tier_02_upgraded", x = 42, y = 70);
                    sprite!("ui_qm_02_upgraded", x = 304, y = 71);
                },
                3 => {
                    sprite!("ui_tier_03_premium", x = 42, y = 70);
                    sprite!("ui_qm_03_premium", x = 304, y = 71);
                },
                4 => {
                    sprite!("ui_tier_04_elite", x = 42, y = 68);
                    sprite!("ui_qm_04_elite", x = 304, y = 71);
                },
                5 => {
                    sprite!("ui_tier_05_ultimate", x = 42, y = 68);
                    sprite!("ui_qm_05_ultimate", x = 304, y = 71);
                },
                6 => {
                    sprite!("ui_tier_06_epic", x = 42, y = 68);
                    sprite!("ui_qm_06_epic", x = 304, y = 71);
                },
                7 => {
                    sprite!("ui_tier_07_legendary", x = 42, y = 68);
                    sprite!("ui_qm_07_legendary", x = 304, y = 71);
                },
                8 => {
                    sprite!("ui_tier_08_mythic", x = 42, y = 68);
                    sprite!("ui_qm_08_mythic", x = 304, y = 71);
                },
                9 => {
                    sprite!("ui_tier_09_derp", x = 42, y = 68);
                    sprite!("ui_qm_09_derp", x = 304, y = 71);
                },
                _ => (),
            }

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

            match local_state.egghead_state {
                true => {
                    // draw button pressed
                    sprite!("button_press", x = 209, y = 152);
                    // match the egghead based on player faction
                    match player_state_deserialized.faction.as_str() {
                        "green" => {
                            sprite!("egghead_green_press", x = 168, y = 121);
                        },
                        "orange" => {
                            sprite!("egghead_orange_press", x = 168, y = 121);
                        },
                        "purple" => {
                            sprite!("egghead_purple_press", x = 168, y = 121);
                        },
                        _ => ()
                    }
                },
                false => {
                    // draw button released
                    sprite!("button_release", x = 209, y = 152);
                    // draw right egg
                    match player_state_deserialized.faction.as_str() {
                        "green" => {
                            sprite!("egghead_green_release", x = 168, y = 121);
                        },
                        "orange" => {
                            sprite!("egghead_orange_release", x = 168, y = 121);
                        },
                        "purple" => {
                            sprite!("egghead_purple_release", x = 168, y = 121);
                        },
                        _ => ()
                    }
                },
            }

            // *** INPUT *** //

            if gamepad(0).start.just_pressed() {
                local_state.egghead_state = true;
                if player_state_deserialized.current_xp == player_state_deserialized.xp_needed_for_next_level - 1 {
                    os::client::exec("project_honkai", "increment_player_xp", &[]);
                    os::client::exec("project_honkai", "increment_faction_level", player_state_deserialized.faction.as_bytes());
                }
                else {
                    os::client::exec("project_honkai", "increment_player_xp", &[]);
                }
            }
            else if gamepad(0).start.just_released() {
                local_state.egghead_state = false;
            }
        },
        _ => ()
    }

    local_state.save();
});

// UTILITY FUNCS

fn deserialize_player(user_id: &str) -> Option<PlayerState>{
    // get the player state, or return early none if anything doesnt exist
    let player_file = os::client::read_file("project_honkai", &format!("players/{user_id}")).ok()?;
    let player_deserialized = states::PlayerState::try_from_slice(&player_file.contents).ok()?;
    Some(player_deserialized)
}

fn deserialize_factions() -> Option<(FactionState, FactionState, FactionState)> {
    // get the factions, or early return None if anything here doesnt exist - thats what the ? does
    // green
    let green_faction_file = os::client::read_file("project_honkai", "factions/green").ok()?;
    let green_faction_deserialized = states::FactionState::try_from_slice(&green_faction_file.contents).ok()?;
    // orange
    let orange_faction_file = os::client::read_file("project_honkai", "factions/orange").ok()?;
    let orange_faction_deserialized = states::FactionState::try_from_slice(&orange_faction_file.contents).ok()?;
    // purple
    let purple_faction_file = os::client::read_file("project_honkai", "factions/purple").ok()?;
    let purple_faction_deserialized = states::FactionState::try_from_slice(&purple_faction_file.contents).ok()?;
    // return
    Some((green_faction_deserialized, orange_faction_deserialized, purple_faction_deserialized))
}