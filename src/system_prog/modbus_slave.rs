use std::{result, error, io, fmt};
use rmodbus::server::context::ModbusContext;
use rmodbus::server::ModbusFrame;
use rmodbus::ModbusProto;
use rmodbus::ModbusFrameBuf;

pub fn modbus_slave<T: io::Read + io::Write>(
    transport: &mut T,
    context: &mut ModbusContext,
    proto: ModbusProto,
    id: u8,
) -> result::Result<bool, ModbusErr> {

    let mut response: Vec<u8> = Vec::with_capacity(8);
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

#[derive(Debug)]
pub enum ModbusErr {
    Io(io::Error),
    Rmodbus(rmodbus::ErrorKind),
}

impl From<io::Error> for ModbusErr {
    fn from(err: io::Error) -> ModbusErr {
        ModbusErr::Io(err)
    }
}

impl From<rmodbus::ErrorKind> for ModbusErr {
    fn from(err: rmodbus::ErrorKind) -> ModbusErr {
        ModbusErr::Rmodbus(err)
    }
}

impl fmt::Display for ModbusErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Rmodbus(e) => e.fmt(f),
        }
    }
}

impl error::Error for ModbusErr {}