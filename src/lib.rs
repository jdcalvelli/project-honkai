use states::FactionState;

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
        egghead_state: bool,
    } = {
        Self::new()
    }
}

impl LocalState {
    fn new() -> Self {
        Self {
            game_scene: 0,
            egghead_state: false
        }
    }
}

turbo::go! ({
    let mut local_state = LocalState::load();

    // get user id, for use across scenes
    let user_id = os::client::user_id().unwrap();
    // get remote data result for this user, for use across scenes
    let player_file_option = os::client::read_file("project_honkai", &format!("players/{user_id}")).ok();

    // LOGIC REWRITE TO FIX UNWRAP PROBLEM

    // if there is no file
    if player_file_option.is_none()
    {
        // create the player file with no faction
        // should make an enum to not pass strings around ultimately
        os::client::exec("project_honkai", "create_player_data", "none".as_bytes());

        // create the factions if they aren't already created
        os::client::exec("project_honkai", "create_faction_data", "green".as_bytes());
        os::client::exec("project_honkai", "create_faction_data", "orange".as_bytes());
        os::client::exec("project_honkai", "create_faction_data", "purple".as_bytes());

        // early return
        local_state.save();
        return;
    }

    // if there is a file, then we want to deserialize that file into its struct
    let player_state_result = states::PlayerState::try_from_slice(&player_file_option.unwrap().contents).ok();

    // now there is a file, but the faction is none
    // so now we want to match on the game scene and the deserialized version of the player file

    match (local_state.game_scene, player_state_result) {
        (0, Some(_player_state_deserialized)) => {


            text!("FACTION PICK SCENE", x = 50, y = 50);
            // this is the pick a faction scene

            // right now its just black and you have to select the right thing

            // *** INPUT *** //

            if gamepad(0).left.just_pressed() {
                // UPDATE CHARACTER FACTION SERVER COMMAND
                os::client::exec("project_honkai", "update_player_faction", "green".as_bytes());
                local_state.game_scene = 1;
            }
            else if gamepad(0).up.just_pressed() {
                // UPDATE CHARACTER FACTION SERVER COMMAND
                os::client::exec("project_honkai", "update_player_faction", "orange".as_bytes());
                local_state.game_scene = 1;
            }
            else if gamepad(0).right.just_pressed() {
                // UPDATE CHARACTER FACTION SERVER COMMAND
                os::client::exec("project_honkai", "update_player_faction", "purple".as_bytes());
                local_state.game_scene = 1;
            }

            local_state.save();
        },
        (1, Some(player_state_deserialized)) => {
            // this is the game scene
            // give me the factions

            match deserialize_factions() {
                Some((green_faction_deserialized, orange_faction_deserialized, purple_faction_deserialized)) => {

                    // *** DRAW *** //

                    // background
                    sprite!("background_layer", x = 0, y = 0);

                    // ui
                    sprite!("ui_faction_bar", x = 38, y = 21);
                    sprite!("ui_xp_bar", x = 39, y = 65);

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
                                color = 0x008000ff
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
                        "none" => {
                            // early return
                            local_state.save();
                            return;
                        },
                        _ => {
                            panic!("a faction that doesn't exist?");
                        },
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
                        _ => panic!("cant have tier higher than 9"),
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
                                _ => {
                                    panic!("incorrect faction state found");
                                }
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
                                _ => {
                                    panic!("incorrect faction state found");
                                }
                            }
                        },
                    }

                    // very foreground
                    sprite!("outerframe_layer", x = 0, y = 0);

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

                    local_state.save();
                },
                None => {

                    // THIS IS THE CASE WHERE THERE IS A NONE IN THE FACTION STATES
                    local_state.save();
                    return;
                }
            }
        },
        _ => {
            // THIS IS WHERE IF THE SCENE IS NOT 0 OR 1, AND THE PLAYER STATE CAN'T BE DESERIALIZED 
            local_state.save();
            return;
        }
    }
});

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