use std::io::Read;
use std::net::TcpListener;
use super::super::task::ConstProgram;
use rmodbus::server::context::ModbusContext;
use rmodbus::ModbusProto;
use rmodbus::ModbusFrameBuf;
use std::{result, error, io};
use super::super::fail_strig;
use super::modbus_slave::modbus_slave;
pub struct ModbusTcpSlave {
    id: u8,
    listener: TcpListener,
}

impl ModbusTcpSlave {

    pub fn new(id: u8, listen: &'static str) -> Self {

        let listener = Self::create_listener(listen);

        Self { id, listener}
    }

    fn create_listener(listen: &'static str) -> TcpListener {
        let listener = TcpListener::bind(listen)
        .unwrap_or_else(|e| panic!("{}", fail_strig(&e)));
        listener.set_nonblocking(true)
            .unwrap_or_else(|e| panic!("{}", fail_strig(&e)));

        listener
    }
}


impl<'a> ConstProgram for ModbusTcpSlave {
    fn run(&self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
        
        let mut stream = match self.listener.accept() {
            Ok((stream, _)) => stream,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        };

        loop {
         
            let mut buf: ModbusFrameBuf = [0; 256];
            stream.read(&mut buf)?;
            
            let end = modbus_slave(&mut stream, context, ModbusProto::TcpUdp, self.id)?;

            match end {
                true => break Ok(()),
                false => (),
            }        
        }      
    }
}