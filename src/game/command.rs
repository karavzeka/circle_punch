use chrono::prelude::Utc;

/// Команда игрока для сервера
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCmd {
//    pub time: u64,
//    #[serde(default = "default_player_id")]
    #[serde(default)]
    pub player_id: String,
    #[serde(default)]
    pub shot: bool,
    #[serde(default)]
    pub move_vector: MoveVector,
}

/// Команда сервера для игрока
#[derive(Serialize, Deserialize, Clone)]
pub struct ServerCmd {
    pub time: u64,
    pub players: Vec<PlayerCmd>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disconnected_players: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerCmd {
    pub player_id: String,
    pub it_is_you: bool,

    #[serde(skip_serializing_if = "Position::is_stay")]
    pub position: Position,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Одна сетевая команда может передать 0, 1 или -1 по каждой из осей.
/// При этом если за такт пришло от игрока 2 команды (сетевые задержки), то они суммируются.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MoveVector {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

impl Position {
    fn is_stay(&self) -> bool {
        match (self.x, self.y) {
            //TODO danger! Floating point accurately compare with zero!
            (0.0, 0.0) => true,
            _ => false
        }
    }
}

impl ServerCmd {
//    pub fn new() -> CommandOut {
//        let now = Utc::now();
//        let ts = (now.timestamp() * 1_000) as u64 + now.timestamp_subsec_millis() as u64;
//
//        CommandOut {
//            time: ts,
//            players: Vec::new(),
//        }
//    }

    pub fn add_player_cmd(&mut self, cmd: PlayerCmd) {
        self.players.push(cmd);
    }
}