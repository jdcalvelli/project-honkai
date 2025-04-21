use crate::*;

pub fn deserialize_player(user_id: &str) -> Option<states::PlayerState>{
	// get the player state, or return early none if anything doesnt exist
	let player_file = os::client::watch_file(PROGRAM_ID, &format!("players/{user_id}")).data?;
	let player_deserialized = states::PlayerState::try_from_slice(&player_file.contents).ok()?;
	Some(player_deserialized)
}