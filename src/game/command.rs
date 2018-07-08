use chrono::prelude::Utc;
use cgmath::{Point2};

const CMD_TYPE_PLAYERS: &str = "players";
const CMD_TYPE_MAP: &str = "map";
const CMD_TYPE_GUEST_ID: &str = "guest_id";
const CMD_TYPE_BAD_REG: &str = "bad_reg";
//const CMD_TYPE_GOOD_REG: &str = "good_reg";

trait CmdType<'a> {
    fn get_cmd_type() -> &'a str;
}

/// Команда сервера для игрока
#[derive(Serialize, Deserialize, Clone)]
pub struct ServerCmd<'a> {
    pub cmd_type: &'a str,
    pub time: u64,
    pub players: Vec<PlayerCmd>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub waves: Vec<WaveCmd>,
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
            waves: Vec::new(),
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
    pub nickname: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WaveCmd {
    id: i64,
    position: Position,
    r: f32,
    is_dead: bool
}

impl WaveCmd {
    pub fn new(id: i64, x: f32, y: f32, r: f32, is_dead: bool) -> WaveCmd {
        WaveCmd {
            id,
            position: Position {x, y},
            r,
            is_dead
        }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GuestIdCmd<'a> {
    pub cmd_type: &'a str,
    pub guest_id: String
}

impl<'a> CmdType<'a> for GuestIdCmd<'a> {
    fn get_cmd_type() -> &'a str {
        CMD_TYPE_GUEST_ID
    }
}

impl<'a> GuestIdCmd<'a> {
    pub fn new(guest_id: String) -> GuestIdCmd<'a> {
        GuestIdCmd {
            cmd_type: GuestIdCmd::get_cmd_type(),
            guest_id
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BadRegistrationCmd<'a> {
    pub cmd_type: &'a str,
    pub message: String
}

impl<'a> CmdType<'a> for BadRegistrationCmd<'a> {
    fn get_cmd_type() -> &'a str {
        CMD_TYPE_BAD_REG
    }
}

impl<'a> BadRegistrationCmd<'a> {
    pub fn new(message: String) -> BadRegistrationCmd<'a> {
        BadRegistrationCmd {
            cmd_type: BadRegistrationCmd::get_cmd_type(),
            message
        }
    }
}

//#[derive(Serialize, Deserialize, Debug)]
//pub struct GoodRegistrationCmd<'a> {
//    pub cmd_type: &'a str,
//    pub nickname: String
//}
//
//impl<'a> CmdType<'a> for GoodRegistrationCmd<'a> {
//    fn get_cmd_type() -> &'a str {
//        CMD_TYPE_GOOD_REG
//    }
//}
//
//impl<'a> GoodRegistrationCmd<'a> {
//    pub fn new(nickname: String) -> GoodRegistrationCmd<'a> {
//        GoodRegistrationCmd {
//            cmd_type: GoodRegistrationCmd::get_cmd_type(),
//            nickname
//        }
//    }
//}

//#[derive(Serialize, Deserialize, Debug)]
//pub struct RegisterPlayerCmd {
//    pub guest_id: String,
//    pub nickname: String,
//}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd_type")]
pub enum IncomingCmdType {
    RegisterPlayer {nickname: String},
    Move {x: i16, y: i16},
    Attack
}

/// Команда игрока для сервера
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCmd {
    #[serde(default)]
    pub player_id: String,
    pub cmd: IncomingCmdType,
}

enum OutgoingCmd {

}