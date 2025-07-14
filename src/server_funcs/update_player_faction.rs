use crate::*;

#[export_name = "turbo_program:command_handler/ggmvgxdev3/update_player_faction"]
unsafe extern "C" fn on_update_player_faction() -> usize {
    // get the user id
    let user_id = os::server::command::user_id();
    // get the function argument
    // could be here too
    let function_arg_deserialized = os::server::command::parse_input::<enums::Factions>().unwrap();
    if function_arg_deserialized == enums::Factions::NoFaction {
        // you cant pick to be in no faction
        return os::server::command::CANCEL
    }
    
    // try to read player file
    let read_result = os::server::fs::read_bytes(&format!("players/{user_id}"));
    if read_result.is_err() {
        // if there is no file, something went wrong so cancel
        return os::server::command::CANCEL
    }
    
    let mut current_player_deserialized = states::PlayerState::try_from_slice(&read_result.unwrap()).unwrap();
    current_player_deserialized.faction = function_arg_deserialized;
    // write the change
    let write_result = os::server::fs::write_bytes(&format!("players/{user_id}"), 
        &borsh::to_vec(&current_player_deserialized).unwrap());
        
    if write_result.is_err() {
        // write error
        return os::server::command::CANCEL
    }
    
    return os::server::command::COMMIT
}
