use std::io::{Read, Write};
use std::{result, error, io, time::Duration};
use std::cell::RefCell;

use super::super::fail_strig;
use super::super::task::ConstProgram;
use rmodbus::server::context::ModbusContext;
use rmodbus::server::ModbusFrame;
use rmodbus::ModbusProto;
use rmodbus::ModbusFrameBuf;
use serial::SerialPort;
pub use serial::PortSettings;
pub use serial::{BaudRate, Parity, CharSize, StopBits, FlowControl};

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
         
            let mut buf: ModbusFrameBuf = [0; 256];
            let mut response: Vec<u8> = Vec::with_capacity(8);
            match self.port.borrow_mut().read(&mut buf) {
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => return Ok(()),
                Err(e) => return Err(Box::new(e)),
                Ok(_) => (),
            };
            let mut frame = ModbusFrame::new(self.id, &buf, ModbusProto::Rtu, &mut response);
            frame.parse()?;
    
            if frame.processing_required {
                match frame.readonly {
                    true => frame.process_read(context),
                    false => frame.process_write(context),
                }?;
            }
    
            if frame.response_required {
                frame.finalize_response()?;
                self.port.borrow_mut().write(response.as_slice())?;
                break Ok(());
            }          
        }
    }
}