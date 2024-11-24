use states::{FactionState, PlayerState};

mod utils;
mod states;
mod enums;
mod game_scenes;
mod server_funcs;

use game_scenes::*;

turbo::cfg! {r#"
    name = "project-honkai"
    version = "0.1.0"
    author = "jd calvelli and devinne moses"
    description = "a game about modern games"
    [settings]
    resolution = [384, 216]
    [turbo-os]
    api-url = "https://os.turbo.computer"
"#}

turbo::init! {
    // something only the player will see? menus, interpolated values, etc, local ui state eg, tweens, transitions, etc
    // dont need to round trip with the server at all!
    struct LocalState {
        game_scene: enums::GameScenes,
        view_flip: bool,
        selector_pos: u16,
        egghead_state: bool,
    } = {
        Self::new()
    }
}

impl LocalState {
    fn new() -> Self {
        Self {
            game_scene: enums::GameScenes::FactionSelectScene,
            view_flip: true,
            selector_pos: 0,
            egghead_state: false
        }
    }
}

turbo::go! ({
    let mut local_state = LocalState::load();

    // get user id, for use across scenes
    let user_id = os::client::user_id().unwrap();

    match (local_state.game_scene, deserialize_player(&user_id), deserialize_factions()) {
        (_, None, _) => {
            os::client::exec("project_honkai", "create_player_data", "none".as_bytes());
        },
        (_, _, None) => {
            os::client::exec("project_honkai", "create_faction_data", "green".as_bytes());
            os::client::exec("project_honkai", "create_faction_data", "orange".as_bytes());
            os::client::exec("project_honkai", "create_faction_data", "purple".as_bytes());
        },
        (enums::GameScenes::FactionSelectScene, Some(player_state_deserialized), Some(faction_states_deserialized)) => {
            faction_select_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            faction_select_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            faction_select_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
        },
        (enums::GameScenes::IdleGameScene, Some(player_state_deserialized), Some(faction_states_deserialized)) => {
            idle_game_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            idle_game_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            idle_game_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
        },
        (enums::GameScenes::TierUpScene, Some(player_state_deserialized), Some(faction_states_deserialized)) => {
            tier_up_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            tier_up_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            tier_up_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
        },
        _ => ()
    }

    local_state.save();
});

// UTILITY FUNCS

fn deserialize_player(user_id: &str) -> Option<PlayerState>{
    // get the player state, or return early none if anything doesnt exist
    let player_file = os::client::read_file("project_honkai", &format!("players/{user_id}")).ok()?;
    let player_deserialized = states::PlayerState::try_from_slice(&player_file.contents).ok()?;
    Some(player_deserialized)
}

fn deserialize_factions() -> Option<(FactionState, FactionState, FactionState)> {
    // get the factions, or early return None if anything here doesnt exist - thats what the ? does
    // green
    let green_faction_file = os::client::read_file("project_honkai", "factions/green").ok()?;
    let green_faction_deserialized = states::FactionState::try_from_slice(&green_faction_file.contents).ok()?;
    // orange
    let orange_faction_file = os::client::read_file("project_honkai", "factions/orange").ok()?;
    let orange_faction_deserialized = states::FactionState::try_from_slice(&orange_faction_file.contents).ok()?;
    // purple
    let purple_faction_file = os::client::read_file("project_honkai", "factions/purple").ok()?;
    let purple_faction_deserialized = states::FactionState::try_from_slice(&purple_faction_file.contents).ok()?;
    // return
    Some((green_faction_deserialized, orange_faction_deserialized, purple_faction_deserialized))
}