use crate::*;

#[export_name = "turbo/update_player_faction"]
unsafe extern "C" fn on_update_player_faction() -> usize {
    // get the function arg
    let function_arg = os::server::get_command_data();
    let function_arg_as_string = String::from_utf8(function_arg);

    // get the user id
    let user_id = os::server::get_user_id();

    // try to read player state data from file, which returns Result
    let read_result = os::server::read_file(&format!("players/{user_id}"));

    // check based on result what we should do next
    match read_result {
        Ok(data) => {
            // if data exists, deserialize the struct and set holder to it
            let mut current_player_deserialized = states::PlayerState::try_from_slice(&data).unwrap();

            // change the faction to whatever is passed in (SHOULD NOT UNWRAP)
            current_player_deserialized.faction = function_arg_as_string.unwrap();

            // write the data to the file
            let write_result = os::server::write_file(&format!("players/{user_id}"), 
                &current_player_deserialized.try_to_vec().unwrap());

            match write_result {
                Ok(_) => {
                    // commit the change if theres no issue writing data
                    os::server::COMMIT
                }
                Err(err) => {
                    // cancel the change if there is an error in writing for some reason
                    os::server::log(&err.to_string());
                    os::server::CANCEL
                }
            }
        },
        Err(_err) => {
            // if there is no read data, cancel the execution
            os::server::CANCEL

        },
    }
}
