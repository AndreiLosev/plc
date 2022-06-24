use std::{io, fmt, error};

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