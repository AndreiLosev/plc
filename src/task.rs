mod task_errors;

use std::time::{Duration, Instant};
use std::error;
use std::result;
use rmodbus::server::context::ModbusContext;
use task_errors::{TaskTimeOutError, TaskOtherError};

pub trait Program {
    fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>>;
}

pub struct Task {
    name: &'static str,
    programs: Vec<Box<dyn Program>>,
    priority: u8,
    event: Event,
    work_time: Option<Instant>,
}

#[derive(Clone, Copy)]
pub enum Event {
    Cycle(Duration),
    Background,
    CoilsFront(u16),
    Interrupt(&'static str),
}

impl Task {
    pub fn new(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
        event: Event,
    ) -> Self { Self { name, programs, priority, event, work_time: None } }

    pub fn get_priority(&self) -> u8 { self.priority }

    pub fn get_event(&self) -> Event { self.event }

    pub fn get_name(&self) -> &str { self.name }

    pub fn run(&mut self, context: &mut ModbusContext, task_num: usize) -> result::Result<usize, Box<dyn error::Error>> {

        if task_num > self.programs.len() {
            let e = TaskOtherError::new("Task::run(), task_num > self.programs.len()");
            return Err(Box::new(e));
        }

        if task_num == 0 {
            self.work_time = Some(Instant::now());
        }

        self.programs[task_num].run(context)?;

        if task_num == self.programs.len() {
            self.stop_time_work()?;
            return Ok(0);
        }

        Ok(task_num + 1)
    }

    fn stop_time_work(&mut self) -> result::Result<(), Box<dyn error::Error>> {
        let set_time = match self.event {
            Event::Cycle(v) => v,
            _ => Duration::from_millis(250),
        };

        let work_time = self.work_time;
        self.work_time = None;

        match work_time {
            Some(v) => {
                if v.elapsed() > set_time {
                    let e = TaskTimeOutError::new(
                        v.elapsed(),
                        self.name,
                        set_time
                    ); 
                    return Err(Box::new(e));
                }

                Ok(())
            },
            None => Err(Box::new(TaskOtherError::new(
                "Task::stop_time_work(), self.work_time is Null",
            ))),
        }
    }

}
