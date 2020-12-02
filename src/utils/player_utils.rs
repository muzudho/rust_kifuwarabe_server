//use kifuwarabe_server::*;
use client_handle_impl::*;
use models::player::Player;
use client_handle_impl::PLAYER_MAP;

pub struct PlayerUtil {}
impl PlayerUtil {
    // ハッシュマップに追加。
    pub fn insert(connection_number: i64, player: Player) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, player);
    }
    // プレイヤー名をセット。
    pub fn set_name(connection_number: i64, player_name: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&connection_number)
            .unwrap()
            .set_name(player_name.to_string());
    }
    // パスワードをセット。
    pub fn set_password(connection_number: i64, password: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&connection_number)
            .unwrap()
            .set_password(password);
    }
    pub fn get_entry_game(connection_number: i64) -> i64 {
        PLAYER_MAP
            .try_read()
            .unwrap()
            .get(&connection_number)
            .expect("player-get-entry-game")
            .get_entry_game()
    }

    pub fn get_name(player_num: i64) -> String {
        PLAYER_MAP
            .try_read()
            .unwrap()
            .get(&player_num)
            .unwrap()
            .get_name()
            .to_string()
    }
    pub fn set_game(player_num: i64, game_num: i64) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&player_num)
            .expect("player-set-game")
            .set_entry_game(game_num)
    }
}
