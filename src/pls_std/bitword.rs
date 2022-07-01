use std::{fmt, error, any, mem};
use std::ops::{BitAnd, Shr, Shl, BitOr, BitXor};
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Overflow {
    type_name: String,
    pos: u8,
}

impl Overflow {
    pub fn new<T>(_: T, pos: u8) -> Self {
        let type_name = any::type_name::<T>().to_string(); 
        Self {type_name, pos }
    }
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bitword::Overflow  attempting to access bit {} for {}", self.pos, self.type_name)
    }
}

impl error::Error for Overflow {}

pub trait BitWord:
    From<u8> +
    Shr<Output = Self> +
    Shl<Output = Self> +
    BitAnd<Output = Self> +
    PartialEq<Self> +
    BitOr<Output = Self> +
    BitXor<Output = Self> +
    Copy
{
    fn get_bit(self, pos: u8) -> Result<bool, Overflow> {
        if pos > (mem::size_of::<Self>() * 8) as u8 {
            return Err(Overflow::new(self, pos));
        }
    
        let x = self >> Self::from(pos);
    
        Ok((x & Self::from(1_u8)) == Self::from(1_u8))
    }
    fn set_bit(&mut self, pos: u8, bit: bool) -> Result<(), Overflow> {
        if pos > (mem::size_of::<Self>() * 8) as u8 {
            return Err(Overflow::new(self, pos));
        }

        let x = Self::from(1_u8) << Self::from(pos);

        match bit {
            true => *self = *self | x,
            false => {
                *self = match self.get_bit(pos).unwrap() {
                    true => *self ^ x,
                    false => *self,
                }
            }
        }

        Ok(())
    }
}

impl BitWord for u8 {}
impl BitWord for u16 {}
impl BitWord for u32 {}
impl BitWord for u64 {}
impl BitWord for u128 {}
impl BitWord for i128 {}
impl BitWord for i64 {}
impl BitWord for i32 {}
impl BitWord for i16 {}
impl BitWord for usize {}
impl BitWord for isize {}

trait ToReg {
    type ReturnType;
    fn to_reg(&self) -> Self::ReturnType;
}

impl ToReg for i32 {
    type ReturnType = [u16; 2];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        [first, second]
    }
}

impl ToReg for i64 {
    type ReturnType = [u16; 4];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        let third = u16::from_be_bytes([bytes[4], bytes[5]]);
        let fourth = u16::from_be_bytes([bytes[6], bytes[7]]);
        [first, second, third, fourth]
    }
}

impl ToReg for i128 {
    type ReturnType = [u16; 8];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        let third = u16::from_be_bytes([bytes[4], bytes[5]]);
        let fourth = u16::from_be_bytes([bytes[6], bytes[7]]);
        let fifth = u16::from_be_bytes([bytes[8], bytes[9]]);
        let sixth = u16::from_be_bytes([bytes[10], bytes[11]]);
        let seventh = u16::from_be_bytes([bytes[12], bytes[13]]);
        let eighth = u16::from_be_bytes([bytes[14], bytes[15]]);
        [first, second, third, fourth, fifth, sixth, seventh, eighth]
    }
}

impl ToReg for u32 {
    type ReturnType = [u16; 2];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        [first, second]
    }
}

impl ToReg for u64 {
    type ReturnType = [u16; 4];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        let third = u16::from_be_bytes([bytes[4], bytes[5]]);
        let fourth = u16::from_be_bytes([bytes[6], bytes[7]]);
        [first, second, third, fourth]
    }
}

impl ToReg for u128 {
    type ReturnType = [u16; 8];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        let third = u16::from_be_bytes([bytes[4], bytes[5]]);
        let fourth = u16::from_be_bytes([bytes[6], bytes[7]]);
        let fifth = u16::from_be_bytes([bytes[8], bytes[9]]);
        let sixth = u16::from_be_bytes([bytes[10], bytes[11]]);
        let seventh = u16::from_be_bytes([bytes[12], bytes[13]]);
        let eighth = u16::from_be_bytes([bytes[14], bytes[15]]);
        [first, second, third, fourth, fifth, sixth, seventh, eighth]
    }
}

impl  ToReg for f32 {
    type ReturnType = [u16; 2];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        [first, second]
    }
}

impl  ToReg for f64 {
    type ReturnType = [u16; 4];

    fn to_reg(&self) -> Self::ReturnType {
        let bytes = self.to_be_bytes();
        let first = u16::from_be_bytes([bytes[0], bytes[1]]);
        let second = u16::from_be_bytes([bytes[2], bytes[3]]);
        let third = u16::from_be_bytes([bytes[4], bytes[5]]);
        let fourth = u16::from_be_bytes([bytes[6], bytes[7]]);
        [first, second, third, fourth]
    }
}

#[test]
fn test_get_bit() {

    let mut result: Vec<bool> = Vec::new();

    for i in 0..16 {
        result.push(34567.get_bit(i).unwrap())
    }

    let expect = vec![
        true, true, true, false,
        false, false, false, false,
        true, true, true, false,
        false, false, false, true,
    ];

    assert_eq!(result, expect);
}

#[test]
fn test_set_bit() {

    let mut result = 76543;

    for (i, &bit) in (vec![
        true, true, true, false,
        false, false, false, false,
        true, true, true, false,
        false, false, false, true,
    ]).iter().enumerate() {
        result.set_bit(i as u8, bit).unwrap();
    }

    let expect = 100103;

    assert_eq!(result, expect);
}

#[test]
fn test_to_reg() {

    use rmodbus::server::context::ModbusContext;

    let mut context = ModbusContext::new();
    context.set_holdings_from_u32(33, 603159).unwrap();
    let value = context.get_holdings_as_u32(33).unwrap();
    
    let mut u32ex = vec![];
    context.get_holdings_bulk(33, 2, &mut u32ex).unwrap();

    assert_eq!(u32ex, value.to_reg());
    assert_eq!(u32ex, (value as i32).to_reg());

    context.set_holdings_from_u64(66, 9875603159).unwrap();
    let value = context.get_holdings_as_u64(66).unwrap();
    
    let mut u64ex = vec![];
    context.get_holdings_bulk(66, 4, &mut u64ex).unwrap();

    assert_eq!(u64ex, value.to_reg());
    assert_eq!(u64ex, (value as i64).to_reg());

    context.set_holdings_from_u64(99, 9544894561525603159).unwrap();
    context.set_holdings_from_u64(103, 5544894561525603159).unwrap();
    let value1 = context.get_holdings_as_u64(99).unwrap().to_be_bytes();
    let value2 = context.get_holdings_as_u64(103).unwrap().to_be_bytes();
    let value = i128::from_be_bytes([
        value1[0], value1[1], value1[2], value1[3],
        value1[4], value1[5], value1[6], value1[7],
        value2[0], value2[1], value2[2], value2[3],
        value2[4], value2[5], value2[6], value2[7],
    ]);
    
    let mut u128ex = vec![];
    context.get_holdings_bulk(99, 8, &mut u128ex).unwrap();

    assert_eq!(u128ex, value.to_reg());
    assert_eq!(u128ex, (value as i128).to_reg());

    let value = context.get_holdings_as_f32(99).unwrap();
    let mut f32ex = vec![];
    context.get_holdings_bulk(99, 2, &mut f32ex).unwrap();
    assert_eq!(f32ex, value.to_reg());

}