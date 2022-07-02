use super::modbus_master::ModbusMaster;
use super::modbus_master_actions::{Acton};
use super::modbus_error::ModbusErr;
use rmodbus::ModbusProto;
use rmodbus::server::context::ModbusContext;
use super::super::task::{ConstProgram};
use std::{net, time, result, io};
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

    fn execute(&self, action: &Acton, context: &mut ModbusContext, stream: &mut net::TcpStream) -> result::Result<(), ModbusErr> {
        match action {
            Acton::ReadCoils(data) => {
                let request = self.modbus_master.read_coils(stream, data.get_offset(), data.get_count())?;
                data.handler(context, request);
                Ok(())
            },
            Acton::ReadDiscretes(data) => {
                let request = self.modbus_master.read_discretes(stream, data.get_offset(), data.get_count())?;
                data.handler(context, request);
                Ok(())
            },
            Acton::ReadHoldings(data) => {
                let request = self.modbus_master.read_holdings(stream, data.get_offset(), data.get_count())?;
                data.handler(context, request);
                Ok(())
            },
            Acton::ReadInputs(data) => {
                let request = self.modbus_master.read_holdings(stream, data.get_offset(), data.get_count())?;
                data.handler(context, request);
                Ok(())
            },
            Acton::WriteCoil(data) => {
                let value = data.handler(context, ());
                self.modbus_master.write_coil(stream, data.get_offset(), value)?;
                Ok(())
            },
            Acton::WriteCoils(data) => {
                let values = data.handler(context, ());
                self.modbus_master.write_multipl_coils(stream, data.get_offset(), values)?;
                Ok(())
            },
            Acton::WriteHolding(data) => {
                let value = data.handler(context, ());
                self.modbus_master.write_holding(stream, data.get_offset(), value)?;
                Ok(())
            },
            Acton::WriteHoldings(data) => {
                let values = data.handler(context, ());
                self.modbus_master.write_multipl_holding(stream, data.get_offset(), values)?;
                Ok(())
            },
        }
    }
}

impl<const N: usize> ConstProgram for ModbusTcpMaster<N> {
    fn run(&self, context: &mut rmodbus::server::context::ModbusContext) -> std::result::Result<(), Box<dyn std::error::Error>> {

        let mut stream = net::TcpStream::connect(self.socket)?;
        stream.set_write_timeout(Some(time::Duration::from_micros(25)))?;
        stream.set_read_timeout(Some(self.timeout_heandler.get_timeout()))?;

        for action in self.actions.iter() {

            if !action.need_run(context)? {
                continue;
            }

            let result = self.execute(action, context, &mut stream);

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