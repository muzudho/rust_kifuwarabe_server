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
use kifuwarabe_server::interfaces::*;
use kifuwarabe_server::*;
use std::thread;
use std::time::Duration;

const CONNECTION_STRING: &str = "127.0.0.1:4081";

fn main() {

    let server = &Server {
        coming: default_coming,
        receiving: default_receiving,
    };

    // 静的に、受信ポートを開いて待機。
    listen_incoming(&server, CONNECTION_STRING);

    loop {
        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}

/**
 * クライアントからの接続があったとき、その接続に番号を振る。
 */
fn default_coming(connection_number:i64) {
    println!("default_coming: Welcome {}!", connection_number);
}

/**
 * クライアントからの入力を受け取り、応答を返す。
 */
fn default_receiving(req: &Request, res: &mut Response) {
    println!("<{} {}", req.get_connection_number(), req.get_message());

    match req.get_message() {
        "LOGIN kifuwarabe a" => res.set_message("LOGIN:kifuwarabe OK"),
        _ => {
            println!("<{} Not match: [{}]", req.get_connection_number(), req.get_message());
        }
    }
}
