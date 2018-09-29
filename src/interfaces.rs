use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait

/**
 * メッセージ受信。
 */
pub type Receiver = fn(connection_number: i64, message: &str, res: &mut Response);

pub fn empty_receiver(connection_number: i64, message: &str, _res: &mut Response) {
    println!("empty_receiver<{} {}", connection_number, message);
}

/**
 * クライアント側の入力を処理したあとに使う。サーバーへまとめる。
 */
pub trait Response {
    fn as_any(&self) -> &dyn Any; // トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any; // トレイトを実装している方を返すのに使う。
    fn set_message(&mut self, &str);
}
