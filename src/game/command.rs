use chrono::prelude::Utc;

/// Команда игрока для сервера
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCmd {
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
    pub x: i16,
    #[serde(default)]
    pub y: i16,
}

impl Position {
    fn is_stay(&self) -> bool {
        match (self.x.is_normal(), self.y.is_normal()) {
            (false, false) => true, // zero
            _ => false
        }
    }
}