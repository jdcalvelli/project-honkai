use states::PlayerState;

mod states;
mod server_funcs;
mod commands;

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
        player_state: states::PlayerState,
        global_state: states::GlobalState,
    } = {
        Self::new()
    }
}

impl LocalState {
    fn new() -> Self {
        Self {
            // need to get the player's state if it exists, otherwise create and send
            player_state: states::PlayerState::load_remote(),
            // similarly, need to get the global state if it exists, otherwise create and send
            global_state: states::GlobalState::new()
        }
    }
}

turbo::go! ({
    let mut local_state = LocalState::load();

    let [canvas_width, canvas_height] = canvas_size!();

    // get user id
    // let user_id = os::user_id().unwrap_or("NO ID".to_string());

    let mut player_state = states::PlayerState::load_remote();

    // input logic
    if gamepad(0).start.just_pressed() {
        PlayerState::exec_increment_xp();

        // if local_state.player_state.get_xp() % 10 == 0 && local_state.player_state.get_xp() != 0 {
        //     local_state.global_state.increment_total();
        // }
    }

    // draws
    text!(&player_state.get_xp().to_string(), x = canvas_width / 2, y = canvas_height / 2);
    // text!(&local_state.global_state.get_total().to_string(), x = canvas_width / 2, y = canvas_height / 2 + 16);
    // text!(&format!("{user_id}"), x = canvas_width / 2, y = canvas_height / 2 + 32);

    local_state.save();
});