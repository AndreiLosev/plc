use std::{time};
use super::modbus_error::ModbusErr;
use rmodbus::server::context::ModbusContext;
use std::cell::RefCell;

pub struct ActonData<U, K> {
    offset: u16,
    count: u16,
    type_action: TypeAction,
    handler: fn(&mut ModbusContext, U) -> K,
}

impl<U, K> ActonData<U, K> {
    pub fn get_offset(&self) -> u16 { self.offset }
    pub fn get_count(&self) -> u16 { self.count }
    
    pub fn need_run(&self, context: &mut ModbusContext) -> Result<bool, ModbusErr> {
        self.type_action.need_run(context)
    }
    
    pub fn handler(&self, context: &mut ModbusContext, data: U) -> K {
        let exec = self.handler;
        exec(context, data)
    }
}

pub enum TypeAction {
    Cycle(time::Duration, RefCell<time::Instant>),
    FrontColi(u16, RefCell<bool>),
    FrontDiscrete(u16, RefCell<bool>),
}

impl TypeAction {
    fn need_run(&self, context: &mut ModbusContext) -> Result<bool, ModbusErr> {
        match &self {
            Self::Cycle(t, i) => {
                let time_left = i.borrow().elapsed();
                if t <= &time_left {
                    // dbg!(time_left, i.borrow().elapsed());
                    i.borrow_mut().clone_from(&time::Instant::now());
                    return Ok(true);
                }
                Ok(false)
            },
            &Self::FrontColi(adr, b) => {
                let bit = context.get_coil(*adr)?;
                let need_run = bit && !*b.borrow();
                b.borrow_mut().clone_from(&bit);
                Ok(need_run)
            },
            &Self::FrontDiscrete(adr, b) => {
                let bit = context.get_discrete(*adr)?;
                let need_run = bit && !*b.borrow();
                b.borrow_mut().clone_from(&bit);
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
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::ReadCoils(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_colis(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::ReadCoils(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_colis(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::ReadCoils(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_read_discretes(
        offset: u16,
        count: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::ReadDiscretes(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_discretes(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::ReadDiscretes(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_discretes(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<bool>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::ReadDiscretes(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_read_inputs(
        offset: u16,
        count: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::ReadInputs(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_inputs(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::ReadInputs(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_inputs(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::ReadInputs(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_read_holdings(
        offset: u16,
        count: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::ReadHoldings(ActonData { offset, count, type_action, handler })
    }

    pub fn front_coil_read_holdings(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::ReadHoldings(ActonData { offset, count, type_action, handler })
    }

    pub fn front_discrete_read_holdings(
        offset: u16,
        count: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, Vec<u16>),
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::ReadHoldings(ActonData { offset, count, type_action, handler })
    }

    pub fn cycle_write_coil(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> bool,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::WriteCoil(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_write_coil(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> bool,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::WriteCoil(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_write_coil(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> bool,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::WriteCoil(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn cycle_write_coils(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> Vec<bool>,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::WriteCoils(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_coils(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<bool>,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::WriteCoils(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_coils(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<bool>,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::WriteCoils(ActonData { offset, count: 0, type_action, handler })
    }
    
    pub fn cycle_write_holding(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> u16,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::WriteHolding(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_write_holding(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> u16,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::WriteHolding(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_write_holding(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> u16,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::WriteHolding(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn cycle_write_holdings(
        offset: u16,
        time: time::Duration,
        handler: fn(&mut ModbusContext, ()) -> Vec<u16>,
    ) -> Self {
        
        let type_action = TypeAction::Cycle(time, RefCell::new(time::Instant::now())); 
        
        Self::WriteHoldings(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_coil_holdings(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<u16>,
    ) -> Self {
        
        let type_action = TypeAction::FrontColi(coil, RefCell::new(false)); 
        
        Self::WriteHoldings(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn front_discrete_holdings(
        offset: u16,
        coil: u16,
        handler: fn(&mut ModbusContext, ()) -> Vec<u16>,
    ) -> Self {
        
        let type_action = TypeAction::FrontDiscrete(coil, RefCell::new(false)); 
        
        Self::WriteHoldings(ActonData { offset, count: 0, type_action, handler })
    }

    pub fn need_run(&self, context: &mut ModbusContext) -> Result<bool, ModbusErr> {
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
