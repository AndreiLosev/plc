use std::{result,io};
use rmodbus::server::context::ModbusContext;
use rmodbus::server::ModbusFrame;
use rmodbus::ModbusProto;
use rmodbus::ModbusFrameBuf;
use super::modbus_error::ModbusErr;

pub fn modbus_slave<T: io::Read + io::Write>(
    transport: &mut T,
    context: &mut ModbusContext,
    proto: ModbusProto,
    id: u8,
) -> result::Result<bool, ModbusErr> {

    let mut response = Vec::with_capacity(8);
    let mut buf: ModbusFrameBuf = [0; 256];

    transport.read(&mut buf)?;

    let mut frame = ModbusFrame::new(id, &buf, proto, &mut response);

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
        return Ok(true);
    }

    Ok(false)
}
