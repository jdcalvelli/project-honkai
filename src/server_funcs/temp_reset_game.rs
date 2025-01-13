use crate::*;

#[export_name = "turbo/temp_reset_game"]
unsafe extern "C" fn on_temp_reset_game() -> usize {
	// just write empty vec to all the factions
	// this resets factions, but not players
	// need some way to reset players as a result of this
	let write_result = os::server::write_file("factions/green", &[]);

	if write_result.is_err() {
		return os::server::CANCEL
	}

	let write_result = os::server::write_file("factions/orange", &[]);
	if write_result.is_err() {
		return os::server::CANCEL
	}

	let write_result = os::server::write_file("factions/purple", &[]);
	if write_result.is_err() {
		return os::server::CANCEL
	}

	os::server::alert!("RESET");

	os::server::COMMIT
}