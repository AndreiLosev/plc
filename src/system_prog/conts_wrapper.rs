use crate::task::{MutProgram, ConstProgram};
use std::cell::RefCell;
use std::{result, error};
use rmodbus::server::context::ModbusContext;

pub struct ConstWrapper<T: MutProgram> {
    prog: RefCell<T>,
}

impl<T: MutProgram> ConstWrapper<T> {
    pub fn new(prog: T) -> Self {
        Self { prog: RefCell::new(prog) }
    }
}

impl<T: MutProgram> ConstProgram for ConstWrapper<T> {
    fn run(&self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
        
        self.prog.borrow_mut().run(context)?;
        Ok(())
    }
}