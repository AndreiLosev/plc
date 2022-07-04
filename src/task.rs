mod task_errors;

use std::time::{Duration, Instant};
use std::{error, result, cmp};
use rmodbus::server::context::{ModbusContext};
use task_errors::{TaskTimeOutError};
use super::pls_std::BitWord;

pub trait MutProgram {
    fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>>;
}

pub trait ConstProgram {
    fn run(&self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>>;
}

pub enum Program<'a> {
    Mut(&'a mut dyn MutProgram),
    Const(&'a dyn ConstProgram),
}

pub struct Task<'a> {
    name: &'static str,
    programs: &'a mut[Program<'a>],
    priority: u8,
    event: Event,
    next_program: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Cycle((Duration, Instant)),
    Background,
    BitFront((u16, u8)),
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

impl<'a> Task<'a> {

    pub fn new_cycle(
        name: &'static str,
        programs: &'a mut[Program<'a>],
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

    pub fn new_input_bit(
        name: &'static str,
        programs: &'a mut[Program<'a>],
        priority: u8,
        bit_addr: u16,
    ) -> Self {
        Self {
            name,
            programs,
            priority,
            event: Event::BitFront((bit_addr, 4)),
            next_program: 0,
        }
    }

    pub fn new_coli_bit(
        name: &'static str,
        programs: &'a mut[Program<'a>],
        priority: u8,
        bit_addr: u16,
    ) -> Self {
        Self {
            name,
            programs,
            priority,
            event: Event::BitFront((bit_addr, 132)),
            next_program: 0,
        }
    }

    pub fn new_background(
        name: &'static str,
        programs: &'a mut[Program<'a>],
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

        if let Some(Program::Const(v)) = self.programs.get(self.next_program as usize) {
            v.run(context)?;
        }

        if let Some(Program::Mut(v)) = self.programs.get_mut(self.next_program as usize) {
            v.run(context)?;
        }

        self.next_program = (self.next_program + 1) % self.programs.len() as u8;

        if self.next_program == 0 {
            self.before_complit()?;
        }

        Ok(self.next_program)
    }

    pub fn need_run(&mut self, context: &ModbusContext) -> result::Result<bool, Box<dyn error::Error>> {
        match &mut self.event {
            Event::Cycle((t, i)) => Ok(*t <= i.elapsed()),
            Event::BitFront((addr, b)) => {
                let first = b.get_bit(0)?;

                let bit = match b.get_bit(7).unwrap() {
                    true => context.get_coil(*addr)?,
                    false => context.get_discrete(*addr)?,
                };

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
            Event::BitFront((_, b)) => { b.set_bit(2, false).unwrap(); },
            Event::Background => (),
        }
    }

    fn before_complit(&mut self) -> result::Result<(), Box<dyn error::Error>> {
        match &mut self.event {
            Event::BitFront((_, b)) => {
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

impl<'a> cmp::PartialEq for Task<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_priority() == other.get_priority()
        && self.get_event() == other.get_event()
    }
}

impl<'a> cmp::PartialOrd for Task<'a> {
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

impl<'a> cmp::Eq for Task<'a> {}

impl<'a>  cmp::Ord for Task<'a> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


#[test]
fn test_task_sort() {
    
    let mut result = vec![
        Task::new_background("new_background: 5", &mut [], 5),
        Task::new_background("new_background: 9", &mut [], 9),
        Task::new_background("new_background: 2", &mut [], 2),
        Task::new_background("new_background: 4", &mut [], 4),
        Task::new_cycle("new_cycle: 4", &mut [], 4, Duration::ZERO),
        Task::new_cycle("new_cycle: 7", &mut [], 7, Duration::ZERO),
        Task::new_cycle("new_cycle: 1", &mut [], 1, Duration::ZERO),
        Task::new_cycle("new_cycle: 9", &mut [], 9, Duration::ZERO),
        Task::new_coli_bit("new_coli_bit: 3", &mut [], 3, 1),
        Task::new_input_bit("new_input_bit: 8", &mut [], 8, 1),
        Task::new_input_bit("new_input_bit: 4", &mut [], 4, 1),
        Task::new_coli_bit("new_coli_bit: 5", &mut [], 5, 1),
        Task::new_coli_bit("new_coli_bit: 1", &mut [], 1, 1),
    ];

    let expect = vec![
        Task::new_cycle("new_cycle: 1", &mut [], 1, Duration::ZERO),
        Task::new_coli_bit("new_coli_bit: 1", &mut [], 1, 1),
        Task::new_coli_bit("new_coli_bit: 3", &mut [], 3, 1),
        Task::new_cycle("new_cycle: 4", &mut [], 4, Duration::ZERO),
        Task::new_input_bit("new_input_bit: 4", &mut [], 4, 1),
        Task::new_coli_bit("new_coli_bit: 5", &mut [], 5, 1),
        Task::new_cycle("new_cycle: 7", &mut [], 7, Duration::ZERO),
        Task::new_input_bit("new_input_bit: 8", &mut [], 8, 1),
        Task::new_cycle("new_cycle: 9", &mut [], 9, Duration::ZERO),
        Task::new_background("new_background: 2", &mut [], 2),
        Task::new_background("new_background: 4", &mut [], 4),
        Task::new_background("new_background: 5", &mut [], 5),
        Task::new_background("new_background: 9", &mut [], 9),
    ];

    result.sort();

    assert_eq!(
        result.iter().map(|i| i.get_name()).collect::<Vec<_>>(),
        expect.iter().map(|i| i.get_name()).collect::<Vec<_>>(),
    );
    
}