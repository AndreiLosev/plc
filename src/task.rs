mod task_errors;

#[path = "pls_std.rs"]
mod pls_std;

use std::time::{Duration, Instant};
use std::error;
use std::result;
use rmodbus::server::context::{ModbusContext};
use task_errors::{TaskTimeOutError, TaskOtherError};
use pls_std::bitword::BitWord;

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
    DiscreteInputFront((u16, u8)),
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

    pub fn new_front_discrete_input(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
        bit_addr: u16,
    ) -> Self {
        Self { name, programs, priority, event: Event::DiscreteInputFront((bit_addr, 4)) }
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
            self.after_start();
        }

        self.programs[task_num].run(context)?;

        if task_num == (self.programs.len() - 1) {
            self.before_complit()?;
            return Ok(0);
        }

        Ok(task_num + 1)
    }

    pub fn need_run(&mut self, context: &ModbusContext) -> result::Result<bool, Box<dyn error::Error>> {
        match &mut self.event {
            Event::Cycle((t, i)) => Ok(*t <= i.elapsed()),
            Event::DiscreteInputFront((addr, b)) => {
                let first = b.get_bit(0)?;
                let bit = context.get_discrete(*addr)?;

                b.set_bit(1, first)?;
                b.set_bit(0, bit)?;

                let first = b.get_bit(0)?;
                let second = b.get_bit(1)?;
                let third = b.get_bit(2)?;

                Ok(first && !second && third)
            },
            Event::Background => Ok(false),
        }
    }

    fn after_start(&mut self) {
        match &mut self.event {
            Event::Cycle((_, i)) => { *i = Instant::now(); },
            Event::DiscreteInputFront((_, b)) => { b.set_bit(2, false).unwrap(); },
            Event::Background => (),
        }
    }

    fn before_complit(&mut self) -> result::Result<(), Box<dyn error::Error>> {
        match &mut self.event {
            Event::DiscreteInputFront((_, b)) => {
                b.set_bit(2, true).unwrap();
                Ok(())
            },
            Event::Cycle((t, i)) => {
                if i.elapsed() > *t {
                    return Err(Box::new(TaskTimeOutError::new(i.elapsed(), self.name, *t)));
                }
                Ok(())
            },
            Event::Background => Ok(()) 
        }
    }

}
