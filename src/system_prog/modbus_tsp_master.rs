use std::io::{Read, Write};
use std::net::{TcpStream};
use super::super::task::ConstProgram;
use rmodbus::server::context::ModbusContext;
use rmodbus::ModbusProto;
use rmodbus::client::ModbusRequest;
use rmodbus::{guess_response_frame_len};
use std::{result, error, io, time};
use super::super::fail_strig;
use std::collections::HashMap;

pub enum ModbusFn<const N: usize> {
    ReadCoilds(u16),
    ReadDiscreteInputs(u16),
    ReadHoldingRegisters(u16),
    ReadInputRegisters(u16),
    WriteSingleCoil(bool),
    WriteSingleHoldingRegister(u16),
    WriteMultipleCoils([bool; N]),
    WriteMultipleHoldingRegisters([u16; N]),
}

pub enum TypeFn {
    Cycle,
    FrontCoil(u16),
    FrontDiscret(u16),
}

pub struct ModbusAction<const N: usize> {
    func: ModbusFn<N>,
    offset: u16,
    type_fn: TypeFn,
}

pub struct ModbusTspMaster<const N: usize, const M: usize> {
    id: u8,
    ip_addr: &'static str,
    actions: [ModbusAction<N>; M],
}

impl<const N: usize, const M: usize> ModbusTspMaster<N, M> {
    fn new(
        id: u8,
        ip_addr: &'static str,
        actions: [ModbusAction<N>; M],
    ) -> Self {
        Self { id, ip_addr, actions }
    }

    // fn read_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, action: &ModbusAction<N>) -> result::Result<Vec<bool>, Box<dyn error::Error>> {
    //     let mut request = Vec::new();
    //     let result = if let ModbusFn::ReadCoilds(len) = action.func {
    //         mreq.generate_get_coils(action.offset, len, &mut request)?;

    //         stream.write(&request)?;

    //         let mut buf = [0u8; 6];

    //         stream.read_exact(&mut buf)?;
    //         let mut response = Vec::new();
    //         response.extend_from_slice(&buf);

    //         let len = guess_response_frame_len(&buf, ModbusProto::TcpUdp)?;

    //         if len > 6 {
    //             let mut rest = vec![0u8; (len - 6) as usize];
    //             stream.read_exact(&mut rest).unwrap();
    //             response.extend(rest);
    //         }

    //         let mut data = Vec::new();
    //         // check if frame has no Modbus error inside and parse response bools into data vec
    //         mreq.parse_bool(&response, &mut data).unwrap();

    //         Ok(data)
    //     } else {
    //         Err(Box::)
    //     }

    //     result
    // }
}