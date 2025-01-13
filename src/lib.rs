mod utils;
mod states;
mod enums;
mod game_scenes;
mod server_funcs;

use game_scenes::*;

static PROGRAM_ID: &str = "a";

turbo::cfg! {r#"
    name = "project-honkai"
    version = "0.1.0"
    author = "jd calvelli and devinne moses"
    description = "a game about modern games"
    [settings]
    resolution = [384, 326]
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
        last_event_time: u32,
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
            egghead_state: false,
            last_event_time: 0
        }
    }
}

turbo::go! ({
    let mut local_state = LocalState::load();

    // get user id, for use across scenes
    let user_id = os::client::user_id().unwrap();

    if let Some(event) = os::client::watch_events(PROGRAM_ID, Some("alert")).data {
        // if the time of the current event is not the same as the last one saved
        if event.created_at != local_state.last_event_time {
            // save the event as the last event
            local_state.last_event_time = event.created_at;
            // then do whatever i want when the event happens, which in this case is reset the player
            // doesn't maintain old faction for now
            os::client::exec(PROGRAM_ID, "create_player_data", &borsh::to_vec(&enums::Factions::NoFaction).unwrap());
            // set game scene back to starting scene
            local_state.game_scene = enums::GameScenes::MainMenuScene;
        }
    }


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
        (_, _, None) => {
            // log!("CREATE FACTIONS");
            os::client::exec(PROGRAM_ID, "create_faction_data", &borsh::to_vec(&enums::Factions::Green).unwrap());
            os::client::exec(PROGRAM_ID, "create_faction_data", &borsh::to_vec(&enums::Factions::Orange).unwrap());
            os::client::exec(PROGRAM_ID, "create_faction_data", &borsh::to_vec(&enums::Factions::Purple).unwrap());
        },
        (_, None, _) => {
            // log!("CREATE PLAYER");
            os::client::exec(PROGRAM_ID, "create_player_data", &borsh::to_vec(&enums::Factions::NoFaction).unwrap());
        },
    }

    // temp reset
    if gamepad(0).up.just_pressed() {
        os::client::exec(PROGRAM_ID, "temp_reset_game", &[]);
    }

    local_state.save();
});
