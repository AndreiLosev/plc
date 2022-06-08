mod task_errors;

#[path = "pls_std.rs"]
mod pls_std;

use std::time::{Duration, Instant};
use std::error;
use std::result;
use rmodbus::server::context::{ModbusContext};
use rmodbus::ErrorKind;
use task_errors::{TaskTimeOutError, TaskOtherError};
use pls_std::bitword;

pub trait Program {
    fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>>;
}

pub struct Task {
    name: &'static str,
    programs: Vec<Box<dyn Program>>,
    priority: u8,
    event: Event,
}

#[derive(Clone, Copy)]
pub enum Event {
    Cycle((Duration, Instant)),
    Background,
    DiscreteInputFront((u16, bool)),
}

impl Task {

    pub fn new_cycle(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
        cycle: Duration,
    ) -> Self {
        Self { name, programs, priority, event: Event::Cycle((cycle, Instant::now())) }
    }

    pub fn new_discrete_input_front(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
        bit_addr: u16,
    ) -> Self {
        Self { name, programs, priority, event: Event::DiscreteInputFront((bit_addr, false)) }
    }

    pub fn new_background(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
    ) -> Self {
        Self { name, programs, priority, event: Event::Background }
    }

    pub fn get_priority(&self) -> u8 { self.priority }

    pub fn get_event(&self) -> Event { self.event }

    pub fn get_name(&self) -> &str { self.name }

    pub fn get_program_count(&self) -> usize { self.programs.len() }

    pub fn run(
        &mut self,
        context: &mut ModbusContext,
        task_num: usize,
    ) -> result::Result<usize, Box<dyn error::Error>> {

        if task_num > (self.programs.len() - 1) {
            let e = TaskOtherError::new("Task::run(), task_num > self.programs.len()");
            return Err(Box::new(e));
        }

        if task_num == 0 {
            self.task_start();
        }

        self.programs[task_num].run(context)?;

        if task_num == (self.programs.len() - 1) {
            return Ok(0);
        }

        Ok(task_num + 1)
    }

    fn task_start(&mut self) {
        match &mut self.event {
            Event::Cycle((_, i)) => { *i = Instant::now(); },
            Event::DiscreteInputFront((_, b)) => { *b = true; },
            Event::Background => (),
        }
    }

    pub fn launch_now(&mut self, context: &ModbusContext) -> result::Result<bool, ErrorKind> {
        match &mut self.event {
            Event::Cycle((t, i)) => Ok(*t <= i.elapsed()),
            Event::DiscreteInputFront((a, b)) => {
                let discrets_input = context.get_discrete(*a)?;
                Ok(discrets_input && !*b)
            },
            Event::Background => Ok(false),
        }
    }

}
