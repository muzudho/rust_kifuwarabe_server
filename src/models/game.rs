//use std::collections::HashMap;
use models::player::*;

pub enum Turn {
    /** 先手 */
    Black,
    /** 後手 */
    White
}

pub struct Game{
    /// 対局発生順番号。
    number: i64,
    /// 対局識別名。
    name: String,

    /// 先手プレイヤー。
    player0: Player,
    /// 後手プレイヤー。
    player1: Player,

    turn: Turn,

    // 汎用的に利用できるハッシュマップ。
    // pub properties: HashMap<String, String>,

    //#[allow(dead_code)]
    //pub game_summary: GameSummary,
}
impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
impl Game{
    pub fn new() -> Game {
        Game {
            number: -1,
            name: "".to_string(),
            player0: Player::new(),
            player1: Player::new(),
            turn: Turn::Black,
            //properties: HashMap::new(),
            //game_summary: GameSummary::new(),
        }
    }

    pub fn get_number(&self) -> i64 {
        self.number
    }
    pub fn set_number(&mut self, value:i64) {
        self.number = value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, value:String) {
        self.name = value
    }

    pub fn get_player0(&self) -> &Player {
        &self.player0
    }
    pub fn get_mut_player0(&mut self) -> &mut Player {
        &mut self.player0
    }
    pub fn set_player0(&mut self, value:Player) {
        self.player0 = value
    }

    pub fn get_player1(&self) -> &Player {
        &self.player1
    }
    pub fn get_mut_player1(&mut self) -> &mut Player {
        &mut self.player1
    }
    pub fn set_player1(&mut self, value:Player) {
        self.player1 = value
    }

    pub fn get_turn(&self) -> &Turn {
        &self.turn
    }
    pub fn set_turn(&mut self, value:Turn) {
        self.turn = value
    }

    /// CSAプロトコルの Game_Summary
    pub fn to_game_summary_string_ln(&self) -> String {
        format!(r#"BEGIN Game_Summary
Protocol_Version:1.2
Protocol_Mode:Server
Format:Shogi 1.0
Declaration:Jishogi 1.1
Game_ID:{}
Name+:{}
Name-:{}
Your_Turn:{}
Rematch_On_Draw:NO
To_Move:+
Max_Moves:256
BEGIN Time
Time_Unit:1sec
Total_Time:600
Byoyomi:10
Least_Time_Per_Move:1
END Time
BEGIN Position
P1-KY-KE-GI-KI-OU-KI-GI-KE-KY
P2 * -HI *  *  *  *  * -KA * 
P3-FU-FU-FU-FU-FU-FU-FU-FU-FU
P4 *  *  *  *  *  *  *  *  * 
P5 *  *  *  *  *  *  *  *  * 
P6 *  *  *  *  *  *  *  *  * 
P7+FU+FU+FU+FU+FU+FU+FU+FU+FU
P8 * +KA *  *  *  *  * +HI * 
P9+KY+KE+GI+KI+OU+KI+GI+KE+KY
P+
P-
+
END Position
END Game_Summary
"#  // 最後に改行が必要。
        , self.name
        , self.get_player0().get_name()
        , self.get_player1().get_name()
        , match self.turn {
            Turn::Black => "+",
            Turn::White => "-",
        }
        )
    }
}