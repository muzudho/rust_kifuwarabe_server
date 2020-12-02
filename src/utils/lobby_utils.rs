use client_handle_impl::LOBBY;

pub struct LobbyUtil {

}
impl LobbyUtil{
    /// 2プレイヤー以上居るなら真。
    pub fn more_than_2player() -> bool {
        1 < LOBBY.try_read().unwrap().waiting_players.len()
    }

    /// キューにプレイヤーを突っ込む。
    pub fn push_player(connection_number: i64) {
        LOBBY
            .try_write()
            .unwrap()
            .waiting_players
            .push_back(connection_number);
    }
    /// キューから先頭の１人を取り出す。
    pub fn pop_player() -> i64 {
        LOBBY
            .try_write()
            .unwrap()
            .waiting_players
            .pop_front()
            .unwrap()
    }
}
