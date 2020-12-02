use kifuwarabe_shell::diagram::*;
use client_handle_impl::*;
use models::shell_var::*;
use utils::lobby_utils::*;
use utils::player_utils::*;

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    // Shell と Diagram は名前被りがあるので、ファイルを分けている。
    // 一度作ってしまえば、あとは読込のみ。
    pub static ref DIAGRAM: RwLock<Diagram<ShellVar>> = RwLock::new(Diagram::new());
}

/// グラフは１つ作れば、どのクライアントでも使いまわす。
pub fn setup_diagram() {
    let mut diagram = DIAGRAM.try_write().unwrap();

    // コントローラー登録。
    diagram.insert_fn("do_player_name", do_player_name);
    diagram.insert_fn("do_password", do_password);

    // ファイル読取。
    diagram.read_file(&DIAGRAM_JSON_FILE.to_string());
}

/// プレイヤー名を読み取った。
pub fn do_player_name(shell_var: &mut ShellVar, req: &Request, _res: &mut dyn Response) {
    // 接続者番号。
    let connection_num = shell_var.get_connection_number();
    let player_name = &req.get_groups()[0];

    // 接続者のログイン名を記録。
    PlayerUtil::set_name(connection_num, &player_name.to_string());

    println!(
        "<{} do_player_name: {}",
        connection_num,
        player_name
    );
}

/// 接続者前とパスワードを分解した。
/// 
/// - ログイン完了の通知をクライアントに返す。
/// - 対局はまだ付いていない。
pub fn do_password(shell_var: &mut ShellVar, req: &Request, _res: &mut dyn Response) {
    // 接続者番号。
    let connection_num = shell_var.get_connection_number();
    println!(
        "<{} do_password: {}",
        connection_num,
        req.get_groups()[0]
    );

    // 接続者のパスワードを記録。
    PlayerUtil::set_password(connection_num, &req.get_groups()[0]);

    // TODO パスワードのチェック。

    // ****************************************************************************************************
    // * 仮に ログインできたものとする。                                                                     *
    // ****************************************************************************************************

    // ログイン完了したら、待ち行列に追加しようぜ☆（＾ｑ＾）
    LobbyUtil::push_player(connection_num);


    // 応答メッセージ作成。
    let player_name;
    {
        player_name = PLAYER_MAP.try_read().unwrap().get(&connection_num).unwrap().get_name().to_string();
    }
    shell_var.set_message_to_client(&format!(
        r#"LOGIN:{} OK
"#, // 改行。
        player_name
    ));
}
