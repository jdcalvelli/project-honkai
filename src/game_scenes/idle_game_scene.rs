use crate::*;

pub fn update(local_state: &mut LocalState) -> () {
    if utils::deserialize_player(&local_state.user_id).is_some() {
        let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

        // read the player state to see if the bool is flipped basically
        if !player_state_deserialized.did_accept_level_up {
            local_state.game_scene = enums::GameScenes::LevelUpScene;
        } else {
            local_state.item_name = "".to_string();
            local_state.is_item_sound_selected = false;
            local_state.num_presses = 0;
        }
        // testing
        // log!("{:?}", player_state_deserialized.items);
    }
}

pub fn draw(local_state: &mut LocalState) -> () {
    if utils::deserialize_metastate().is_some()
        && utils::deserialize_factions().is_some()
        && utils::deserialize_player(&local_state.user_id).is_some()
    {
        let metastate_deserialized = utils::deserialize_metastate().unwrap();
        let (green_faction_deserialized, orange_faction_deserialized, purple_faction_deserialized) =
            utils::deserialize_factions().unwrap();
        let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

        // *** DRAW *** //

        // ui
        sprite!("ui_faction_bar", x = 38, y = 21);
        sprite!("ui_xp_bar", x = 39, y = 65);
        // light sprite moving
        sprite!(
            "lights_overlay",
            x = { 98 + (time::tick() % 36 / 4) * 24 },
            y = 63
        );

        // draw correct button based on winning faction
        let t = (time::tick() / 8) as f32;
        let s = 2. * t.sin();
        match metastate_deserialized.last_faction_win {
            enums::Factions::Green => sprite!("green_badge", x = 326, y = 29, rotation = 5. * s),
            enums::Factions::Purple => sprite!("purple_badge", x = 326, y = 29, rotation = 5. * s),
            enums::Factions::Orange => sprite!("orange_badge", x = 326, y = 29, rotation = 5. * s),
            enums::Factions::NoFaction => (),
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
                    color = 0x49aa10ff
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
                    color = 0x9241f3ff
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
                    color = 0xc37100ff
                );

                // green faction card
                sprite!("ui_faction_profile_green", x = 272, y = 121);
            }
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
                    color = 0xc37100ff
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
                    color = 0x49aa10ff
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
                    color = 0x9241f3ff
                );

                // orange faction card
                sprite!("ui_faction_profile_orange", x = 272, y = 121);
            }
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
                    color = 0x9241f3ff
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
                    color = 0xc37100ff
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
                    color = 0x49aa10ff
                );

                // purple faction card
                sprite!("ui_faction_profile_purple", x = 272, y = 121);
            }
            enums::Factions::NoFaction => (),
        }

        text!(
            "TOTAL",
            x = 314,
            y = 142,
            color = 0x000000ff,
            font = "small"
        );
        text!("SUIT", x = 314, y = 148, color = 0x000000ff, font = "small");
        text!(
            "WINS:",
            x = 314,
            y = 154,
            color = 0x000000ff,
            font = "small"
        );

        match player_state_deserialized.faction {
            enums::Factions::Green => {
                let hold = &format!("{}", metastate_deserialized.green_total_wins);
                text!(hold, x = 314, y = 160, color = 0x000000ff, font = "medium");
            }
            enums::Factions::Orange => {
                let hold = &format!("{}", metastate_deserialized.orange_total_wins);
                text!(hold, x = 314, y = 160, color = 0x000000ff, font = "medium");
            }
            enums::Factions::Purple => {
                let hold = &format!("{}", metastate_deserialized.purple_total_wins);
                text!(hold, x = 314, y = 160, color = 0x000000ff, font = "medium");
            }
            enums::Factions::NoFaction => (),
        }

        // non faction specific draws
        // tier related (draw both the block and the question mark circle)
        let hold = &format!("ui_tier_{}", player_state_deserialized.current_tier);
        sprite!(hold, x = 39, y = 67);
        let hold = &format!("ui_qm_{}", player_state_deserialized.current_tier);
        sprite!(hold, x = 304, y = 71);

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
        // NOW THE ITEM SPRITE!
        let t = (time::tick() / 8) as f32;
        let s = 2. * t.sin();
        if player_state_deserialized.did_accept_level_up {
            match player_state_deserialized.items[0].item_type {
                enums::ItemTypes::NoItem => (),
                enums::ItemTypes::Stapler => sprite!(
                    "item_stapler",
                    x = 83.,
                    y = 149. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::BendedFolder => sprite!(
                    "item_bended_folder",
                    x = 84.,
                    y = 142. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::YogurtCup => sprite!(
                    "item_yogurt",
                    x = 87.,
                    y = 144. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::UsedNapkins => sprite!(
                    "item_used_napkin",
                    x = 85.,
                    y = 145. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::Eggs => sprite!(
                    "item_eggs",
                    x = 84.,
                    y = 146. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::Books => sprite!(
                    "item_books",
                    x = 83.,
                    y = 135. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::Box => sprite!(
                    "item_box",
                    x = 83.,
                    y = 144. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::Pantaloons => sprite!(
                    "item_pantaloons",
                    x = 82.,
                    y = 137. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::PuzzlePiece => sprite!(
                    "item_puzzle_piece",
                    x = 83.,
                    y = 143. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::BowlingBall => sprite!(
                    "item_bowling_ball",
                    x = 84.,
                    y = 138. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::GOGOBucks => sprite!(
                    "item_go_go_bucks",
                    x = 81.,
                    y = 143. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::CreditCard => sprite!(
                    "item_credit_card",
                    x = 84.,
                    y = 143. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::GOGOCard => sprite!(
                    "item_go_go_card",
                    x = 84.,
                    y = 143. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::GOGOSticker => sprite!(
                    "item_go_go_sticker",
                    x = 79.,
                    y = 133. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::InGameTokens => sprite!(
                    "item_in_game_token",
                    x = 88.,
                    y = 146. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::Sock => sprite!(
                    "item_sock",
                    x = 85.,
                    y = 139. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::BakedBeans => sprite!(
                    "item_baked_beans",
                    x = 85.,
                    y = 136. + s,
                    color = player_state_deserialized.items[0].color
                ),
                enums::ItemTypes::Cube => sprite!(
                    "item_cube",
                    x = 83.,
                    y = 140. + s,
                    color = player_state_deserialized.items[0].color
                ),
            }
        }

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
                    }
                    enums::Factions::Orange => {
                        sprite!("egghead_orange_press", x = 168, y = 121);
                        if metastate_deserialized.last_faction_win == enums::Factions::Orange {
                            // CROWN
                            sprite!("jester_hat_02", x = 160, y = 107);
                        }
                    }
                    enums::Factions::Purple => {
                        sprite!("egghead_purple_press", x = 168, y = 121);
                        if metastate_deserialized.last_faction_win == enums::Factions::Purple {
                            // CROWN
                            sprite!("jester_hat_02", x = 160, y = 107);
                        }
                    }
                    enums::Factions::NoFaction => (),
                }
            }
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
                    }
                    enums::Factions::Orange => {
                        sprite!("egghead_orange_release", x = 168, y = 121);
                        if metastate_deserialized.last_faction_win == enums::Factions::Orange {
                            // CROWN
                            sprite!("jester_hat_01", x = 161, y = 107);
                        }
                    }
                    enums::Factions::Purple => {
                        sprite!("egghead_purple_release", x = 168, y = 121);
                        if metastate_deserialized.last_faction_win == enums::Factions::Purple {
                            // CROWN
                            sprite!("jester_hat_01", x = 161, y = 107);
                        }
                    }
                    enums::Factions::NoFaction => (),
                }
            }
        }
    }
}

pub fn input(local_state: &mut LocalState) -> () {
    if utils::deserialize_player(&local_state.user_id).is_some()
        && utils::deserialize_factions().is_some()
        && utils::deserialize_metastate().is_some()
    {
        let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

        // *** INPUT *** //

        if pointer::screen().just_pressed() || gamepad::get(0).start.just_pressed() {
            audio::play("button_hit");
            local_state.egghead_state = true;
            if player_state_deserialized.current_xp
                == player_state_deserialized.xp_needed_for_next_level - 1
            {
                os::client::command::exec_raw(PROGRAM_ID, "increment_player_xp", &[]);
                os::client::command::exec_raw(
                    PROGRAM_ID,
                    "increment_faction_level",
                    &borsh::to_vec(&player_state_deserialized.faction).unwrap(),
                );
            } else {
                os::client::command::exec_raw(PROGRAM_ID, "increment_player_xp", &[]);
            }
        } else if pointer::screen().just_released() || gamepad::get(0).start.just_released() {
            audio::play("button_release");
            local_state.egghead_state = false;
        }
    }
}
