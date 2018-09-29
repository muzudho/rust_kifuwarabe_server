# What is rust_kifuwarabe_server?

## Overview.

- １つのサーバー と、
- 複数のクライアント が、
- TCP/IP 通信で

接続してくる部分のプログラムだぜ☆（*＾～＾*）

何を楽にするかというと、

```
/**
 * クライアントからの接続があったとき、その接続に番号を振る。
 */
fn default_coming(connection_number:i64) {
    println!("Welcome {}!", connection_number);

    // 接続番号をたよりに ここで変数を初期化したりする。
}

/**
 * クライアントからの入力を受け取り、応答を返す。
 */
fn default_receiving(req: &Request, res: &mut Response) {
    println!("{} by No.{}.", req.get_message(), req.get_connection_number());

    // ここで ごりごり パーサーを書く。
    match req.get_message() {
        "Hello!" => res.set_message("Hi!"),
        _ => { println!("What!?"); }
    }
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

## 実行。

```
// 接続文字列。
const CONNECTION_STRING: &str = "127.0.0.1:4081";

fn main() {

    // サーバー構造体に、コールバック関数を登録していけだぜ。
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
```
