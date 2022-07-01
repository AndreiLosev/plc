use std::net::TcpListener;
use super::super::task::ConstProgram;
use rmodbus::server::context::ModbusContext;
use rmodbus::ModbusProto;
use std::{result, error, io};
use super::super::fail_strig;
use super::modbus_slave::{ModbusSlave};
pub struct ModbusTcpSlave {
    listener: TcpListener,
    modbus_slave: ModbusSlave,
}

impl ModbusTcpSlave {

    pub fn new(id: u8, socket: &'static str) -> Self {

        let listener = Self::create_listener(socket);
        let modbus_slave = ModbusSlave::new(id, ModbusProto::TcpUdp);

        Self { listener, modbus_slave }
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

        self.modbus_slave.handler(&mut stream, context)?;
        
        Ok(())
    }
}