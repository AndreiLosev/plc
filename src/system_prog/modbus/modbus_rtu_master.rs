use super::modbus_master::ModbusMaster;
use super::modbus_master_actions::{Acton};
use super::modbus_error::ModbusErr;
use rmodbus::ModbusProto;
use rmodbus::server::context::ModbusContext;
use crate::task::ConstProgram;
use std::time::Duration;
use std::{io,  result, error};
use super::timeaut_heandler::TimeautHeandler;
use serial::SerialPort;
pub use serial::PortSettings;

pub struct ModbusRtuMaster<const N: usize> {
    port: &'static str,
    settings: serial::PortSettings,
    modbus_master: ModbusMaster,
    actions: [Acton; N],
    timeout_heandler: TimeautHeandler,
}

impl<const N: usize> ModbusRtuMaster<N> {
    pub fn new(id: u8, port: &'static str, settings: serial::PortSettings, actions: [Acton; N], timeout_heandler: TimeautHeandler) -> Self {
        
        let modbus_master = ModbusMaster::new(id, ModbusProto::Rtu);

        Self { port, modbus_master, actions, timeout_heandler, settings }
    }

    fn create_prot(&self) -> result::Result<serial::SystemPort, Box<dyn error::Error>> {
        let mut port = serial::open(self.port)?;

        port.configure(&self.settings)?; 

        port.set_timeout(self.timeout_heandler.get_timeout())?; 

        Ok(port)
    }
}

impl<const N: usize> ConstProgram for ModbusRtuMaster<N> {
    fn run(&self, context: &mut ModbusContext) -> std::result::Result<(), Box<dyn std::error::Error>> {

        let mut serial_port = self.create_prot()?;

        std::thread::sleep(Duration::from_millis(50));

        for action in self.actions.iter() {

            if !action.need_run(context)? {
                continue;
            }

            let result = self.modbus_master.execute_action(action, context, &mut serial_port);

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