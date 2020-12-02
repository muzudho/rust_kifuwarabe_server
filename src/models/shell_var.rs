pub use models::player::*;

/// 任意のオブジェクト。
pub struct ShellVar {
    connection_number: i64,
    /// クライアントに送信する文字列。
    message_to_client: String,
}
impl Default for ShellVar {
    fn default() -> Self {
        Self::new()
    }
}
impl ShellVar {
    pub fn new() -> ShellVar {
        ShellVar {
            connection_number: -1,
            message_to_client: "".to_string(),
        }
    }

    pub fn get_connection_number(&self) -> i64 {
        self.connection_number
    }
    pub fn set_connection_number(&mut self, value:i64){
        self.connection_number = value
    }
        
    pub fn get_message_to_client(&self) -> String {
        self.message_to_client.to_string()
    }
    pub fn set_message_to_client(&mut self, value: &str) {
        self.message_to_client = value.to_string();
    }
}
