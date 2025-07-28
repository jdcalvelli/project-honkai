use crate::*;

#[export_name = "turbo_program:command_handler/ggmvgxdev4/increment_player_xp"]
unsafe extern "C" fn on_increment_player_xp() -> usize {
    // get the user id
    let user_id = os::server::command::user_id();

    // try to read player state data from file, which returns Result
    let read_result = os::server::fs::read_bytes(&format!("players/{user_id}"));
    if read_result.is_err() {
        // read error
        return os::server::command::CANCEL;
    }

    let mut current_player_deserialized =
        states::PlayerState::try_from_slice(&read_result.unwrap()).unwrap();
    current_player_deserialized.current_xp += 1;
    // if players xp is high enough, level up
    if current_player_deserialized.current_xp
        == current_player_deserialized.xp_needed_for_next_level
    {
        // first we increase player level!
        current_player_deserialized.current_level_in_tier += 1;
        // then we need to set the prev needed level xp to whatever currently next needed level xp is
        current_player_deserialized.xp_needed_for_prev_level =
            current_player_deserialized.xp_needed_for_next_level;
        // then we calculate what the next level xp needs to be
        // currently, the equation is new level to the 2th power + 5?
        current_player_deserialized.xp_needed_for_next_level =
            (current_player_deserialized.current_level_in_tier + 1).pow(2) + 5;

        current_player_deserialized.did_accept_level_up = false;

        // need to get a random type from the item_types
        let mut rand_num: u8 = random::u8();
        rand_num = rand_num % 19;
        let rand_item_type: enums::ItemTypes = unsafe { std::mem::transmute(rand_num) };
        // generate a random item
        // STILL NEED TO DO RANDOM COLOR
        let color_options: Vec<u32> = vec![
            0x00A5E3ff, 0x8DD7BFff, 0xFF96C5ff, 0xFF5768ff, 0xFFBF65ff, 0xFC6238ff, 0xFFD872ff,
            0xF2D4CCff, 0xE77577ff, 0x6C88C4ff, 0xC05780ff, 0xFF828Bff, 0xE7C582ff, 0x00B0BAff,
            0x0065A2ff, 0x00CDACff, 0xFFDACCff, 0xCFF800ff, 0xFF5C77ff, 0x4DD091ff, 0xFFEC59ff,
            0xFFA23Aff,
        ];
        let mut rand_num: usize = random::u64() as usize;
        rand_num = rand_num % color_options.len();
        let player_item = structs::Item::new(rand_item_type, color_options[rand_num]);
        current_player_deserialized.items.insert(0, player_item);
    }
    // if the players level is high enough to tier up, also tier up the player
    if current_player_deserialized.current_level_in_tier == 10
        && current_player_deserialized.current_tier != 9
    {
        // this is the tier up, so we need to increment tier, and return all else to zero?
        current_player_deserialized.current_tier += 1;
        current_player_deserialized.current_level_in_tier = 0;
        current_player_deserialized.current_xp = 0;
        current_player_deserialized.xp_needed_for_prev_level = 0;
        // this 6 value is based on the current equation which can be found in player_state.rs
        // assumption rn is that it will be the same xp curve regardless of tier
        current_player_deserialized.xp_needed_for_next_level = 3;

        current_player_deserialized.did_accept_tier_up = false;
    }

    // try write
    let write_result = os::server::fs::write_bytes(
        &format!("players/{user_id}"),
        &borsh::to_vec(&current_player_deserialized).unwrap(),
    );
    if write_result.is_err() {
        // write error
        return os::server::command::CANCEL;
    }

    os::server::command::COMMIT
}
