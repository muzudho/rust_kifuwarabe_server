# What is rust_kifuwarabe_server?

## Overview.

- １つのサーバー と、
- 複数のクライアント が、
- TCP/IP 通信で

接続してくる部分のプログラムだぜ☆（*＾～＾*）

### このプログラムが やってくれること。

次のことを やってくれるライブラリだぜ。

- (1) on_coming: クライアントに一意の connection-number (連番)を振る。i64型。
- (2) on_receiving: クライアントから受け取ったメッセージを処理できる コールバック関数を１つ提供。
- (3) on_sending: サーバーからクライアントにメッセージを送るための コールバック関数を１つ提供。

ソースの例。

```
/**
 * (1) クライアントからの接続があったとき、その接続に番号を振る。
 */
fn on_coming_default(connection_number:i64) {
    println!("Welcome {}!", connection_number);

    // 接続番号をたよりに ここで変数を初期化したりする。
}

/**
 * (2) クライアントからの入力を受け取り、応答を返す。
 */
fn on_receiving_default(req: &Request, res: &mut Response) {
    println!("{} by No.{}.", req.get_message(), req.get_connection_number());

    // ここで ごりごり パーサーを書く。
    match req.get_message() {
        "Hello!" => res.set_message("Hi!"),
        _ => { println!("What!?"); }
    }
}

/**
 * (3) サーバーからクライアントへメッセージを送信できるタイミング。
 */
pub fn on_sending_default(_connection_number:i64, _res: &mut Response) {
    // やることがなければ、何もしない。
}
```

というように、

- 接続番号
- 接続者からのメッセージ
- 接続者にメッセージを送る

という３つのことに集中して、パーサーを ごりごり 書こうぜ、というものだぜ☆（＾～＾）

# Instration.

```
[dependencies.kifuwarabe_server]
git = "https://github.com/muzudho/rust_kifuwarabe_server.git"
rev = "6369602... Please get new rev from git hub."
```

rev は Git hub を見て新しいのを入れろだぜ☆（＾～＾）

# How to use rust_kifuwarabe_server?

## ファイルの冒頭。

```
extern crate kifuwarabe_server;
use kifuwarabe_server::interfaces::*;
use kifuwarabe_server::*;
use std::thread;
use std::time::Duration;
```

## 設定。

```
/// (1) 接続文字列。
const CONNECTION_STRING: &str = "127.0.0.1:4081";

fn main() {

    // (2) サーバー構造体に、コールバック関数を登録していけだぜ。
    let server = &Server {
        on_coming: on_coming_default,
        on_receiving: on_receiving_default,
        on_sending: on_sending_default,
    };

}
```

## 実行。

```
fn main() {

    // 略。(Omit)

    // (1) 静的に、受信ポートを別スレッドで実行。
    listen_incoming(&server, CONNECTION_STRING);

    // (2) メインスレッドは無限ループ。
    loop {
        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}
```
