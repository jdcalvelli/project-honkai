mod utils;
mod structs;
mod states;
mod enums;
mod game_scenes;
mod server_funcs;

use game_scenes::*;

static PROGRAM_ID: &str = "asdfghjkl";

turbo::cfg! {r#"
    name = "project-honkai"
    version = "1.1.0"
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
        item_name: String,
        has_broken_level_up: bool,
        has_broken_computer: bool
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
            last_event_time: 0,
            item_name: "".to_string(),
            has_broken_level_up: false,
            has_broken_computer: false,
        }
    }
}

turbo::go! ({
    let mut local_state = LocalState::load();

    // background true of every scene

    sprite!("background_layer", x = 0, y = 0);

    // multiply the length of item_name by 5
    let item_name_length = local_state.item_name.len() * 5;
    if item_name_length >= 220 {
        local_state.has_broken_level_up = true;
    }
    if item_name_length >= 270 {
        local_state.has_broken_computer = true;
    }

    sprite!("bg_keyboard", x = 0, y = 210);
    if local_state.egghead_state {
        sprite!("spacebar_02", x = 126, y = 269);
        sprite!("hand_02", x = 48, y = 266);
    }
    else {
        sprite!("spacebar_01", x = 126, y = 268);
        sprite!("hand_01", x = 47, y = 263);
    }

    // check for reset event

    if let Some(event) = os::client::watch_events(PROGRAM_ID, Some("alert")).data {
        // if the time of the current event is not the same as the last one saved
        if event.created_at != local_state.last_event_time {
            // save the event as the last event
            local_state.last_event_time = event.created_at;
            // then do whatever i want when the event happens, which in this case is reset the player
            // set game scene back to starting scene
            local_state.game_scene = enums::GameScenes::LastFactionWinScene;
        }
    }

    // more robust scene management
    // get user id, for use across scenes
    let user_id = os::client::user_id().unwrap();

    match (local_state.game_scene, utils::deserialize_player(&user_id), utils::deserialize_factions(), utils::deserialize_metastate()) {
        (_, _, _, None) => {
            // create metastate
            os::client::exec(PROGRAM_ID, "create_meta_state_data", &[]);
        },
        (_, _, None, _) => {
            // log!("CREATE FACTIONS");
            os::client::exec(PROGRAM_ID, "create_faction_data", &borsh::to_vec(&enums::Factions::Green).unwrap());
            os::client::exec(PROGRAM_ID, "create_faction_data", &borsh::to_vec(&enums::Factions::Orange).unwrap());
            os::client::exec(PROGRAM_ID, "create_faction_data", &borsh::to_vec(&enums::Factions::Purple).unwrap());
        },
        (_, None, _, _) => {
            // log!("CREATE PLAYER");
            os::client::exec(PROGRAM_ID, "create_player_data", &borsh::to_vec(&enums::Factions::NoFaction).unwrap());
        },
        (enums::GameScenes::MainMenuScene, Some(player_state_deserialized), Some(faction_states_deserialized), Some(metastate_deserialized)) => {
            main_menu_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            main_menu_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            main_menu_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
        },
        (enums::GameScenes::FactionSelectScene, Some(player_state_deserialized), Some(faction_states_deserialized), Some(metastate_deserialized)) => {
            faction_select_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            faction_select_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            faction_select_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
        },
        (enums::GameScenes::IdleGameScene, Some(player_state_deserialized), Some(faction_states_deserialized), Some(metastate_deserialized)) => {
            idle_game_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            idle_game_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            idle_game_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
        },
        (enums::GameScenes::TierUpScene, Some(player_state_deserialized), Some(faction_states_deserialized), Some(metastate_deserialized)) => {
            tier_up_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            tier_up_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            tier_up_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
        },
        (enums::GameScenes::LevelUpScene, Some(player_state_deserialized), Some(faction_states_deserialized), Some(metastate_deserialized)) => {
            level_up_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            level_up_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            level_up_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
        },
        (enums::GameScenes::LastFactionWinScene, Some(player_state_deserialized), Some(faction_states_deserialized), Some(metastate_deserialized)) => {
            last_faction_win_scene::update(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            last_faction_win_scene::draw(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
            last_faction_win_scene::input(&mut local_state, &player_state_deserialized, &faction_states_deserialized, &metastate_deserialized);
        }
    }

    if local_state.has_broken_computer {
        sprite!("outerframe_layer_broken", x = 16, y = 0);
    }
    else {
        sprite!("outerframe_layer", x = 16, y = 0);
    }

    //
    // TESTING AREA
    //
    // let test_read = os::client::read_file(PROGRAM_ID, "metastate");
    // if test_read.is_ok() {
    //     // log!{"{:?}", states::MetaState::try_from_slice(&test_read.clone().unwrap().contents).unwrap().last_faction_win};
    //     // log!{"{:?}", states::MetaState::try_from_slice(&test_read.clone().unwrap().contents).unwrap().player_list};
    // }

    local_state.save();
});
