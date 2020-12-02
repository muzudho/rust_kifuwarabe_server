use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait

/**
 * クライアントから接続があったときのコールバック。
 */
pub type OnComingFn = fn(connection_number:i64);
/**
 * メッセージ受信。
 */
pub type OnReceivedFromClientFn = fn(req: &Request, res: &mut Response);
/**
 * サーバーからクライアントへメッセージを送信できるタイミング。
 */
pub type OnSendToClientFn = fn(connection_number:i64, res: &mut Response);
/**
 * サーバーが何か処理をするタイミング。
 */
pub type OnServerSteppedFn = fn();

pub fn on_coming_empty(connection_number:i64) {
    println!("on_coming_empty<{}", connection_number);
}

pub fn on_received_from_client_empty(req: &Request, _res: &mut Response) {
    println!("on_received_from_client_empty<{} {}", req.get_connection_number(), req.get_message());
}

pub fn on_send_to_client_empty(connection_number:i64, _res: &mut Response) {
    println!("on_send_to_client_empty<{}", connection_number);
}

pub trait Request {
    fn as_mut_any(&mut self) -> &mut dyn Any; // トレイトを実装している方を返すのに使う。
    fn get_connection_number(&self) -> i64;
    fn get_message(&self) -> &str;
}

/**
 * クライアント側の入力を処理したあとに使う。サーバーへまとめる。
 */
pub trait Response {
    fn as_any(&self) -> &dyn Any; // トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any; // トレイトを実装している方を返すのに使う。
    fn set_message(&mut self, &str);
}
