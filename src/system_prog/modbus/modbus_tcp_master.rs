use super::modbus_master::ModbusMaster;
use super::modbus_master_actions::{Acton};
use super::modbus_error::ModbusErr;
use rmodbus::ModbusProto;
use rmodbus::server::context::ModbusContext;
use crate::task::ConstProgram;
use std::{net, time, io};
use super::timeaut_heandler::TimeautHeandler;

pub struct ModbusTcpMaster<const N: usize> {
    socket: &'static str,
    modbus_master: ModbusMaster,
    actions: [Acton; N],
    timeout_heandler: TimeautHeandler,
}

impl<const N: usize> ModbusTcpMaster<N> {
    pub fn new(id: u8, socket: &'static str, actions: [Acton; N], timeout_heandler: TimeautHeandler) -> Self {
        
        let modbus_master = ModbusMaster::new(id, ModbusProto::TcpUdp);

        Self { socket, modbus_master, actions, timeout_heandler }
    }
}

impl<const N: usize> ConstProgram for ModbusTcpMaster<N> {
    fn run(&self, context: &mut ModbusContext) -> std::result::Result<(), Box<dyn std::error::Error>> {

        let mut stream = net::TcpStream::connect(self.socket)?;
        stream.set_write_timeout(Some(time::Duration::from_micros(25)))?;
        stream.set_read_timeout(Some(self.timeout_heandler.get_timeout()))?;

        for action in self.actions.iter() {

            if !action.need_run(context)? {
                continue;
            }

            let result = self.modbus_master.execute_action(action, context, &mut stream);

            if let Err(err) = result {
                match err {
                    ModbusErr::Io(ref e)
                        if e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock => {
                            continue; //TODO
                        }
                    _ => return Err(Box::new(err))
                }
            }
        }

        Ok(())
    }
}