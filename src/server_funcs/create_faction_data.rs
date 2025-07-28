use crate::*;

// this will get passed through the name of the faction to create

#[export_name = "turbo_program:command_handler/ggmvgxdev4/create_faction_data"]
unsafe extern "C" fn on_create_faction_data() -> usize {
    // get the function data deserialized
    // COME BACK TO THIS NNOTENONOENOENTOENTOE
    let function_data_deserialized = os::server::command::parse_input::<enums::Factions>().unwrap();

    match function_data_deserialized {
        enums::Factions::NoFaction => {
            // error, we dont want to write a file for no faction
            os::server::command::CANCEL
        }
        enums::Factions::Green => write_faction_file_if_not_there("green"),
        enums::Factions::Purple => write_faction_file_if_not_there("orange"),
        enums::Factions::Orange => write_faction_file_if_not_there("purple"),
    }
}

fn write_faction_file_if_not_there(faction_name: &str) -> usize {
    // check if the data exists
    let read_result = os::server::fs::read_bytes(&format!("factions/{faction_name}"));
    if read_result.is_ok() {
        // data already exists
        if !read_result.unwrap().is_empty() {
            // if not empty, then cancel
            return os::server::command::CANCEL;
        }
        // if it is empty we need to keep going
    }
    // data doesnt exist so write it
    let write_result = os::server::fs::write_bytes(
        &format!("factions/{faction_name}"),
        &borsh::to_vec(&states::FactionState::new()).unwrap(),
    );
    if write_result.is_err() {
        // write error
        return os::server::command::CANCEL;
    }

    os::server::command::COMMIT
}
