use std::time;
use super::modbus_error::ModbusErr;
use rmodbus::server::context::ModbusContext;

pub struct ActonData<U, K> {
    offset: u16,
    count: u16,
    type_action: TypeAction,
    handler: fn(&mut ModbusContext, U) -> K,
}

impl<U, K> ActonData<U, K> {
    pub fn get_offset(&self) -> u16 { self.offset }
    pub fn get_count(&self) -> u16 { self.count }
    
    pub fn need_run(&mut self, context: &mut ModbusContext) -> Result<bool, ModbusErr> {
        self.type_action.need_run(context)
    }
    
    pub fn handler(&self, context: &mut ModbusContext, data: U) -> K {
        let exec = self.handler;
        exec(context, data)
    }
}

pub enum TypeAction {
    Cycle(time::Duration, time::Instant),
    FrontColi(u16, bool),
    FrontDiscrete(u16, bool),
}

impl TypeAction {
    fn need_run(&mut self, context: &mut ModbusContext) -> Result<bool, ModbusErr> {
        match self {
            Self::Cycle(t, i) => {
                if *t <= i.elapsed() {
                    *i = time::Instant::now();
                    return Ok(true);
                }
                Ok(false)
            },
            Self::FrontColi(adr, b) => {
                let bit = context.get_coil(*adr)?;
                let need_run = bit && !*b;
                *b = bit;
                Ok(need_run)
            },
            Self::FrontDiscrete(adr, b) => {
                let bit = context.get_discrete(*adr)?;
                let need_run = bit && !*b;
                *b = bit;
                Ok(need_run)
            },
        }
    }    
}

pub enum Acton {
    ReadCoils(ActonData<Vec<bool>, ()>),
    ReadDiscretes(ActonData<Vec<bool>, ()>),
    ReadHoldings(ActonData<Vec<u16>, ()>),
    ReadInputs(ActonData<Vec<u16>, ()>),
    WriteCoil(ActonData<(), bool>),
    WriteHolding(ActonData<(), u16>),
    WriteCoils(ActonData<(), Vec<bool>>),
    WriteHoldings(ActonData<(), Vec<u16>>),
}

impl Acton {
    pub fn cycle_read_colis(
        offset: u16,
        count: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::ReadCoils(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_colis(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::ReadCoils(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_colis(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::ReadCoils(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_read_discretes(
        offset: u16,
        count: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::ReadDiscretes(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_discretes(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::ReadDiscretes(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_discretes(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::ReadDiscretes(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_read_inputs(
        offset: u16,
        count: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::ReadInputs(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_inputs(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::ReadInputs(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_inputs(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::ReadInputs(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_read_holdings(
        offset: u16,
        count: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::ReadHoldings(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_holdings(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::ReadHoldings(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_holdings(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::ReadHoldings(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_write_coil(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> bool,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::WriteCoil(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_write_coil(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> bool,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::WriteCoil(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_write_coil(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> bool,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::WriteCoil(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn cycle_write_coils(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> Vec<bool>,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::WriteCoils(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_coils(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<bool>,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::WriteCoils(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_coils(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<bool>,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::WriteCoils(ActonData { offset, count: 0, type_action, handler })
    }
    
    pub fn cycle_write_holding(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> u16,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::WriteHolding(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_write_holding(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> u16,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::WriteHolding(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_write_holding(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> u16,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::WriteHolding(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn cycle_write_holdings(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> Vec<u16>,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, time::Instant::now()); 
        
        Self::WriteHoldings(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_holdings(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<u16>,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, false); 
        
        Self::WriteHoldings(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_holdings(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<u16>,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, false); 
        
        Self::WriteHoldings(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn need_run(&mut self, context: &mut ModbusContext) -> Result<bool, ModbusErr> {
        match self {
            Self::ReadCoils(data) => { data.need_run(context) }
            Self::ReadDiscretes(data) => { data.need_run(context) }
            Self::ReadHoldings(data) => { data.need_run(context) }
            Self::ReadInputs(data) => { data.need_run(context) }
            Self::WriteCoil(data) => { data.need_run(context) }
            Self::WriteCoils(data) => { data.need_run(context) }
            Self::WriteHolding(data) => { data.need_run(context) }
            Self::WriteHoldings(data) => { data.need_run(context) }
        }
    }
}
