use kifuwarabe_server::interfaces::*;
use kifuwarabe_shell::shell::*;
use models::game::*;
use models::shell_var::*;
use shell_impl::DIAGRAM;
use std::collections::HashMap;
use std::collections::VecDeque;
use utils::game_utils::*;
use utils::player_utils::*;
use utils::shell_map_utils::*;
use utils::shell_var_utils::*;

pub const DIAGRAM_JSON_FILE: &str = "diagram.json";

/// ロビー。マッチングするためのもの。
#[derive(Default)]
pub struct Lobby {
    // 余ってるプレイヤー番号。
    pub waiting_players: VecDeque<i64>,
}
impl Lobby {
    pub fn new() -> Lobby {
        Lobby {
            waiting_players: VecDeque::new(),
        }
    }
}

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    /// 待機しているプレイヤー番号など。
    pub static ref LOBBY: RwLock<Lobby> = RwLock::new(Lobby::new());

    /// クライアント（接続者番号）に対応づくオブジェクト。
    pub static ref PLAYER_MAP: RwLock<HashMap<i64,Player>> = RwLock::new(HashMap::new());

    /// <ShellVar>型引数が付くので、 PLAYER_MAP とは分けている。
    pub static ref SHELL_MAP: RwLock<HashMap<i64,Shell<ShellVar>>> = RwLock::new(HashMap::new());

    /// クライアントに対応づく ShellVar。
    pub static ref SHELL_VAR_MAP: RwLock<HashMap<i64,ShellVar>> = RwLock::new(HashMap::new());

    /// 対局部屋相当。プレイヤー２つが対応付く。
    pub static ref GAME_MAP: RwLock<HashMap<i64,Game>> = RwLock::new(HashMap::new());
}

#[derive(Default)]
pub struct ServerController {}
impl ServerController {
    pub fn new() -> ServerController {
        ServerController {}
    }
}

/// クライアント１つに対応づく。
/// クライアントからの接続があったとき。
/// 対局待ちキューに、その接続に番号を入れる。
pub fn on_coming_shogi(connection_number: i64) {
    println!("Welcome {}!", connection_number);

    // プレイヤー オブジェクトを与えようぜ☆（＾～＾）
    PlayerUtil::insert(connection_number, Player::new());

    // シェルを与えようぜ☆（＾～＾）
    ShellMapUtils::insert(connection_number, Shell::new());

    // シェルの内部状態変数を与えようぜ☆（＾～＾）
    ShellVarUtil::insert(connection_number, ShellVar::new());

    // まだ接続しただけで、ログインはしてないぜ☆（＾～＾）
}

/// クライアント１つに対応づく。
/// クライアントからの入力は このメソッド内で処理する。
pub fn on_received_from_client_shogi(req: &Request, res: &mut Response) {
    println!("<{} {}", req.get_connection_number(), req.get_message());

    match SHELL_MAP
        .try_write()
        .unwrap()
        .get_mut(&req.get_connection_number())
    {
        Some(shell) => {
            match SHELL_VAR_MAP
                .try_write()
                .unwrap()
                .get_mut(&req.get_connection_number())
            {
                Some(shell_var) => {
                    shell_var.set_connection_number(req.get_connection_number());

                    // コマンドラインは、ダイアグラムに従って パースさせるぜ☆（*＾～＾*）
                    shell_var.set_message_to_client("");
                    {
                        let mut diagram = DIAGRAM.try_write().unwrap();
                        println!("client_handle_impl.rs/on_received_from_client_shogi/ダイアグラムの入り口: [{}].", diagram.get_entry_point());
                        // ダイアグラムの入り口に遷移。
                        shell.enter(&diagram);
                        println!("client_handle_impl.rs/on_received_from_client_shogi/シェルのカレント: [{}].", shell.get_current());
                        // ********************************************************************************
                        // * コマンドライン解析。                                                           *
                        // ********************************************************************************
                        shell.execute_line(&mut diagram, shell_var, &req.get_message());
                        println!("client_handle_impl.rs/on_received_from_client_shogi/コマンドライン解析後のシェルのカレント: [{}].", shell.get_current());
                    }

                    let message_to_client = shell_var.get_message_to_client();

                    if &message_to_client.to_string() != "" {
                        // 応答メッセージ。
                        res.set_message(&message_to_client);
                    } else {
                        println!(
                            "<{} Nothing response message to client: [{}]",
                            req.get_connection_number(),
                            req.get_message()
                        );
                    }
                }
                None => panic!("Not found in shell var map."),
            }
        }
        None => panic!("Not found in shell map."),
    }

    /* TODO
    match req.get_message() {
        "LOGIN kifuwarabe a" => {

        }
        r#"
AGREE"# => {
            res.set_message(&format!(
                r#"START:{}
"#, // 最後に改行が必要。
                GAME_MAP.try_read().unwrap()[&0].game_summary.get_game_id()
            ));
        }
    }
*/
}

/// クライアント１つに対応づく。
/// クライアントのいずれか１つが、サーバーからのメッセージを待っているタイミング。
pub fn on_send_to_client_shogi(connection_number: i64, res: &mut Response) {

    /*
    // TODO クライアントが starting 状態か？
    if PlayerUtil::is_state(connection_number, "starting") {
        println!("{} は、startingだ！", connection_number);
        // 相手が CSAプロトコルと決めつけて ゲームサマリーを送り付ける。

        // 接続者が入っている部屋番号。
        let game_num = PlayerUtil::get_entry_game(connection_number);
        println!("game_num: {}", game_num);

        // メッセージ作成。
        res.set_message(&GameUtil::get_game_summary_string(game_num));

        // 接続者のステータス変更。
        PlayerUtil::set_state(connection_number, &"isAgree".to_string());

        println!(
            "{} のステータスを変更したはず。",
            connection_number
        );
    }
    */

    // TODO クライアントに AGREE を送りたい？
}
