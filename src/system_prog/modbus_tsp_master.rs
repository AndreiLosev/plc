use super::modbus_master::ModbusMaster;
use super::modbus_master_actions::Acton;
use rmodbus::ModbusProto;
// use rmodbus::server::context::ModbusContext;


pub struct ModbusTcpMaster<const N: usize> {
    listen: &'static str,
    modbus_master: ModbusMaster,
    actions: [Acton; N],
}

impl<const N: usize> ModbusTcpMaster<N> {
    pub fn new(id: u8, listen: &'static str, actions: [Acton; N]) -> Self {
        
        let modbus_master = ModbusMaster::new(id, ModbusProto::TcpUdp);

        Self { listen, modbus_master, actions }
    }
}