use std::{result,io};
use rmodbus::server::context::ModbusContext;
use rmodbus::server::ModbusFrame;
use rmodbus::ModbusProto;
use rmodbus::ModbusFrameBuf;
use super::modbus_error::ModbusErr;

pub struct ModbusSlave {
    id: u8,
    proto: ModbusProto,
}

impl ModbusSlave {
    pub fn new (id: u8, proto: ModbusProto) -> Self {
        Self { id, proto }
    }

    pub fn handler<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        context: &mut ModbusContext,
    ) -> result::Result<(), ModbusErr> {

        loop {
            let mut response = Vec::with_capacity(8);
            let mut buf: ModbusFrameBuf = [0; 256];
        
            transport.read(&mut buf)?;
        
            let mut frame = ModbusFrame::new(self.id, &buf, self.proto, &mut response);
        
            frame.parse()?;
            
            if frame.processing_required {
                match frame.readonly {
                    true => frame.process_read(context),
                    false => frame.process_write(context),
                }?;
            }
        
            if frame.response_required {
                frame.finalize_response()?;
                transport.write(response.as_slice())?;
                break Ok(());
            }
        }
    }
}
