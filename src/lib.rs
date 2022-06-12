pub mod task;
pub mod pls_std;
mod config;

use rmodbus::server::context::ModbusContext;
use ansi_term::Color::Red;
use std::{env, fs, result, error};

struct TaskLink {
    task_priority: u8,
    task_position: u8,
}

pub struct Plc {
    task_event: Vec<task::Task>,
    bacground: Vec<task::Task>,
    context: ModbusContext,
    config: Config,
    call_stack: Vec<task::Task>
}

impl Plc {
    pub fn new(tasks: Vec<task::Task>) -> Self {

        let config = Self::config_adapter(Self::read_config()); 
        let context = ModbusContext::new();

        let mut task_event: Vec<task::Task> = Vec::new();
        let mut bacground: Vec<task::Task> = Vec::new();

        for task in tasks {
            match task.get_event() {
                task::Event::Cycle(_) => task_event.push(task),
                task::Event::DiscreteInputFront(_) => task_event.push(task),
                task::Event::Background => bacground.push(task),
            }
        }

        bacground.sort_unstable();

        Self { task_event, bacground, context, config, call_stack: Vec::new() }
    }

    pub fn run(&mut self) {    
        loop {
            match self.set_call_stack() {
                Err(e) => println!("err : {}", e),
                _ => (),
            };

            let result = match self.call_stack.last_mut() {
                Some(task) => match task.run(&mut self.context) {
                    Ok(n) => n,
                    Err(e) => {
                        println!("err: {}", e);
                        0
                    }
                },
                None => continue,
            };

            if result == 0 {
                let task = self.call_stack.remove(self.call_stack.len() - 1);
                match task.get_event() {
                    task::Event::Background => self.bacground.push(task),
                    _ => self.task_event.push(task)
                }
            }

        }
    }

    fn set_call_stack(&mut self) -> result::Result<(), Box<dyn error::Error>> {
        for i in 0..self.task_event.len() {

            let need_run = self.task_event[i].need_run(&self.context)?;

            if need_run {
                self.call_stack.push(self.task_event.swap_remove(i));
            }
        }

        if self.call_stack.is_empty() {
            let n = self.bacground.len();
            if let Some(_) = self.bacground.get(n) {
                self.call_stack.push(self.bacground.remove(n));   
            }
        }

        self.call_stack.sort_unstable();

        Ok(())
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
