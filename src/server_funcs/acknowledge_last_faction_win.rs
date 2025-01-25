use crate::*;

#[export_name = "turbo/acknowledge_last_faction_winner"]
unsafe extern "C" fn on_acknowledge_last_faction_winner() -> usize {
    // get the user id
    let user_id = os::server::get_user_id();

    // try to read player state data from file, which returns Result
    let read_result = os::server::read_file(&format!("players/{user_id}"));
    if read_result.is_err() {
        // no read data?
        return os::server::CANCEL
    }
    
    let mut current_player_deserialized = states::PlayerState::try_from_slice(&read_result.unwrap()).unwrap();
    current_player_deserialized.did_accept_last_faction_winner = true;
    
    // try write
    let write_result = os::server::write_file(&format!("players/{user_id}"), 
        &current_player_deserialized.try_to_vec().unwrap());
    if write_result.is_err() {
        // write error
        return os::server::CANCEL
    }
    
    os::server::COMMIT
}
