use super::modbus_master::ModbusMaster;
use super::modbus_master_actions::Acton;
use rmodbus::ModbusProto;
use super::super::task::MutProgram;
use std::net::{self, TcpStream};
use std::io::{Read, Write};


pub struct ModbusTcpMaster<const N: usize> {
    socket: &'static str,
    modbus_master: ModbusMaster,
    actions: [Acton; N],
}

impl<const N: usize> ModbusTcpMaster<N> {
    pub fn new(id: u8, socket: &'static str, actions: [Acton; N]) -> Self {
        
        let modbus_master = ModbusMaster::new(id, ModbusProto::TcpUdp);

        Self { socket, modbus_master, actions }
    }
}

impl<const N: usize> MutProgram for ModbusTcpMaster<N> {
    fn run(&mut self, context: &mut rmodbus::server::context::ModbusContext) -> std::result::Result<(), Box<dyn std::error::Error>> {

        let mut stream = net::TcpStream::connect(self.socket)?;
        stream.set_nonblocking(true)?;

        for action in self.actions.iter_mut() {

            if !action.need_run(context)? {
                continue;
            }

            match action {
                Acton::ReadCoils(data) => {
                    self.modbus_master.read_coils(&mut stream, data.get_offset(), data.get_count())?;
                },
                Acton::ReadDiscretes(data) => {},
                Acton::ReadHoldings(data) => {},
                Acton::ReadInputs(data) => {},
                Acton::WriteCoil(data) => {},
                Acton::WriteCoils(data) => {},
                Acton::WriteHolding(data) => {},
                Acton::WriteHoldings(data) => {},
            }
        }

        Ok(())
    }
}