use std::{io, result};
use rmodbus::client::ModbusRequest;
use rmodbus::guess_response_frame_len;
use rmodbus::ModbusProto;
use super::modbus_error::ModbusErr;


pub struct ModbusMaster {
    id: u8,
    proto: ModbusProto,
}

impl ModbusMaster {

    pub fn new(id: u8, proto: ModbusProto) -> Self {
        Self { id, proto }
    }

    pub fn read_coils<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        count: u16,
    ) -> result::Result<Vec<bool>, ModbusErr> {
        
        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::new();
        mreq.generate_get_coils(offset, count, &mut request)?;
        transport.write(&request)?;
    
        let response = self.read_request(transport)?;
    
        let mut data = Vec::new();
        mreq.parse_bool(&response, &mut data).unwrap();

        Ok(data)
    }
    
    pub fn read_discretes<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        count: u16,
    ) -> result::Result<Vec<bool>, ModbusErr> {
        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::new();
        mreq.generate_get_discretes(offset, count, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        let mut data = Vec::new();
        mreq.parse_bool(&response, &mut data).unwrap();

        Ok(data)
    }

    fn read_request<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
    ) -> result::Result<Vec<u8>, ModbusErr> {
    
        let mut buf = [0u8; 6];
        transport.read_exact(&mut buf)?;
        let mut response = Vec::with_capacity(8);
        response.extend_from_slice(&buf);
        let len = guess_response_frame_len(&buf, self.proto)?;
    
        if len > 6 {
            let mut rest = vec![0u8; (len - 6) as usize];
            transport.read_exact(&mut rest).unwrap();
            response.extend(rest);
        }
    
        Ok(response)
    }

}
