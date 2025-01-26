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

    let t1_names = vec!["Free", "Plain", "Flimsy", "Stinky", "Old", "Cheap"];
    let t2_names = vec!["Basic", "Firm", "Solid", "Functional", "Adequate", "Unoriginal", "Free", "Plain", "Flimsy", "Old"];
    let t3_names = vec!["Upgraded", "Uncommon", "New", "Resold", "Cool", "Satisfactory", "Firm", "Solid", "Functional", "Adequate"];
    let t4_names = vec!["Premium", "Expensive", "Rare", "Flashy", "Vibrant", "Stylish", "New", "Resold", "Cool", "Functional", "Uncommon", "Satisfactory"];
    let t5_names = vec!["Elite", "Striking", "Lavish", "Limited", "Insured", "Exclusive", "Expensive", "Rare", "Flashy", "Vibrant", "Stylish", "Resold", "New"];
    let t6_names = vec!["Ultimate", "Jumbo", "Mega", "Super", "Ultra", "Permanent", "Lavish", "Limited", "Insured", "Exclusive", "Rare", "New", "Vibrant"];
    let t7_names = vec!["Epic", "Coveted", "Humongous", "Grand", "Whopping", "Dynamic", "Jumbo", "Mega", "Super", "Ultra", "Permanent", "Limited", "Insured", "Exclusive", "Rare"];
    let t8_names = vec!["Legendary", "Colossal", "Herculean", "Renowned", "Exalted", "Sublime", "Coveted", "Humongous", "Grand", "Whopping", "Dynamic", "Jumbo", "Mega", "Super", "Ultra", "Insured", "Limited", "Rare"];
    let t9_names = vec!["Mythic", "Astronomic", "Heroic", "Fabled", "Apocryphal", "Folkloric", "Colossal", "Herculean", "Renowned", "Exalted", "Sublime", "Coveted", "Humongous", "Grand", "Jumbo", "Mega", "Super", "Ultra", "Insured", "Limited", "Rare"];
    let t10_names = vec!["Nonfungible", "Nonfungible", "Nonfungible", "Nonfungible", "Nonfungible", "Nonfungible", "Astronomic", "Heroic", "Fabled", "Apocryphal", "Folkloric", "Colossal", "Herculean", "Renowned", "Exalted", "Jumbo", "Mega", "Coveted", "Sublime"];

    if local_state.has_broken_level_up {
        sprite!("level_up_broken", x = 79, y = 20);
    }
    else {
        sprite!("level_up", x = 79, y = 20);
    }

    text!(&format!("{}", player_state_deserialized.current_level_in_tier), x = 189, y = 24, color = 0x000000ff);

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

    // need to pick number based on current tier
    // for item in range up to current player tier plus one create an item string and pick and concat
    // take the item string and pass it into text
    if local_state.item_name == "" {
        match player_state_deserialized.current_tier {
            0 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t1_names.len() as u32;
                    local_state.item_name.push_str(t1_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            1 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t2_names.len() as u32;
                    local_state.item_name.push_str(t2_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            2 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t3_names.len() as u32;
                    local_state.item_name.push_str(t3_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            3 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t4_names.len() as u32;
                    local_state.item_name.push_str(t4_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            4 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t5_names.len() as u32;
                    local_state.item_name.push_str(t5_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            5 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t6_names.len() as u32;
                    local_state.item_name.push_str(t6_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            6 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t7_names.len() as u32;
                    local_state.item_name.push_str(t7_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            7 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t8_names.len() as u32;
                    local_state.item_name.push_str(t8_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            8 => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t9_names.len() as u32;
                    local_state.item_name.push_str(t9_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            },
            _ => {
                for _ in 0..player_state_deserialized.current_tier + 1 {
                    let rand_num = rand() % t10_names.len() as u32;
                    local_state.item_name.push_str(t10_names[rand_num as usize]);
                    local_state.item_name.push_str(" ");
                }
            }
        }
        local_state.item_name.push_str(&player_state_deserialized.items[0].item_type.to_string());
    }
    log!("{:?}", local_state.item_name);

    text!(local_state.item_name.as_str(), x = 83, y = 132, color = 0x000000ff, font = Font::M);

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