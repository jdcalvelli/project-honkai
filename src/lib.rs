//

const METADATA_BYTES: &[u8; 425] = b"{\"name\":\"project honkai\",\"program_id\":\"ggmvgxdev3\",\"owner_id\":\"ed8ac764-ff3f-40fd-b4b8-9191e590432a\",\"commands\":[{\"name\":\"acknowledge_last_faction_win\"},{\"name\":\"acknowledge_level_up\"},{\"name\":\"acknowledge_tier_up\"},{\"name\":\"create_faction_data\"},{\"name\":\"create_meta_state_data\"},{\"name\":\"create_player_data\"},{\"name\":\"increment_faction_level\"},{\"name\":\"increment_player_xp\"},{\"name\":\"update_player_faction\"}],\"channels\":[]}";

#[used]
#[allow(non_upper_case_globals)]
#[link_section = "turbo_os_program_metadata"]
pub static TURBO_METADATA: [u8; 425] = *METADATA_BYTES;

//

use turbo::*;
use borsh::BorshDeserialize;

mod enums;
mod game_scenes;
mod server_funcs;
mod states;
mod structs;
mod utils;

use game_scenes::*;

static PROGRAM_ID: &str = "ggmvgxdev3";

#[turbo::game]
struct LocalState {
    game_scene: enums::GameScenes,
    view_flip: bool,
    selector_pos: u16,
    egghead_state: bool,
    last_event_time: u32,
    item_name: String,
    has_broken_level_up: bool,
    has_broken_computer: bool,
    is_item_sound_selected: bool,
    num_presses: u32,
}

impl LocalState {
    pub fn new() -> Self {
        Self {
            game_scene: enums::GameScenes::MainMenuScene,
            view_flip: true,
            selector_pos: 0,
            egghead_state: false,
            last_event_time: 0,
            item_name: "".to_string(),
            has_broken_level_up: false,
            has_broken_computer: false,
            is_item_sound_selected: false,
            num_presses: 0,
        }
    }

    pub fn update(&mut self) {
        // background true of every scene

        sprite!("background_layer", x = 0, y = 0);

        // multiply the length of item_name by 5
        let item_name_length = self.item_name.len() * 5;
        if item_name_length >= 220 {
            self.has_broken_level_up = true;
        }
        if item_name_length >= 270 {
            self.has_broken_computer = true;
        }

        sprite!("bg_keyboard", x = 0, y = 210);
        if self.egghead_state {
            sprite!("spacebar_02", x = 126, y = 269);
            sprite!("hand_02", x = 48, y = 266);
        } else {
            sprite!("spacebar_01", x = 126, y = 268);
            sprite!("hand_01", x = 47, y = 263);
        }

        // check for reset event

        if let Some(event) = os::client::watch_events(PROGRAM_ID, Some("alert")).data {
            // if the time of the current event is not the same as the last one saved
            if event.created_at != self.last_event_time {
                // save the event as the last event
                self.last_event_time = event.created_at;
                // then do whatever i want when the event happens, which in this case is reset the player
                // set game scene back to starting scene
                self.game_scene = enums::GameScenes::LastFactionWinScene;
            }
        }

        // more robust scene management
        // get user id, for use across scenes
        let user_id = os::client::user_id().unwrap();

        match (
            &mut self.game_scene,
            utils::deserialize_player(&user_id),
            utils::deserialize_factions(),
            utils::deserialize_metastate(),
        ) {
            (_, _, _, None) => {
                // create metastate
                if gamepad::get(0).start.just_pressed() {
                    log!("CREATE METASTATE");
                    os::client::command::exec_raw(PROGRAM_ID, "create_meta_state_data", &[]);
                }
            }
            (_, _, None, _) => {
                if gamepad::get(0).start.just_pressed() {
                    log!("CREATE FACTIONS");
                    os::client::command::exec_raw(
                        PROGRAM_ID,
                        "create_faction_data",
                        &borsh::to_vec(&enums::Factions::Green).unwrap(),
                    );
                    os::client::command::exec_raw(
                        PROGRAM_ID,
                        "create_faction_data",
                        &borsh::to_vec(&enums::Factions::Orange).unwrap(),
                    );
                    os::client::command::exec_raw(
                        PROGRAM_ID,
                        "create_faction_data",
                        &borsh::to_vec(&enums::Factions::Purple).unwrap(),
                    );
                }
            }
            (_, None, _, _) => {
                if gamepad::get(0).start.just_pressed() {
                    log!("CREATE PLAYER");
                    os::client::command::exec_raw(
                        PROGRAM_ID,
                        "create_player_data",
                        &borsh::to_vec(&enums::Factions::NoFaction).unwrap(),
                    );
                }
            }
            (
                enums::GameScenes::MainMenuScene,
                Some(player_state_deserialized),
                Some(faction_states_deserialized),
                Some(metastate_deserialized),
            ) => {
                main_menu_scene::update(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                main_menu_scene::draw(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                main_menu_scene::input(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
            }
            (
                enums::GameScenes::FactionSelectScene,
                Some(player_state_deserialized),
                Some(faction_states_deserialized),
                Some(metastate_deserialized),
            ) => {
                faction_select_scene::update(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                faction_select_scene::draw(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                faction_select_scene::input(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
            }
            (
                enums::GameScenes::IdleGameScene,
                Some(player_state_deserialized),
                Some(faction_states_deserialized),
                Some(metastate_deserialized),
            ) => {
                idle_game_scene::update(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                idle_game_scene::draw(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                idle_game_scene::input(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
            }
            (
                enums::GameScenes::TierUpScene,
                Some(player_state_deserialized),
                Some(faction_states_deserialized),
                Some(metastate_deserialized),
            ) => {
                tier_up_scene::update(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                tier_up_scene::draw(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                tier_up_scene::input(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
            }
            (
                enums::GameScenes::LevelUpScene,
                Some(player_state_deserialized),
                Some(faction_states_deserialized),
                Some(metastate_deserialized),
            ) => {
                level_up_scene::update(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                level_up_scene::draw(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                level_up_scene::input(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
            }
            (
                enums::GameScenes::LastFactionWinScene,
                Some(player_state_deserialized),
                Some(faction_states_deserialized),
                Some(metastate_deserialized),
            ) => {
                last_faction_win_scene::update(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                last_faction_win_scene::draw(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
                last_faction_win_scene::input(
                    self,
                    &player_state_deserialized,
                    &faction_states_deserialized,
                    &metastate_deserialized,
                );
            }
        }

        if self.has_broken_computer {
            sprite!("outerframe_layer_broken", x = 16, y = 0);
        } else {
            sprite!("outerframe_layer", x = 16, y = 0);
        }

        // play the background music loop only on title screen
        // if self.game_scene != enums::GameScenes::MainMenuScene {
        //     audio::stop("Title_Screen_Loop");
        // } else if !audio::is_playing("Title_Screen_Loop") {
        //     audio::play("Title_Screen_Loop");
        // }

        //
        // TESTING AREA
        //
        // let test_read = os::client::fs::watch(format!("{}/metastate", PROGRAM_ID));
        // if test_read.data.is_some() {
        //     log! {"{:?}", states::MetaState::try_from_slice(&test_read.data.unwrap().contents).unwrap().player_list};
        // } else {
        //     log!("no data at metastate?");
        // }
    }
}
