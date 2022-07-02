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
        let mut request = Vec::with_capacity(count as usize);
        mreq.generate_get_coils(offset, count, &mut request)?;
        transport.write(&request)?;
    
        let response = self.read_request(transport)?;
    
        let mut data = Vec::with_capacity(count as usize);
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
        let mut request = Vec::with_capacity(count as usize);
        mreq.generate_get_discretes(offset, count, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        let mut data = Vec::new();
        mreq.parse_bool(&response, &mut data).unwrap();

        Ok(data)
    }

    pub fn read_holdings<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        count: u16,
    ) -> result::Result<Vec<u16>, ModbusErr> {

        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::with_capacity((count * 2) as usize);
        mreq.generate_get_holdings(offset, count, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        let mut data = Vec::new();
        mreq.parse_u16(&response, &mut data).unwrap();

        Ok(data)
    }

    pub fn read_inputs<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        count: u16,
    ) -> result::Result<Vec<u16>, ModbusErr> {

        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::with_capacity((count * 2) as usize);
        mreq.generate_get_inputs(offset, count, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        let mut data = Vec::new();
        mreq.parse_u16(&response, &mut data).unwrap();

        Ok(data)
    }

    pub fn write_coil<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        value: bool,
    ) -> result::Result<(), ModbusErr> {

        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::with_capacity(8);
        mreq.generate_set_coil(offset, value, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        mreq.parse_ok(&response)?;

        Ok(())
    }

    pub fn write_multipl_coils<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        values: Vec<bool>,
    ) ->result::Result<(), ModbusErr> {
        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::with_capacity(8);
        mreq.generate_set_coils_bulk(offset, &values, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        mreq.parse_ok(&response)?;

        Ok(())
    }

    pub fn write_holding<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        value: u16,
    ) -> result::Result<(), ModbusErr> {

        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::with_capacity(8);
        mreq.generate_set_holding(offset, value, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        mreq.parse_ok(&response)?;

        Ok(())
    }

    pub fn write_multipl_holding<T: io::Read + io::Write>(
        &self,
        transport: &mut T,
        offset: u16,
        values: Vec<u16>,
    ) ->result::Result<(), ModbusErr> {

        let mut mreq = ModbusRequest::new(self.id, self.proto);
        let mut request = Vec::with_capacity(8);
        mreq.generate_set_holdings_bulk(offset, &values, &mut request)?;
        transport.write(&request)?;

        let response = self.read_request(transport)?;

        mreq.parse_ok(&response)?;

        Ok(())
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
