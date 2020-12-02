use kifuwarabe_shell::diagram::*;
use client_handle_impl::SHELL_MAP;
use kifuwarabe_shell::shell::*;
use models::shell_var::*;

pub struct ShellMapUtils {}
impl ShellMapUtils {
    pub fn insert(connection_number: i64, shell: Shell<ShellVar>) {
        SHELL_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, shell);
    }

    pub fn get_current(connection_number: i64) -> String {
        SHELL_MAP
            .try_read()
            .expect("shell_map_utils.rs/ShellMapUtils/get_current/SHELL_MAP.try_write()")
            .get(&connection_number)
            .unwrap_or_else(|| panic!("shell_map_utils.rs/ShellMapUtils/get_current/get({})", &connection_number))
            .get_current()
    }

    pub fn forward(connection_number: i64, diagram: &Diagram<ShellVar>, next: &str) {
        SHELL_MAP
            .try_write()
            .expect("shell_map_utils.rs/ShellMapUtils/forward/SHELL_MAP.try_write()")
            .get_mut(&connection_number)
            //.expect(&format!("shell_map_utils.rs/ShellMapUtils/forward/get_mut({})", &connection_number))
            .unwrap_or_else(|| panic!("shell_map_utils.rs/ShellMapUtils/forward/get_mut({})", &connection_number))
            .forward_force(diagram, next);
    }
/*
    pub fn set_state(connection_number: i64, state: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&connection_number)
            .expect("player-set-state")
            .set_state(&state.to_string());
    }
    pub fn is_state(player_num: i64, state: &str) -> bool {
        //println!("is_state: {}. expected: {}.", player_num, state);

        // 接続者のステータス。
        let player_state;
        {
            player_state = PLAYER_MAP
                .try_read()
                .unwrap()
                .get(&player_num)
                .expect("is-state")
                .get_state();
        }

        // 比較。
        player_state == state

        /*
        match CLIENT_MAP
            .try_read()
            .expect("CLIENT_MAP.try_read()")
            .get(&player_num)
            .unwrap()
            .properties
            .get(&state.to_string()) {
                Some(n) => {state == n},
                None => {false},
            }
    */
        /*
        state == CLIENT_MAP
            .try_read()
            .expect("CLIENT_MAP.try_read()")
            .get(&player_num)
            .unwrap()
            .properties[&state.to_string()]
        */
    }
*/
}
