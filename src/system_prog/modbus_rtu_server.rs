// use std::io::{Read, Write};
// use super::super::task::Program;
// use rmodbus::server::context::ModbusContext;
// use rmodbus::server::ModbusFrame;
// use rmodbus::ModbusProto;
// use rmodbus::ModbusFrameBuf;
// use std::{result, error, io};
// use ansi_term::Color::Red;

// pub struct ModbusTcpServer {
//     id: u8,
//     listener: serial::SystemPort,
// }

// impl ModbusTcpServer {

//     pub fn new(id: u8, listen: &'static str) -> Self {

//         let listener = Self::create_listener(listen);

//         Self { id, listener}
//     }

//     fn create_listener(listen: &'static str) -> TcpListener {
//         let listener = TcpListener::bind(listen)
//         .unwrap_or_else(|e| panic!(
//             "err: {}",
//             &Red.paint(format!("{}", e))
//         ));
//         listener.set_nonblocking(true)
//             .unwrap_or_else(|e| panic!(
//                 "err: {}",
//                 &Red.paint(format!("{}", e))
//             ));

//         listener
//     }
// }


// impl<'a> Program for ModbusTcpServer {
//     fn run(&mut self, context: &mut ModbusContext) -> result::Result<(), Box<dyn error::Error>> {
        
//         let mut stream = match self.listener.accept() {
//             Ok((stream, _)) => stream,
//             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(()),
//             Err(e) => return Err(Box::new(e)),
//         };

//         loop {
         
//             let mut buf: ModbusFrameBuf = [0; 256];
//             let mut response: Vec<u8> = Vec::with_capacity(8);
//             stream.read(&mut buf)?;
//             let mut frame = ModbusFrame::new(self.id, &buf, ModbusProto::TcpUdp, &mut response);
//             frame.parse()?;
    
//             if frame.processing_required {
//                 match frame.readonly {
//                     true => frame.process_read(context),
//                     false => frame.process_write(context),
//                 }?;
//             }
    
//             if frame.response_required {
//                 frame.finalize_response()?;
//                 stream.write(response.as_slice())?;
//                 break Ok(());
//             }          
//         }      
//     }
// }