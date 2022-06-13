mod task_errors;

#[path = "pls_std.rs"]
mod pls_std;

use std::time::{Duration, Instant};
use std::{error, result, cmp, fmt};
use rmodbus::server::context::{ModbusContext};
use task_errors::{TaskTimeOutError};
use pls_std::bitword::BitWord;

pub trait Program: fmt::Debug {
    fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>>;
}

#[derive(Debug)]
pub struct Task {
    name: &'static str,
    programs: Vec<Box<dyn Program>>,
    priority: u8,
    event: Event,
    next_program: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Cycle((Duration, Instant)),
    Background,
    DiscreteInputFront((u16, u8)),
}

impl cmp::PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        let first = match self {
            Self::Background => 2,
            _ => 1,
        };

        let second = match other {
            Self::Background => 2,
            _ => 1,
        };

        first == second
    }
}

impl cmp::PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let first = match self {
            Self::Background => 2,
            _ => 1,
        };

        let second = match other {
            Self::Background => 2,
            _ => 1,
        };

        first.partial_cmp(&second)

    }
}

impl Task {

    pub fn new_cycle(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
        cycle: Duration,
    ) -> Self {
        Self {
            name,
            programs,
            priority,
            event: Event::Cycle((cycle, Instant::now())),
            next_program: 0,
        }
    }

    pub fn new_front_discrete_input(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
        bit_addr: u16,
    ) -> Self {
        Self {
            name,
            programs,
            priority,
            event: Event::DiscreteInputFront((bit_addr, 4)),
            next_program: 0,
        }
    }

    pub fn new_background(
        name: &'static str,
        programs: Vec<Box<dyn Program>>,
        priority: u8,
    ) -> Self {
        Self {
            name,
            programs,
            priority,
            event: Event::Background,
            next_program: 0,
        }
    }

    pub fn get_priority(&self) -> u8 { self.priority }

    pub fn get_event(&self) -> Event { self.event }

    pub fn get_name(&self) -> &str { self.name }

    pub fn get_program_count(&self) -> usize { self.programs.len() }

    pub fn run(
        &mut self,
        context: &mut ModbusContext,
    ) -> result::Result<u8, Box<dyn error::Error>> {

        if self.next_program == 0 {
            self.after_start();
        }

        self.programs[self.next_program as usize].run(context)?;

        self.next_program = (self.next_program + 1) % self.programs.len() as u8;

        if self.next_program == 0 {
            self.before_complit()?;
        }

        Ok(self.next_program)
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

impl cmp::PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.get_priority() == other.get_priority()
        && self.get_event() == other.get_event()
    }
}

impl cmp::PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {

        let event_comparete = self.get_event()
            .partial_cmp(&other.get_event());
            
        if let Some(i) = event_comparete {
            return match i {
                cmp::Ordering::Equal => {
                    let first = self.get_priority();
                    let second = other.get_priority();
                    first.partial_cmp(&second)
                },
                _ => Some(i),    
            };
        }

        event_comparete
    }
}

impl cmp::Eq for Task {}

impl  cmp::Ord for Task {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


#[test]
fn test_task_sort() {
    
    let mut result = vec![
        Task::new_background("", vec![], 5),
        Task::new_background("", vec![], 9),
        Task::new_background("", vec![], 2),
        Task::new_background("", vec![], 4),
        Task::new_cycle("", vec![], 4, Duration::ZERO),
        Task::new_cycle("", vec![], 7, Duration::ZERO),
        Task::new_cycle("", vec![], 1, Duration::ZERO),
        Task::new_cycle("", vec![], 9, Duration::ZERO),
        Task::new_front_discrete_input("", vec![], 3, 1),
        Task::new_front_discrete_input("", vec![], 8, 1),
        Task::new_front_discrete_input("", vec![], 4, 1),
        Task::new_front_discrete_input("", vec![], 5, 1),
        Task::new_front_discrete_input("", vec![], 1, 1),
    ];

    let expect = vec![
        Task::new_front_discrete_input("", vec![], 1, 1),
        Task::new_cycle("", vec![], 1, Duration::ZERO),
        Task::new_front_discrete_input("", vec![], 3, 1),
        Task::new_front_discrete_input("", vec![], 4, 1),
        Task::new_cycle("", vec![], 4, Duration::ZERO),
        Task::new_front_discrete_input("", vec![], 5, 1),
        Task::new_cycle("", vec![], 7, Duration::ZERO),
        Task::new_front_discrete_input("", vec![], 8, 1),
        Task::new_cycle("", vec![], 9, Duration::ZERO),
        Task::new_background("", vec![], 2),
        Task::new_background("", vec![], 4),
        Task::new_background("", vec![], 5),
        Task::new_background("", vec![], 9),
    ];

    result.sort();

    assert_eq!(result, expect);
    
}