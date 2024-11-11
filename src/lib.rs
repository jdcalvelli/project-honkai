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
    api-url = "http://localhost:8000"
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
    let this_user_id = os::client::user_id().unwrap();
    // get remote data result for this user, for use across scenes
    let this_player_remote_data = os::client::read_file("project_honkai", &format!("players/{this_user_id}"));

    // add local game scene check
    match local_state.game_scene {
        0 => {

            // intro scene, pick faction?

            // check if there is player data,
            match this_player_remote_data {
                Ok(_file) => {
                    // if so, advance to the next scene for now
                    local_state.game_scene = 1;

                },
                Err(_err) => {
                    // if not, we need to create a new player, then go to next scene
                    // determine faction based on key press for now, then make a file for player on server
                    // we need to make a player file for u on the server



                    // *** INPUT *** //

                    if gamepad(0).left.just_pressed() {
                        // create player of circle faction
                        os::client::exec("project_honkai", "create_player_data", "green".as_bytes());
                        // also create all the factions if not created
                        os::client::exec("project_honkai", "create_faction_data", "green".as_bytes());
                        os::client::exec("project_honkai", "create_faction_data", "orange".as_bytes());
                        os::client::exec("project_honkai", "create_faction_data", "purple".as_bytes());
                    }
                    else if gamepad(0).up.just_pressed() {
                        // create player of square faction
                        os::client::exec("project_honkai", "create_player_data", "orange".as_bytes());
                        // also create all the factions if not created
                        os::client::exec("project_honkai", "create_faction_data", "green".as_bytes());
                        os::client::exec("project_honkai", "create_faction_data", "orange".as_bytes());
                        os::client::exec("project_honkai", "create_faction_data", "purple".as_bytes());
                    }
                    else if gamepad(0).right.just_pressed() {
                        // create player of triangle faction
                        os::client::exec("project_honkai", "create_player_data", "purple".as_bytes());
                        // also create all the factions if not created
                        os::client::exec("project_honkai", "create_faction_data", "green".as_bytes());
                        os::client::exec("project_honkai", "create_faction_data", "orange".as_bytes());
                        os::client::exec("project_honkai", "create_faction_data", "purple".as_bytes());
                    }
                }
            }

        },
        1 => {

            // actual clicker game scene

            // INSIDE OF HERE, BASICALLY, IS POST US GETTING THE PLAYER
            match this_player_remote_data {
                Ok(player_file) => {



                    // *** UPDATE *** //

                    // deserialize player state
                    let player_state_deserialized = states::PlayerState::try_from_slice(&player_file.contents).unwrap();

                    // now we want to get all of the faction states
                    let green_faction_remote_data = os::client::read_file("project_honkai", "factions/green");
                    let green_faction_deserialized: states::FactionState;
                    match green_faction_remote_data {
                        Ok(file) => green_faction_deserialized = states::FactionState::try_from_slice(&file.contents).unwrap(),
                        Err(_) => return,
                    };
                    let orange_faction_remote_data = os::client::read_file("project_honkai", "factions/orange");
                    let orange_faction_deserialized: states::FactionState;
                    match orange_faction_remote_data {
                        Ok(file) => orange_faction_deserialized = states::FactionState::try_from_slice(&file.contents).unwrap(),
                        Err(_) => return,
                    };
                    let purple_faction_remote_data = os::client::read_file("project_honkai", "factions/purple");
                    let purple_faction_deserialized: states::FactionState;
                    match purple_faction_remote_data {
                        Ok(file) => purple_faction_deserialized = states::FactionState::try_from_slice(&file.contents).unwrap(),
                        Err(_) => return,
                    };
                    // this is for tier up immediacy, but i feel like this call should be elsewhere
                    if player_state_deserialized.current_level_in_tier == 10 && player_state_deserialized.current_tier != 9 {
                        os::client::exec("project_honkai", "tier_up_player", &[]);
                        // NEED TO FIGURE OUT WHAT HAPPENS WHEN SOMEONE GETS TO MAX TIER MAX LEVEL
                        // infinity symbol at the bottom lol
                    }



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
                        }
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
                        os::client::exec("project_honkai", "increment_player_xp", &[]);
                    }
                    else if gamepad(0).start.just_released() {
                        local_state.egghead_state = false;

                        if player_state_deserialized.current_xp == player_state_deserialized.xp_needed_for_next_level {
                            // also increment the faction score of the player!
                            os::client::exec("project_honkai", "increment_faction_level", player_state_deserialized.faction.as_bytes());
                            // level up the player
                            os::client::exec("project_honkai", "level_up_player", &[]);
                        }
                    }
                },
                Err(_err) => {
                    // at this point, there should be player data, if not we need to panic
                    panic!("PLAYER DATA SHOULD EXIST, BUT DOESN'T?")
                }
            }

        },
        _ => {

            panic!("SCENE OUT OF BOUNDS???");

        }
    }

    local_state.save();
});