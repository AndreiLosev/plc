use std::{fmt, error, any, mem};
use std::ops::{BitAnd, Shr};
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

trait BitWord: From<u8> + Shr<Output = Self> + BitAnd<Output = Self> + PartialEq<Self> {
    fn get_bit(self, pos: u8) -> Result<bool, Overflow> {
        if pos > (mem::size_of::<Self>() * 8) as u8 {
            return Err(Overflow::new(self, pos));
        }
    
        let x = self >> Self::from(pos);
    
        Ok((x & Self::from(1_u8)) == Self::from(1_u8))
    }
    // fn set_bit(pos: usize, value: bool) -> Result<bool, Overflow>;
}

impl BitWord for u16 {}


#[test]
fn test_bitword() {
    let res = vec![
        15_u16.get_bit(0).unwrap(),
        15_u16.get_bit(1).unwrap(),
        15_u16.get_bit(2).unwrap(),
        15_u16.get_bit(3).unwrap(),
        15_u16.get_bit(4).unwrap(),
    ];

    dbg!(res);

    assert!(false);
}