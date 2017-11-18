use chrono::prelude::Utc;

/// Команда игрока
#[derive(Serialize, Deserialize)]
pub struct Command {
    pub time: u64,
    players: Vec<PlayerCmd>
}

#[derive(Serialize, Deserialize)]
pub struct PlayerCmd {
    player_id: String,

    #[serde(skip_serializing_if = "MoveTo::is_stay")]
    pub move_vector: MoveTo,
}

/// Одна сетевая команда может передать 0, 1 или -1 по каждой из осей.
/// При этом если за такт пришло от игрока 2 команды (сетевые задержки), то они суммируются.
#[derive(Serialize, Deserialize)]
pub struct MoveTo {
    pub x: i16,
    pub y: i16,
}

impl MoveTo {
    fn is_stay(&self) -> bool {
        match (self.x, self.y) {
            (0, 0) => true,
            _ => false
        }
    }
}

impl Command {
    pub fn new() -> Command {
        let now = Utc::now();
        let ts = (now.timestamp() * 1_000) as u64 + now.timestamp_subsec_millis() as u64;

        Command {
            time: ts,
            players: Vec::new(),
        }
    }

    pub fn add_player_cmd(&mut self, cmd: PlayerCmd) {
        self.players.push(cmd);
    }
}