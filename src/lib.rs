mod utils;
mod states;
mod server_funcs;

turbo::cfg! {r#"
    name = "project-honkai"
    version = "0.1.0"
    author = "jd calvelli and devinne moses"
    description = "a game about modern games"
    [settings]
    resolution = [384, 216]
    [turbo-os]
    api-url = "http://localhost:8000"
"#}


turbo::init! {
    // something only the player will see? menus, interpolated values, etc, local ui state eg, tweens, transitions, etc
    // dont need to round trip with the server at all!
    struct LocalState {
        egghead_state: bool,
    } = {
        Self::new()
    }
}

impl LocalState {
    fn new() -> Self {
        Self {
            egghead_state: false
        }
    }
}

turbo::go! ({
    let mut local_state = LocalState::load();

    // get user id
    let this_user_id = os::user_id().unwrap();

    // get remote data result for this user
    let this_player_remote_data = os::read_file("project_honkai", &format!("players/{this_user_id}"));

    // INSIDE OF HERE, BASICALLY, IS POST US GETTING THE PLAYER
    match this_player_remote_data {
        Ok(file) => {

            // *** UPDATE *** //

            let this_player_state = states::PlayerState::try_from_slice(&file.contents).unwrap();

            if this_player_state.current_xp == this_player_state.xp_needed_for_next_level {
                os::exec("project_honkai", "level_up_player", &[]);
            }

            // *** DRAW *** //

            // background
            sprite!("background_layer", x = 0, y = 0);

            // ui
            sprite!("ui_faction_bar", x = 38, y = 21);
            sprite!("ui_bp_bar", x = 39, y = 65);

            // rect overlay
            // interpolate btw player xp to 215 always - need something like the map() function in processing
            rect!(
                x = 86, 
                y = 88, 
                w = utils::range_map(
                    this_player_state.current_xp as f64, 
                    this_player_state.xp_needed_for_prev_level as f64, 
                    this_player_state.xp_needed_for_next_level as f64, 
                    0., 
                    215.),
                h = 6, color = 0xff0000ff
            );

            // foreground
            match local_state.egghead_state {
                true => {
                    sprite!("anim_egghead_press", x = 0, y = 0);
                },
                false => {
                    sprite!("anim_egghead_release", x = 0, y = 0);
                },
            }

            // very foreground
            sprite!("outerframe_layer", x = 0, y = 0);

            // testing
            text!(&this_player_state.current_level.to_string(), x = 100, y = 100, color = 0x000000ff);
            text!(&this_player_state.current_xp.to_string(), x = 100, y = 110, color = 0x000000ff);
            text!(&this_player_state.xp_needed_for_prev_level.to_string(), x = 100, y = 120, color = 0x000000ff);
            text!(&this_player_state.xp_needed_for_next_level.to_string(), x = 100, y = 130, color = 0x000000ff);

            // *** INPUT *** //

            if gamepad(0).start.just_pressed() {
                local_state.egghead_state = true;
                os::exec("project_honkai", "increment_player_xp", &[]);
            }
            else if gamepad(0).start.just_released() {
                local_state.egghead_state = false;
            }
        },
        Err(_err) => {
            // log!("DATA NOT THERE");
            // we need to make a player file for u on the server
            os::exec("project_honkai", "create_player_data", &[]);
        }
    }

    local_state.save();
});