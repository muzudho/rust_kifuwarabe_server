//use kifuwarabe_server::*;
use models::game::*;
use client_handle_impl::GAME_MAP;
//use client_handle_impl::LOBBY;
//use client_handle_impl::*;

pub struct GameUtil {}
impl GameUtil {
    /// 対局番号をセット。
    pub fn set_number(game: &mut Game, value: i64) {
        game.set_number(value);
    }
    /// プレイヤー名をセット。
    pub fn set_player0_name(game: &mut Game, player_name: &str) {
        game.get_mut_player0().set_name(player_name.to_string());
    }
    /// プレイヤー名をセット。
    pub fn set_player1_name(game: &mut Game, player_name: &str) {
        game.get_mut_player1().set_name(player_name.to_string());
    }
    pub fn insert(game_num: i64, game: Game) {
        GAME_MAP.try_write().unwrap().insert(game_num as i64, game);
    }
    /// メッセージ作成。
    pub fn get_game_summary_string(game_num: i64) -> String {
        GAME_MAP.try_read().unwrap()[&game_num].to_game_summary_string_ln()
    }
}
