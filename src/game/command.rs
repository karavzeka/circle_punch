use chrono::prelude::Utc;

/// Команда игрока для сервера
#[derive(Serialize, Deserialize)]
pub struct CommandIn {
    pub time: u64,
    pub player_id: String,
    pub move_vector: MoveTo,
}

/// Команда сервера для игрока
#[derive(Serialize, Deserialize, Clone)]
pub struct CommandOut {
    pub time: u64,
    pub players: Vec<PlayerCmd>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerCmd {
    pub player_id: String,
    pub it_is_you: bool,

    #[serde(skip_serializing_if = "MoveTo::is_stay")]
    pub move_vector: MoveTo,
}

/// Одна сетевая команда может передать 0, 1 или -1 по каждой из осей.
/// При этом если за такт пришло от игрока 2 команды (сетевые задержки), то они суммируются.
#[derive(Serialize, Deserialize, Clone)]
pub struct MoveTo {
    pub x: f32,
    pub y: f32,
}
//TODO add to CommandIn
//pub struct MoveTo {
//    pub x: f32,
//    pub y: f32,
//}
//TODO rename to Position
impl MoveTo {
    fn is_stay(&self) -> bool {
        match (self.x, self.y) {
            //TODO danger! Floating point accurately compare with zero!
            (0.0, 0.0) => true,
            _ => false
        }
    }
}

impl CommandOut {
    pub fn new() -> CommandOut {
        let now = Utc::now();
        let ts = (now.timestamp() * 1_000) as u64 + now.timestamp_subsec_millis() as u64;

        CommandOut {
            time: ts,
            players: Vec::new(),
        }
    }

    pub fn add_player_cmd(&mut self, cmd: PlayerCmd) {
        self.players.push(cmd);
    }
}