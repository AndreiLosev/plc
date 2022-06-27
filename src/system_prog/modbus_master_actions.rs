use std::time;
use rmodbus::server::context::ModbusContext;

pub struct ActonData<U, K> {
    offset: u16,
    count: u16,
    type_action: TypeAction,
    handler: fn(&mut ModbusContext, U) -> K,
}

pub enum TypeAction {
    Cycle(time::Duration, time::Instant),
    FrontColi(u16, bool),
    FrontDiscrete(u16, bool),
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
}
