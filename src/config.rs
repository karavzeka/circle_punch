use std::env;
use jconfig;

pub struct Config;

impl Config {
    /// Return 'jsconfig' object
    pub fn load(path: &str) -> jconfig::Config {
        let exe_path = env::current_exe().unwrap().parent().unwrap().to_owned();
        let config_path = &exe_path.join(path);
        let config = jconfig::Config::load(config_path).unwrap();
        config
    }
}