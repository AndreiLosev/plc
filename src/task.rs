mod task_errors;

use std::time::{Duration, Instant};
use std::error;
use std::result;
use rmodbus::server::context::{ModbusContext};
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

pub struct TaskSettings {
    max_work_time_for_not_cycle_task: Duration,
    return_time_work: Duration,
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

    pub fn get_program_count(&self) -> usize { self.programs.len() }

    pub fn run(
        &mut self,
        context: &mut ModbusContext,
        task_num: usize,
        settings: &TaskSettings,
    ) -> result::Result<usize, Box<dyn error::Error>> {

        if task_num > (self.programs.len() - 1) {
            let e = TaskOtherError::new("Task::run(), task_num > self.programs.len()");
            return Err(Box::new(e));
        }

        if task_num == 0 {
            self.work_time = Some(Instant::now());
        }

        self.programs[task_num].run(context)?;

        if task_num == (self.programs.len() - 1) {
            self.stop_time_work(settings)?;
            return Ok(0);
        }

        if let Some(i) = self.work_time {
            if i.elapsed() > settings.return_time_work {
                let e = TaskTimeOutError::new(
                    i.elapsed(),
                    self.name,
                    settings.return_time_work,
                );

                return Err(Box::new(e));
            }
        }

        Ok(task_num + 1)
    }

    fn stop_time_work(&mut self, settings: &TaskSettings) -> result::Result<(), Box<dyn error::Error>> {
        let set_time = match self.event {
            Event::Cycle(v) => v,
            _ => settings.max_work_time_for_not_cycle_task,
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


#[test]
fn test_task_is_work() {
    let mut context = ModbusContext::new();
    
    struct Prog1;

    impl Program for Prog1 {
        fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
            context.set_holdings_bulk(0, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])?;
            Ok(())
        }
    }

    struct  Prog2;

    impl Program for Prog2 {
        fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
            for i in 0..10 {
                let new_value = context.get_holding(i)? * 3;
                context.set_holding(i, new_value)?
            }

            Ok(())
        }
    }
    
    let task_event = Event::Background;

    let programs: Vec<Box<dyn Program>> = vec![Box::new(Prog1), Box::new(Prog2), Box::new(Prog2)];

    let mut task = Task::new("test_task", programs, 2, task_event);

    let task_setting = TaskSettings {
        max_work_time_for_not_cycle_task: Duration::from_secs(100),
        return_time_work: Duration::from_secs(100),
    };

    let mut n = 0;

    loop {
        n = task.run(&mut context, n, &task_setting).unwrap();

        if n == 0 {
            break;
        }
    }    

    let mut result = Vec::new() as Vec<u16>;

    context.get_holdings_bulk(0, 10, &mut result).unwrap();

    assert_eq!(result, vec![9, 18, 27, 36, 45, 54, 63, 72, 81, 90]);

}


#[test]
fn test_task_is_error() {

    use std::any::{Any, TypeId};

    let mut context = ModbusContext::new();
    
    struct Prog1;

    impl Program for Prog1 {
        fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
            context.set_holdings_bulk(0, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])?;
            Ok(())
        }
    }

    let task_event = Event::Cycle(Duration::ZERO);

    let programs: Vec<Box<dyn Program>> = vec![Box::new(Prog1)];

    let mut task = Task::new("test_task", programs, 2, task_event);

    let task_setting = TaskSettings {
        max_work_time_for_not_cycle_task: Duration::from_secs(100),
        return_time_work: Duration::from_secs(100),
    };

    let result = task.run(&mut context, 0, &task_setting);

    // let execp = Box::new(TaskTimeOutError::new(Duration::ZERO, task.get_name(), Duration::ZERO));

    let err = result.err().expect("test faled");

    assert_eq!("task: test_task, TaskTimeOutError, time set: 0,  time left: 0", format!("{}", err));


}