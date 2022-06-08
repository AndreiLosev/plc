pub mod task;
pub mod pls_std;
mod config;

use rmodbus::server::context::ModbusContext;
use ansi_term::Color::Red;
use std::{env, fs, time};

pub struct Plc {
    tasks: Vec<task::Task>,
    context: ModbusContext,
    config: Config,
}

impl Plc {
    pub fn new(tasks: Vec<task::Task>) -> Self {

        let config = Self::config_adapter(Self::read_config()); 
        let context = ModbusContext::new();

        Self { tasks, context, config }
    }

    pub fn run() {
        
    }

    fn read_config() -> config::General {
        let mut work_dir = env::current_dir().unwrap();
        work_dir.push("config");
        work_dir.push("gпeneral.yaml");
        
        let config_as_string = fs::read_to_string(work_dir)
            .expect(&Red.paint("general config file not found in ${workingdirectory}/config/general.yaml").to_string());
        
        let config: config::General = serde_yaml::from_str(&config_as_string)
            .expect(&Red.paint("general config file ${workingdirectory}/config/general.yaml not valid yaml").to_string());
        
        config
    }

    fn config_adapter(yaml_config: config::General) -> Config {
        Config {}
    }
}

struct Config {}

#[test]
fn test_plc() {

}
