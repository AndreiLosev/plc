pub mod task;
pub mod pls_std;
pub mod system_prog;


mod config;

pub use rmodbus::server::context::ModbusContext;

use ansi_term::Color::Red;
use ansi_term::ANSIGenericString;
use std::{result, error};

pub struct Plc<'a> {
    task_event: Vec<task::Task<'a>>,
    bacground: Vec<task::Task<'a>>,
    context: ModbusContext,
    call_stack: Vec<task::Task<'a>>,
}

impl<'a> Plc<'a> {
    pub fn new<const N: usize>(tasks: [task::Task<'a>; N]) -> Self {

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

        Self { task_event, bacground, context, call_stack: Vec::new() }
    }

    pub fn run(&mut self) {    
        loop {

            if let Err(e) = self.set_call_stack() {
                error_log(e, None);
            }

            let result = match self.call_task() {
                Ok(v) => v,
                Err(e) => {
                    let task_name = match self.call_stack.first() {
                        Some(t) => t.get_name(),
                        None => "empty call stack",
                    };
                    error_log(e, Some(task_name));
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

}

fn error_log(e:  Box<dyn error::Error>, task: Option<&str>) {
    match task {
        Some(t) => println!("{}", Red.paint(format!("task: {}, err: {}", t, e))),
        None => println!("{}", Red.paint(format!("err: {}", e))),
    }
}

fn fail_strig(e: & dyn error::Error) -> ANSIGenericString<str> {
    Red.paint(format!("err: {}", e))
}