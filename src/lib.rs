pub mod task;
pub mod pls_std;
pub mod system_prog;

mod config;

pub use rmodbus as rmodbus;
use rmodbus::server::context::ModbusContext;
// use ansi_term::Color::Red;
use std::{result, error};

pub struct Plc {
    task_event: Vec<task::Task>,
    bacground: Vec<task::Task>,
    context: ModbusContext,
    _config: Config,
    call_stack: Vec<task::Task>
}

impl Plc {
    pub fn new(tasks: Vec<task::Task>) -> Self {

        let _config = Self::config_adapter(Self::read_config()); 
        let context = ModbusContext::new();

        let mut task_event: Vec<task::Task> = Vec::new();
        let mut bacground: Vec<task::Task> = Vec::new();

        for task in tasks {
            match task.get_event() {
                task::Event::Cycle(_) => task_event.push(task),
                task::Event::BitFront(_) => task_event.push(task),
                task::Event::Background => bacground.push(task),
            }
        }

        bacground.sort_unstable();

        Self { task_event, bacground, context, _config, call_stack: Vec::new() }
    }

    pub fn run(&mut self) {    
        loop {

            if let Err(e) = self.set_call_stack() {
                println!("err : {}", e)
            }

            let result = match self.call_task() {
                Ok(v) => v,
                Err(e) => {
                    println!("err: {}", e);
                    0
                }
            };

            if result != 0 {
                continue;
            }

            self.return_task();

        }
    }

    fn set_call_stack(&mut self) -> result::Result<(), Box<dyn error::Error>> {

        let mut need_run_index: Vec<usize> = Vec::with_capacity(self.task_event.len());

        for i in 0..self.task_event.len() {

            let need_run = self.task_event[i].need_run(&self.context)?;

            if need_run {
                need_run_index.push(i);
            }
        }

        for i in need_run_index {
            self.call_stack.push(self.task_event.swap_remove(i));
        }

        if self.call_stack.is_empty() && self.bacground.get(0).is_some() {
            self.call_stack.push(self.bacground.remove(0));
        }

        self.call_stack.sort_unstable();

        Ok(())
    }

    fn call_task(&mut self) -> result::Result<u8, Box<dyn error::Error>> {
        let result = match self.call_stack.first_mut() {
            Some(task) => task.run(&mut self.context)?,
            None => u8::MAX,
        };

        Ok(result)
    }

    fn return_task(&mut self) {
        if self.call_stack.get(0).is_none() {
            return;
        }

        let task = self.call_stack.remove(0);
        match task.get_event() {
            task::Event::Background => self.bacground.push(task),
            _ => self.task_event.push(task)
        }

    }

    fn read_config() -> config::General {
        // let mut work_dir = env::current_dir().unwrap();
        // work_dir.push("config");
        // work_dir.push("gпeneral.yaml");
        // dbg!(&work_dir);
        // let config_as_string = fs::read_to_string(work_dir)
        //     .unwrap_or_else(|e| panic!(
        //         "err: {}, doc: {}",
        //         &Red.paint(format!("{}", e)),
        //         &Red.paint("general config file not found in ${workingdirectory}/config/general.yaml"),
        //     ));
        
        // let config: config::General = serde_yaml::from_str(&config_as_string)
        //     .unwrap_or_else(|e| panic!(
        //         "err: {}, doc: {}",
        //         &Red.paint("general config file ${workingdirectory}/config/general.yaml not valid yaml"),
        //         &Red.paint(format!("{}", e))
        //     ));
        
        config::General{}
    }

    fn config_adapter(_yaml_config: config::General) -> Config {
        Config {}
    }
}

struct Config {}

#[test]
fn test_plc() {

}
