mod task_error;

use std::time::{Duration, Instant};
use std::error;
use std::result;
use rmodbus::server::context::ModbusContext;
use task_error::TaskTimeOutError;

pub trait Program {
    fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>>;
}

pub struct Task {
    name: &'static str,
    programs: [Box<dyn Program>; 32],
    priority: u8,
    event: Event,
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
        programs: [Box<dyn Program>; 32],
        priority: u8,
        event: Event,
    ) -> Self { Self { name, programs, priority, event } }

    pub fn get_priority(&self) -> u8 { self.priority }

    pub fn get_event(&self) -> Event { self.event }

    pub fn get_name(&self) -> &str { self.name }

    pub fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {

        let cycle_event = self.run_cycle_event_timer();

        for prog in self.programs.iter_mut() {
            prog.run(context)?
        }

        Ok(())

        // match cycle_event {
        //     Some((time_left, set_time)) => {
        //         if time_left.elapsed() > set_time {
        //             let e = TaskTimeOutError::new(
        //                 time_left.elapsed(),
        //                 self.name,
        //                 set_time
        //             ); 
        //             Err(e)
        //         } else {
        //             Ok(())
        //         }
        //     }
        //     _ => Ok(())
        // }
    }

    fn run_cycle_event_timer(&self) -> Option<(Instant, Duration)> {
        let set_time = match self.event {
            Event::Cycle(v) => Some(v),
            _ => None,
        };

        let start_time = match set_time {
            Some(_) => Some(Instant::now()),
            None => None,
        };

        if set_time.is_none() || start_time.is_none() {
            return None;
        }

        Some((start_time.unwrap(), set_time.unwrap()))
    }
}
