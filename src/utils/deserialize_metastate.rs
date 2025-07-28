use crate::*;

pub fn deserialize_metastate() -> Option<states::MetaState> {
    let metastate_file = os::client::fs::watch(format!("{}/metastate", PROGRAM_ID)).data?;
    let metastate_deserialized =
        states::MetaState::try_from_slice(&metastate_file.contents).ok()?;
    Some(metastate_deserialized)
}

