use crate::*;

pub fn deserialize_factions() -> Option<(states::FactionState, states::FactionState, states::FactionState)> {
	// get the factions, or early return None if anything here doesnt exist - thats what the ? does
	// green
	let green_faction_file = os::client::watch_file(PROGRAM_ID, "factions/green").data?;
	let green_faction_deserialized = states::FactionState::try_from_slice(&green_faction_file.contents).ok()?;
	// orange
	let orange_faction_file = os::client::watch_file(PROGRAM_ID, "factions/orange").data?;
	let orange_faction_deserialized = states::FactionState::try_from_slice(&orange_faction_file.contents).ok()?;
	// purple
	let purple_faction_file = os::client::watch_file(PROGRAM_ID, "factions/purple").data?;
	let purple_faction_deserialized = states::FactionState::try_from_slice(&purple_faction_file.contents).ok()?;
	// return
	Some((green_faction_deserialized, orange_faction_deserialized, purple_faction_deserialized))
}