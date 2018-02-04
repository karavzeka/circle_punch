use chrono::prelude::Utc;
use cgmath::{Point2};

const CMD_TYPE_PLAYERS: &str = "players";
const CMD_TYPE_MAP: &str = "map";

trait CmdType<'a> {
    fn get_cmd_type() -> &'a str;
}

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
pub struct ServerCmd<'a> {
    pub cmd_type: &'a str,
    pub time: u64,
    pub players: Vec<PlayerCmd>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disconnected_players: Vec<String>,
}

impl<'a> ServerCmd<'a> {
    pub fn new() -> ServerCmd<'a> {
        let now = Utc::now();
        let ts = (now.timestamp() * 1_000) as u64 + now.timestamp_subsec_millis() as u64;
        ServerCmd {
            cmd_type: ServerCmd::get_cmd_type(),
            time: ts,
            players: Vec::new(),
            disconnected_players: Vec::new(),
        }
    }
}

impl<'a> CmdType<'a> for ServerCmd<'a> {
    fn get_cmd_type() -> &'a str {
        CMD_TYPE_PLAYERS
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerCmd {
    pub player_id: String,
    pub it_is_you: bool,

    #[serde(skip_serializing_if = "Position::is_stay")]
    pub position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_max: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MapCmd<'a> {
    pub cmd_type: &'a str,
    pub width: u32,
    pub height: u32,
    pub walls: Vec<WallCmd>,
    pub spikes: Vec<SpikeCmd>,
}

impl<'a> MapCmd<'a> {
    pub fn new(width: u32, height:u32) -> MapCmd<'a> {
        MapCmd {
            cmd_type: MapCmd::get_cmd_type(),
            width,
            height,
            walls: Vec::new(),
            spikes: Vec::new(),
        }
    }
}

impl<'a> CmdType<'a> for MapCmd<'a> {
    fn get_cmd_type() -> &'a str {
        CMD_TYPE_MAP
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WallCmd {
    pub position: Position,
    pub edge_size: f32,
}

impl WallCmd {
    pub fn new(x: f32, y: f32) -> WallCmd {
        WallCmd {
            position: Position {x, y},
            edge_size: super::EDGE_SIZE,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpikeCmd {
    pub draw_body: Line,
    pub danger_body: Line,
    pub normal: Position,
    pub vec_along_spike: Position,
    pub height: f32,
    pub needle_half_width: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Line {
    pub point_1: Position,
    pub point_2: Position,
}

impl Position {
    fn is_stay(&self) -> bool {
        match (self.x.is_normal(), self.y.is_normal()) {
            (false, false) => true, // zero
            _ => false
        }
    }
}