use std::io::Read;
use std::{result, error, io, time::Duration};
use std::cell::RefCell;

use super::super::fail_strig;
use super::super::task::ConstProgram;
use rmodbus::server::context::ModbusContext;
use rmodbus::ModbusProto;
use rmodbus::ModbusFrameBuf;
use serial::SerialPort;
pub use serial::PortSettings;
pub use serial::{BaudRate, Parity, CharSize, StopBits, FlowControl};
use super::modbus_slave::{modbus_slave, ModbusErr};

pub struct ModbusRtuSlave {
    id: u8,
    port: RefCell<serial::SystemPort>,
}

impl ModbusRtuSlave {

    pub fn new(id: u8, listen: &'static str, settings: serial::PortSettings) -> Self {

        let port = Self::create_prot(listen, settings);

        Self { id, port: RefCell::new(port) }
    }

    fn create_prot(listen: &'static str, settings: serial::PortSettings) -> serial::SystemPort {
        let mut port = serial::open(listen)
            .unwrap_or_else(|e| panic!("{}", fail_strig(&e))); 

        port.configure(&settings)
            .unwrap_or_else(|e| panic!("{}", fail_strig(&e))); 

        port.set_timeout(Duration::ZERO)
            .unwrap_or_else(|e| panic!("{}", fail_strig(&e))); 

        port
    }
}


impl<'a> ConstProgram for ModbusRtuSlave {
    fn run(&self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
        
        loop {
         
            let end = match modbus_slave(&mut *self.port.borrow_mut(), context, ModbusProto::Rtu, self.id) {
                Ok(end) => end,
                Err(err) => match err {
                    ModbusErr::Io(ref e) if e.kind() == io::ErrorKind::TimedOut => return Ok(()),
                    _ => return Err(Box::new(err)),
                }
            };

            match end {
                true => break Ok(()),
                false => (),
            }    
        }
    }
}