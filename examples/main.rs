/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
/// - https://gigazine.net/news/20120831-10000-jointer-1-server-cedec2012/ |サーバーマシン1台で同時接続者数1万名を実現するにはどうすればいいのかというノウハウと考え方
///
/// # コマンド例。
///
/// ```
/// ### [Windows]+[R]キー, "cmd"+[Enter]。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_server
/// 
/// ### コンパイル。
/// cargo clippy
///
/// ### コンパイル(リリース用)。
/// cargo build --release
///
/// ### 実行。
/// cargo run --example main
/// 
/// ### 開けているポート確認。
/// netstat
/// ```

extern crate kifuwarabe_server;
use kifuwarabe_server::*;

const CONNECTION_STRING: &str = "127.0.0.1:4081";

fn main(){
    listen(CONNECTION_STRING);
}