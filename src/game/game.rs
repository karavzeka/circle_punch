use super::{Arena};

use config::Config;
use GAME_CONF_PATH;

pub struct Game {
    pub arena: Option<Arena>,
}

impl Game {
    pub fn new() -> Game {
        Game {arena: None}
    }

    pub fn init(&mut self) {
        let config = Config::load(GAME_CONF_PATH);
        let arena = Arena::new(
            config.get_raw("width").unwrap().as_i64().unwrap() as i32,
            config.get_raw("height").unwrap().as_i64().unwrap() as i32
        );
        self.arena = Some(arena);
    }

//    pub fn get_arena(&mut self) -> &Arena {
//        match self.arena {
//            Some(ref mut arena) => arena,
//            None => {
//                panic!("Game is not initialized");
//            }
//        }
//    }
}