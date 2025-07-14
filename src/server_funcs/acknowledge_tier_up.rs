use crate::*;

#[export_name = "turbo_program:command_handler/ggmvgxdev3/acknowledge_tier_up"]
unsafe extern "C" fn on_acknolwedge_tier_up() -> usize {
    // get the user id
    let user_id = os::server::command::user_id();

    // try to read player state data from file, which returns Result
    let read_result = os::server::fs::read_bytes(&format!("players/{user_id}"));
    if read_result.is_err() {
        // no read data?
        return os::server::command::CANCEL
    }

    let mut current_player_deserialized = states::PlayerState::try_from_slice(&read_result.unwrap()).unwrap();
    current_player_deserialized.did_accept_tier_up = true;

    // try write
    let write_result = os::server::fs::write_bytes(&format!("players/{user_id}"), 
        &borsh::to_vec(&current_player_deserialized).unwrap());
    if write_result.is_err() {
        // write error
        return os::server::command::CANCEL
    }

    os::server::command::COMMIT
}
