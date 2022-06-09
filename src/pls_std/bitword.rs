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

#[test]
fn test_bitword() {

    assert!(false);
}