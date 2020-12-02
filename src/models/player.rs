#[derive(Default)]
/// Shell遷移状態は <ShellVar>ジェネリック型が付いて煩雑になるので 別グローバル変数に外出しした。
pub struct Player {
    /// 接続者番号。
    number: i64,
    /// 対局者名。
    name: String,
    /// ログイン時のパスワード。
    password: String,
    /// 参加している対局の番号。
    entry_game: i64,
}
impl Player {
    pub fn new() -> Player {
        Player {
            number: -1,
            name: "".to_string(),
            password: "".to_string(),
            entry_game: -1,
        }
    }

    pub fn get_number(&self) -> i64 {
        self.number
    }
    pub fn set_number(&mut self, value:i64) {
        self.number = value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, value:String) {
        self.name = value
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }
    pub fn set_password(&mut self, value: &str) {
        self.password = value.to_string();
    }

    pub fn get_entry_game(&self) -> i64 {
        self.entry_game
    }
    pub fn set_entry_game(&mut self, value:i64) {
        self.entry_game = value
    }

}
