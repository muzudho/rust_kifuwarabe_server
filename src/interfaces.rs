use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait

/**
 * メッセージ受信。
 */
pub type Receiver = fn(req: &Request, res: &mut Response);

pub fn empty_receiver(req: &Request, _res: &mut Response) {
    println!("empty_receiver<{} {}", req.get_connection_number(), req.get_message());
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