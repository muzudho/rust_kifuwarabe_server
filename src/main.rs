/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
/// - https://gigazine.net/news/20120831-10000-jointer-1-server-cedec2012/ |サーバーマシン1台で同時接続者数1万名を実現するにはどうすればいいのかというノウハウと考え方
///
/// # コマンド例。
///
/// ```
/// ###  [Windows]+[R]キー, "cmd"+[Enter]。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shogi_server
///
/// ### コンパイル(開発中)。
/// cargo clippy
///
/// ### コンパイル(リリース用)。
/// cargo build --release
///
/// ### 実行。
/// cargo run --release
///
/// ### 開けているポート確認。
/// netstat
/// ```

#[macro_use]
extern crate lazy_static;

extern crate kifuwarabe_server;
use kifuwarabe_server::*;

mod details;
mod models;
mod utils;

extern crate kifuwarabe_shell;
extern crate serde_json;

mod shell_impl;
use shell_impl::*;

mod server_impl;
use server_impl::*;

pub mod client_handle_impl;
use client_handle_impl::*;

use std::thread;
use std::time::Duration;

const CONNECTION_STRING: &str = "127.0.0.1:4081";

fn main() {
    println!("I am a shogi server!");

    // グローバル変数の用意。
    setup_diagram();

    // サーバー構造体に、コールバック関数を登録していけだぜ。
    let server = &Server {
        on_coming: on_coming_shogi,
        on_received_from_client: on_received_from_client_shogi,
        on_send_to_client: on_send_to_client_shogi,
    };

    // 静的に、受信ポートを開いて待機。
    listen_incoming(&server, CONNECTION_STRING);

    loop {
        // 
        loop_server();

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}
