use crate::*;

pub fn deserialize_metastate() -> Option<states::MetaState>{
	let metastate_file = os::client::watch_file(PROGRAM_ID, "metastate").data?;
	let metastate_deserialized = states::MetaState::try_from_slice(&metastate_file.contents).ok()?;
	Some(metastate_deserialized)
}