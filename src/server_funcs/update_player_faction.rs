use crate::*;

#[export_name = "turbo/update_player_faction"]
unsafe extern "C" fn on_update_player_faction() -> usize {
    // get the user id
    let user_id = os::server::get_user_id();
    // get the function argument
    let function_arg_deserialized = os::server::command!(enums::Factions);
    if function_arg_deserialized == enums::Factions::NoFaction {
        // you cant pick to be in no faction
        return os::server::CANCEL
    }
    
    // try to read player file
    let read_result = os::server::read_file(&format!("players/{user_id}"));
    if read_result.is_err() {
        // if there is no file, something went wrong so cancel
        return os::server::CANCEL
    }
    
    let mut current_player_deserialized = states::PlayerState::try_from_slice(&read_result.unwrap()).unwrap();
    current_player_deserialized.faction = function_arg_deserialized;
    // write the change
    let write_result = os::server::write_file(&format!("players/{user_id}"), 
        &current_player_deserialized.try_to_vec().unwrap());
        
    if write_result.is_err() {
        // write error
        return os::server::CANCEL
    }
    
    return os::server::COMMIT
}
