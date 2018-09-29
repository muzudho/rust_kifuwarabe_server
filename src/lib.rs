use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::io::Write;
/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
/// - https://gigazine.net/news/20120831-10000-jointer-1-server-cedec2012/ |サーバーマシン1台で同時接続者数1万名を実現するにはどうすればいいのかというノウハウと考え方
///
/// # コマンド例。
///
/// ```
/// ### [Windows]+[R]キー, "cmd"+[Enter]。
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
use std::net::{TcpListener, TcpStream};
use std::{thread, time};
#[macro_use]
extern crate lazy_static;
use std::any::Any;
use std::time::Duration; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait

pub mod interfaces;
use interfaces::*;

/// クライアント１つに１つずつ割り当てる変数。
#[derive(Default)]
pub struct ClientVar {
    /// 汎用的に利用できるハッシュマップ。
    #[allow(dead_code)]
    properties: HashMap<String, String>,
}
impl ClientVar {
    pub fn new() -> ClientVar {
        ClientVar {
            properties: HashMap::new(),
        }
    }
}

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    /// クライアントを超えて共有する。 <接続番号,変数>
    pub static ref CLIENT_MAP: RwLock<HashMap<i64, ClientVar>> = RwLock::new(HashMap::new());
}

/// このアプリケーションのオプション。
pub struct Server {
    pub receiver: Receiver,
}
impl Server {
    pub fn new() -> Server {
        Server {
            receiver: empty_receiver,
        }
    }
}
impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

/// リクエスト。
pub struct RequestStruct {
    pub message: String, // String型は長さが可変なので、固定長のBoxでラップする。
    pub connection_number: i64,
}
impl RequestStruct {
    fn new() -> RequestStruct {
        RequestStruct {
            message: "".to_string(),
            connection_number: -1,
        }
    }
}
impl Request for RequestStruct {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    fn get_connection_number(&self) -> i64 {
        self.connection_number
    }
    fn get_message(&self) -> &str {
        &self.message
    }
}

/// レスポンス。
#[derive(Default)]
pub struct ResponseStruct {
    pub message: String,
}
impl ResponseStruct {
    pub fn new() -> ResponseStruct {
        ResponseStruct {
            message: "".to_string(),
        }
    }
}
impl Response for ResponseStruct {
    fn as_any(&self) -> &dyn Any {
        self
    }
    /// トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    // .rs にハードコーディングして使う。
    fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }
}

/**
 * 静的に、受信ポートを開いて待機します。
 */
pub fn listen(server: &'static Server, connection_str: &'static str) {
    // println!("I am a server!");

    // 接続受付スレッド。
    thread::spawn(move || {
        println!("S> Listen!!");
        let mut connection_number = 0;
        let sever_listener = TcpListener::bind(connection_str).unwrap();
        for stream_wrap in sever_listener.incoming() {

            // クライアントごとの変数を割り当てる。
            CLIENT_MAP
                .try_write()
                .unwrap()
                .insert(connection_number, ClientVar::new());

            // さらに別スレッド開始。
            thread::spawn(move || {
                handle_client(server, connection_number, &mut stream_wrap.unwrap());
            });
            connection_number += 1;
        }
    });

    // 各クライアントに何かしたいことがあれば 以下に書く。
    loop {
        /*
        let mut count = 0;
        match CLIENT_MAP.try_read() {
            Ok(client_map) => {
                for (_connection_number, _client_var) in client_map.iter() {
                    count += 1;
                }
            },
            Err(_) => unreachable!(),
        };
        println!("count = {}", count);
        */
        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}

/// クライアントをずっと捕まえておく。
fn handle_client(server: &'static Server, connection_number: i64, stream: &mut TcpStream) {
    println!("S2> Welcome {}.", connection_number);

    // ブロックし続けないようにする。
    // let _ = stream.set_read_timeout(Some(Duration::new(10, 0)));
    stream
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    // 使いまわすもの。
    let mut buf = [0];
    let mut buf_arr = [0; 1024]; // FIXME 短くないか☆（＾～＾）？
    let mut index = 0;
    let mut req = RequestStruct::new();
    let mut res = ResponseStruct::new();
    // FIXME 切断のエラーをキャッチしたい。
    loop {
        // 読み取り。
        // FIXME マルチバイト文字の受け取り方が分からん☆（＾～＾）1バイトずつ取る。
        match stream.take(1).read(&mut buf) {
            Ok(len) => {
                if 0 == len {
                    // 長さが 0 なら切断と判定する。
                    break;
                }

                if buf[0] == b'\n' {
                    // ジョイン。(改行は含まない)
                    let line = String::from_utf8_lossy(&buf_arr[0..index]);

                    // ****************************************************************************************************
                    //  クライアントからの入力を、呼び出し側に処理させる。
                    // ****************************************************************************************************
                    // println!("S2>{} {}", connection_number, line);
                    req.connection_number = connection_number;
                    req.message = line.to_string();
                    (server.receiver)(&mut req, &mut res);

                    /*
                    println!(
                         "S2>{} message: [{}] line: [{}]",
                         connection_number, res.message, line
                    );
                    */

                    index = 0;
                }

                buf_arr[index] = buf[0];
                index += 1;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // ブロックしなければ、ここにくる。
                // print!("[wait]");
                let msec = time::Duration::from_millis(10);
                thread::sleep(msec);
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };

        if res.message != "" {
            println!(
                    "S2>{} {}",
                    connection_number, res.message
            );
            // 何か応答したい。
            match stream.write(res.message.as_bytes()) {
                Ok(_n) => {},
                Err(e) => panic!("encountered IO error: {}", e),
            }
            match stream.flush(){
                Ok(_n) => {},
                Err(e) => panic!("encountered IO error: {}", e),
            }

            // クリアー。
            res.message = "".to_string();
        }
    }

    println!("S2> Bye {}.", connection_number);
}
