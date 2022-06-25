use std::{result, error, io, time::Duration};
use std::cell::RefCell;

use super::super::fail_strig;
use super::super::task::ConstProgram;
use rmodbus::server::context::ModbusContext;
use rmodbus::ModbusProto;
use serial::SerialPort;
pub use serial::PortSettings;
pub use serial::{BaudRate, Parity, CharSize, StopBits, FlowControl};
use super::modbus_slave::ModbusSlave;
use super::modbus_error::ModbusErr;

pub struct ModbusRtuSlave {
    port: RefCell<serial::SystemPort>,
    modbus_slave: ModbusSlave,
}

impl ModbusRtuSlave {

    pub fn new(id: u8, listen: &'static str, settings: serial::PortSettings) -> Self {

        let port = Self::create_prot(listen, settings);

        let modbus_slave = ModbusSlave::new(id, ModbusProto::Rtu);

        Self { port: RefCell::new(port), modbus_slave }
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
        
        match self.modbus_slave.handler(&mut *self.port.borrow_mut(), context) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                ModbusErr::Io(ref e) if e.kind() == io::ErrorKind::TimedOut => return Ok(()),
                _ => return Err(Box::new(err)),
            }
        }
    }
}