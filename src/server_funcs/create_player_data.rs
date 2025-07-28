use crate::*;

#[export_name = "turbo_program:command_handler/ggmvgxdev4/create_player_data"]
unsafe extern "C" fn on_create_player_data() -> usize {
    // get the user id
    let user_id = os::server::command::user_id();

    // when i create a player i need to add them to the meta list
    let read_result = os::server::fs::read_bytes("metastate");
    if read_result.is_err() {
        return os::server::command::CANCEL;
    }
    let mut current_meta_state_deserialized =
        states::MetaState::try_from_slice(&read_result.unwrap()).unwrap();

    if !current_meta_state_deserialized
        .player_list
        .contains(&user_id)
    {
        current_meta_state_deserialized
            .player_list
            .push(user_id.clone());
    }

    let write_result = os::server::fs::write_bytes(
        "metastate",
        &borsh::to_vec(&current_meta_state_deserialized).unwrap(),
    );
    if write_result.is_err() {
        return os::server::command::CANCEL;
    }

    // get the function data deserialized
    // ANOTHER POSSIBLE PROBLEM
    let function_input_deserialized =
        os::server::command::parse_input::<enums::Factions>().unwrap();
    if function_input_deserialized != enums::Factions::NoFaction {
        // if the wrong input came in, cancel
        return os::server::command::CANCEL;
    }

    // below is what i had when i cared if data was already there lol
    // try to read player state data from file, which returns Result
    let read_result = os::server::fs::read_bytes(&format!("players/{user_id}"));
    if read_result.is_ok() {
        // if there is data there already, then cancel info creation
        return os::server::command::CANCEL;
    }

    // there is not currently a player file, lets create one
    let mut current_player_deserialized: states::PlayerState = states::PlayerState::new();
    // set the faction to be what i want, which is no faction at this point
    current_player_deserialized.faction = function_input_deserialized;
    // write the file
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
