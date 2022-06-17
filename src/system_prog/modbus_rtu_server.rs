use std::io::{Read, Write};
use serial::SystemPort;
use serial::{open, core::SerialPort};
use super::super::task::Program;
use rmodbus::server::context::ModbusContext;
use rmodbus::server::ModbusFrame;
use rmodbus::ModbusProto;
use rmodbus::ModbusFrameBuf;
use std::{result, error, io};
use ansi_term::Color::Red;

pub struct ModbusRtuServer<'a> {
    id: u8,
    listener: &'a SystemPort,
}

impl<'a> ModbusRtuServer<'a> {
    pub fn create_rtu_port(listen: &'static str) -> SystemPort {
        let mut port = open(listen)
            .unwrap_or_else(|e| panic!(
                "err: {}",
                &Red.paint(format!("{}", e))
            ));
        port.set_timeout(std::time::Duration::from_millis(1))
            .unwrap_or_else(|e| panic!(
                "err: {}",
                &Red.paint(format!("{}", e))
            ));
        port
    }

    pub fn new(id: u8, listener: &'a SystemPort) -> Self {
        Self { id, listener}
    }
}


impl Program for ModbusRtuServer<'static> {
    fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
        // // let steem
        // loop {
         
        //     let mut buf: ModbusFrameBuf = [0; 256];
        //     let mut response: Vec<u8> = Vec::with_capacity(8);
        //     stream.read(&mut buf)?;
        //     let mut frame = ModbusFrame::new(self.id, &buf, ModbusProto::TcpUdp, &mut response);
        //     frame.parse()?;
    
        //     if frame.processing_required {
        //         match frame.readonly {
        //             true => frame.process_read(context),
        //             false => frame.process_write(context),
        //         }?;
        //     }
    
        //     if frame.response_required {
        //         frame.finalize_response()?;
        //         stream.write(response.as_slice())?;
        //         break Ok(());
        //     }          
        // }  
        Ok(())    
    }
}