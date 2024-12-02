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
            game_scene: enums::GameScenes::MainMenuScene,
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

    match (local_state.game_scene, utils::deserialize_player(&user_id), utils::deserialize_factions()) {
        (enums::GameScenes::MainMenuScene, Some(player_state_deserialized), Some(faction_states_deserialized)) => {
            main_menu_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            main_menu_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            main_menu_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
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
        (enums::GameScenes::LevelUpScene, Some(player_state_deserialized), Some(faction_states_deserialized)) => {
            level_up_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            level_up_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
            level_up_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized);
        },
        (_, None, _) => {
            log!("CREATE PLAYER");
            os::client::exec("project_honkai", "create_player_data", &borsh::to_vec(&enums::Factions::NoFaction).unwrap());
        },
        (_, _, None) => {
            log!("CREATE FACTIONS");
            os::client::exec("project_honkai", "create_faction_data", &borsh::to_vec(&enums::Factions::Green).unwrap());
            os::client::exec("project_honkai", "create_faction_data", &borsh::to_vec(&enums::Factions::Orange).unwrap());
            os::client::exec("project_honkai", "create_faction_data", &borsh::to_vec(&enums::Factions::Purple).unwrap());
        },
    }

    local_state.save();
});
