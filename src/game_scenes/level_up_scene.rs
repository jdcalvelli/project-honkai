use crate::*;

pub fn update(local_state: &mut LocalState) -> () {
    if utils::deserialize_player(&local_state.user_id).is_some() {
        let player_state_deserialized = utils::deserialize_player(&local_state.user_id).unwrap();

        // *** UPDATE *** //

        if player_state_deserialized.did_accept_level_up
            && !player_state_deserialized.did_accept_tier_up
        {
            local_state.game_scene = enums::GameScenes::TierUpScene;
        } else if player_state_deserialized.did_accept_level_up {
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

        let t1_names = vec!["Free", "Plain", "Flimsy", "Stinky", "Old", "Cheap"];
        let t2_names = vec![
            "Basic",
            "Firm",
            "Solid",
            "Functional",
            "Adequate",
            "Unoriginal",
            "Free",
            "Plain",
            "Flimsy",
            "Old",
        ];
        let t3_names = vec![
            "Upgraded",
            "Uncommon",
            "New",
            "Resold",
            "Cool",
            "Satisfactory",
            "Firm",
            "Solid",
            "Functional",
            "Adequate",
        ];
        let t4_names = vec![
            "Premium",
            "Expensive",
            "Rare",
            "Flashy",
            "Vibrant",
            "Stylish",
            "New",
            "Resold",
            "Cool",
            "Functional",
            "Uncommon",
            "Satisfactory",
        ];
        let t5_names = vec![
            "Elite",
            "Striking",
            "Lavish",
            "Limited",
            "Insured",
            "Exclusive",
            "Expensive",
            "Rare",
            "Flashy",
            "Vibrant",
            "Stylish",
            "Resold",
            "New",
        ];
        let t6_names = vec![
            "Ultimate",
            "Jumbo",
            "Mega",
            "Super",
            "Ultra",
            "Permanent",
            "Lavish",
            "Limited",
            "Insured",
            "Exclusive",
            "Rare",
            "New",
            "Vibrant",
        ];
        let t7_names = vec![
            "Epic",
            "Coveted",
            "Humongous",
            "Grand",
            "Whopping",
            "Dynamic",
            "Jumbo",
            "Mega",
            "Super",
            "Ultra",
            "Permanent",
            "Limited",
            "Insured",
            "Exclusive",
            "Rare",
        ];
        let t8_names = vec![
            "Legendary",
            "Colossal",
            "Herculean",
            "Renowned",
            "Exalted",
            "Sublime",
            "Coveted",
            "Humongous",
            "Grand",
            "Whopping",
            "Dynamic",
            "Jumbo",
            "Mega",
            "Super",
            "Ultra",
            "Insured",
            "Limited",
            "Rare",
        ];
        let t9_names = vec![
            "Mythic",
            "Astronomic",
            "Heroic",
            "Fabled",
            "Apocryphal",
            "Folkloric",
            "Colossal",
            "Herculean",
            "Renowned",
            "Exalted",
            "Sublime",
            "Coveted",
            "Humongous",
            "Grand",
            "Jumbo",
            "Mega",
            "Super",
            "Ultra",
            "Insured",
            "Limited",
            "Rare",
        ];
        let t10_names = vec![
            "Nonfungible",
            "Nonfungible",
            "Nonfungible",
            "Nonfungible",
            "Nonfungible",
            "Nonfungible",
            "Astronomic",
            "Heroic",
            "Fabled",
            "Apocryphal",
            "Folkloric",
            "Colossal",
            "Herculean",
            "Renowned",
            "Exalted",
            "Jumbo",
            "Mega",
            "Coveted",
            "Sublime",
        ];

        if local_state.has_broken_level_up {
            sprite!("level_up_broken", x = 79, y = 20);
        } else {
            sprite!("level_up", x = 79, y = 20);
        }

        match player_state_deserialized.items[0].item_type {
            enums::ItemTypes::NoItem => (),
            enums::ItemTypes::Stapler => sprite!(
                "item_stapler",
                x = 180,
                y = 92,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::BendedFolder => sprite!(
                "item_bended_folder",
                x = 181,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::YogurtCup => sprite!(
                "item_yogurt",
                x = 183,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::UsedNapkins => sprite!(
                "item_used_napkin",
                x = 182,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::Eggs => sprite!(
                "item_eggs",
                x = 180,
                y = 89,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::Books => sprite!(
                "item_books",
                x = 180,
                y = 82,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::Box => sprite!(
                "item_box",
                x = 180,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::Pantaloons => sprite!(
                "item_pantaloons",
                x = 179,
                y = 85,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::PuzzlePiece => sprite!(
                "item_puzzle_piece",
                x = 181,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::BowlingBall => sprite!(
                "item_bowling_ball",
                x = 181,
                y = 85,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::GOGOBucks => sprite!(
                "item_go_go_bucks",
                x = 179,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::CreditCard => sprite!(
                "item_credit_card",
                x = 182,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::GOGOCard => sprite!(
                "item_go_go_card",
                x = 182,
                y = 88,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::GOGOSticker => sprite!(
                "item_go_go_sticker",
                x = 176,
                y = 82,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::InGameTokens => sprite!(
                "item_in_game_token",
                x = 185,
                y = 89,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::Sock => sprite!(
                "item_sock",
                x = 183,
                y = 84,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::BakedBeans => sprite!(
                "item_baked_beans",
                x = 182,
                y = 84,
                color = player_state_deserialized.items[0].color
            ),
            enums::ItemTypes::Cube => sprite!(
                "item_cube",
                x = 181,
                y = 85,
                color = player_state_deserialized.items[0].color
            ),
        }

        // need to pick number based on current tier
        // for item in range up to current player tier plus one create an item string and pick and concat
        // take the item string and pass it into text
        if local_state.item_name == "" {
            match player_state_deserialized.current_tier {
                0 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t1_names.len() as u32;
                        local_state.item_name.push_str(t1_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                1 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t2_names.len() as u32;
                        local_state.item_name.push_str(t2_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                2 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t3_names.len() as u32;
                        local_state.item_name.push_str(t3_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                3 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t4_names.len() as u32;
                        local_state.item_name.push_str(t4_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                4 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t5_names.len() as u32;
                        local_state.item_name.push_str(t5_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                5 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t6_names.len() as u32;
                        local_state.item_name.push_str(t6_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                6 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t7_names.len() as u32;
                        local_state.item_name.push_str(t7_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                7 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t8_names.len() as u32;
                        local_state.item_name.push_str(t8_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                8 => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t9_names.len() as u32;
                        local_state.item_name.push_str(t9_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
                _ => {
                    for _ in 0..player_state_deserialized.current_tier + 1 {
                        let rand_num = random::u32() % t10_names.len() as u32;
                        local_state.item_name.push_str(t10_names[rand_num as usize]);
                        local_state.item_name.push_str(" ");
                    }
                }
            }
            local_state
                .item_name
                .push_str(&player_state_deserialized.items[0].item_type.to_string());
        }
        // log!("{:?}", local_state.item_name);

        text!(
            local_state.item_name.as_str(),
            x = 83,
            y = 132,
            color = 0x000000ff,
            font = "medium"
        );

        if local_state.view_flip {
            sprite!("red_claim_01", x = 150, y = 146);
        } else {
            sprite!("red_claim_02", x = 150, y = 146);
        }

        rect!(
            x = 150,
            y = 172,
            w = (86 / 4) * (local_state.num_presses % 5),
            h = 1,
            color = 0xffd700ff
        );

        // item sounds array
        let item_sounds = [
            "item_shine_01",
            "item_shine_02",
            "item_shine_03",
            "item_shine_04",
            "item_shine_05",
        ];
        let mut item_sound_to_play = "";
        if !local_state.is_item_sound_selected {
            let rand_num = random::u32() as usize % item_sounds.len();
            item_sound_to_play = item_sounds[rand_num];
            local_state.is_item_sound_selected = true;
        }

        if !audio::is_playing(item_sound_to_play) {
            audio::play(item_sound_to_play);
        }

        if !audio::is_playing("level_up") {
            audio::play("level_up");
        }
    }
}

pub fn input(local_state: &mut LocalState) -> () {
    if utils::deserialize_player(&local_state.user_id).is_none() {
        return;
    }
    if pointer::screen().just_pressed() || gamepad::get(0).start.just_pressed() {
        audio::play("button_hit");
        local_state.egghead_state = true;
        // now i need a transaction to set flag back
        if local_state.num_presses == 4 {
            os::client::command::exec_raw(PROGRAM_ID, "acknowledge_level_up", &[]);
        } else {
            local_state.num_presses += 1;
        }
    } else if pointer::screen().just_released() || gamepad::get(0).start.just_released() {
        audio::play("button_release");
        local_state.egghead_state = false;
    }
}
