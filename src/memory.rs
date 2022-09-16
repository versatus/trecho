#![allow(unused, unused_mut, dead_code)]
use crate::exceptions::Exception;
use crate::register::RegisterValue;
use std::fmt::{Display, Formatter};
use std::error::Error;
use crate::consts::{MAX_MEM, INDICES, };

pub const BASE: u64 = 0x8000_0000;
pub const BYTE: u8 = 8;
pub const DOUBLEWORD: u8 = 64;
pub const HALFWORD: u8 = 16;
pub const WORD: u8 = 32;
pub const MEM_SIZE: u64 = 1024 * 1024 * 128;

// Trait to provide memory functionality and types. 
// b == Byte
// hw == Half Word
// w == Word
// dw == DoubleWord
pub trait Memory: Default {
    type RegValue: RegisterValue + From<u8> + From<u16> + From<u32> + From<u64>;
    type Bytes;
    type Error: Error;

    fn init(&mut self, addr: u64, size: u64, flags: u8, source: Option<Self::Bytes>, offset: u64) -> Result<(), Self::Error>;
    fn get_flag(&mut self, index: u64) -> Result<u8, Self::Error>;
    fn set_flag(&mut self, index: u64, flag: u8) -> Result<(), Self::Error>;
    fn clear_flag(&mut self, index: u64, flag: u8) -> Result<(), Self::Error>;

    fn store_byte(&mut self, addr: u64, size: u64, val: u8) -> Result<(), Self::Error>;
    fn store_byte_array(&mut self, addr: u64, val: &[u8]) -> Result<(), Self::Error>;
    fn execute_loadhw(&mut self, addr: u64) -> Result<u16, Self::Error>;
    fn execute_loadw(&mut self, addr: u64) -> Result<u32, Self::Error>;


    fn loadb(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error>;
    fn loadhw(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error>;
    fn loadw(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error>;
    fn loaddw(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error>;
    
    fn storeb(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error>;
    fn storehw(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error>;
    fn storew(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error>;
    fn storedw(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error>;
    
    fn read(&self, addr: &Self::RegValue, size: u8) -> Result<Self::RegValue, Self::Error>;
    fn readb(&self, addr: &Self::RegValue) -> Self::RegValue;
    fn readhw(&self, addr: &Self::RegValue) -> Self::RegValue;
    fn readw(&self, addr: &Self::RegValue) -> Self::RegValue;
    fn readdw(&self, addr: &Self::RegValue) -> Self::RegValue;

    fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Self::Error>;
    fn write_byte(&mut self, addr: u64, val: u64);
    fn write_halfword(&mut self, addr: u64, val: u64);
    fn write_word(&mut self, addr: u64, val: u64);
    fn write_doubleword(&mut self, addr: u64, val: u64);
    fn into_u64(&self, val: &Self::RegValue) -> u64;

 }

pub struct Dram {
    pub mem: Vec<u8>,
    flags: Vec<u8>,
    size: u64,
}

impl Dram {
    pub fn new() -> Dram {
        Dram {
            mem: vec![0; MAX_MEM],
            flags: vec![0; INDICES],
            size: 0
        }
    }

    pub fn init(&mut self, bin: Vec<u8>) {
        self.size = bin.len() as u64;
        self.mem.splice(..bin.len(), bin.iter().cloned());
    }

    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        match size {
            BYTE => {
                Ok(self.read_byte(addr))
            },
            HALFWORD => {
                Ok(self.read_halfword(addr))
            },
            WORD => {
                Ok(self.read_word(addr))
            },
            DOUBLEWORD => {
                Ok(self.read_doubleword(addr))
            },
            _ => return Err(Exception::LoadAccessFault)
        }
    }

    fn read_byte(&self, addr: u64) -> u64 {
        let idx = (addr - BASE) as usize;
        self.mem[idx] as u64
    }

    fn read_halfword(&self, addr: u64) -> u64 {
        let idx = (addr - BASE) as usize;
        return (self.mem[idx] as u64) | ((self.mem[idx + 1] as u64) << 8);
    }

    fn read_word(&self, addr: u64) -> u64 {
        let idx = (addr - BASE) as usize;
        return (self.mem[idx] as u64) | 
            ((self.mem[idx + 1] as u64) << 8) | 
            ((self.mem[idx + 1] as u64) << 16) | 
            ((self.mem[idx + 1] as u64) << 24);
    }

    fn read_doubleword(&self, addr: u64) -> u64 {
        let idx = (addr - BASE) as usize;
        return (self.mem[idx] as u64) |
            ((self.mem[idx + 1] as u64) << 8) |
            ((self.mem[idx + 2] as u64) << 16) |
            ((self.mem[idx + 3] as u64) << 24) |
            ((self.mem[idx + 4] as u64) << 32) |
            ((self.mem[idx + 5] as u64) << 40) |
            ((self.mem[idx + 6] as u64) << 48) |
            ((self.mem[idx + 7] as u64) << 56)
    }

    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match size {
            BYTE => { self.write_byte(addr, value) },
            HALFWORD => { self.write_halfword(addr, value) },
            WORD => { self.write_word(addr, value) },
            DOUBLEWORD => { self.write_doubleword(addr, value) },
            _ => return Err(Exception::StoreAMOAccessFault),
        }
        Ok(())
    }

    fn write_byte(&mut self, addr: u64, val: u64) {
        let idx = (addr - BASE) as usize;
        self.mem[idx] = val as u8;
    }

    fn write_halfword(&mut self, addr: u64, val: u64) {
        let idx = (addr - BASE) as usize;
        self.mem[idx] = (val & 0xff) as u8;
        self.mem[idx + 1] = ((val >> 8) & 0xff) as u8;
    }

    fn write_word(&mut self, addr: u64, val: u64) {
        let idx = (addr - BASE) as usize;
        self.mem[idx] = (val & 0xff) as u8;
        self.mem[idx + 1] = ((val >> 8) & 0xff) as u8;
        self.mem[idx + 2] = ((val >> 16) & 0xff) as u8;
        self.mem[idx + 3] = ((val >> 24) & 0xff) as u8;
    }

    fn write_doubleword(&mut self, addr: u64, val: u64) {
        let idx = (addr - BASE) as usize;
        self.mem[idx] = (val & 0xff) as u8;
        self.mem[idx + 1] = ((val >> 8) & 0xff) as u8;
        self.mem[idx + 2] = ((val >> 16) & 0xff) as u8;
        self.mem[idx + 3] = ((val >> 24) & 0xff) as u8;
        self.mem[idx + 4] = ((val >> 32) & 0xff) as u8;
        self.mem[idx + 5] = ((val >> 40) & 0xff) as u8;
        self.mem[idx + 6] = ((val >> 48) & 0xff) as u8;
        self.mem[idx + 7] = ((val >> 56) & 0xff) as u8;
    }
}

#[derive(Debug, Clone)]
pub enum MemError {
    OutOfBounds
}

impl Display for MemError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MemError {}

impl Memory for Dram {
    type RegValue = u64;
    type Bytes = Vec<u8>;
    type Error = MemError;

    fn init(&mut self, addr: u64, size: u64, flags: u8, source: Option<Self::Bytes>, offset: u64) -> Result<(), Self::Error> {
        let mut written = 0;
        if offset > 0 {
            let actual = std::cmp::min(size, offset);
            if let Err(e) = self.store_byte(addr, actual, 0) {
                return Err(e)
            }
            written += actual;
        }
        if let Some(src) = source {
            let actual = std::cmp::min(size - written, src.len() as u64);
            if actual > 0 {
                if let Err(e) = self.store_byte_array(addr + written, &src[0..actual as usize]) {
                    return Err(e)
                }
                written += actual;
            }
        }
        if written < size {
            if let Err(e) = self.store_byte(addr + written, size - written, 0) {
                return Err(e)
            }
        }

        Ok(())
    }

    fn get_flag(&mut self, idx: u64) -> Result<u8, Self::Error> {
        if idx < INDICES as u64 {
            Ok(self.flags[idx as usize])
        } else {
            Err(MemError::OutOfBounds)
        }
    }

    fn set_flag(&mut self, idx: u64, flag: u8) -> Result<(), Self::Error> {
        if idx < INDICES as u64 {
            self.flags[idx as usize] |= flag;
            Ok(())
        } else {
            Err(MemError::OutOfBounds)
        }
    }

    fn clear_flag(&mut self, idx: u64, flag: u8) -> Result<(), Self::Error> {
        if idx < INDICES as u64 {
            self.flags[idx as usize] &= !flag;
            Ok(())
        } else {
            Err(MemError::OutOfBounds)
        }
    }

    fn store_byte(&mut self, addr: u64, size: u64, val: u8) -> Result<(), Self::Error> { todo!() }
    fn store_byte_array(&mut self, addr: u64, val: &[u8]) -> Result<(), Self::Error> { todo!() }
    fn execute_loadhw(&mut self, addr: u64) -> Result<u16, Self::Error> { todo!() }
    fn execute_loadw(&mut self, addr: u64) -> Result<u32, Self::Error> { todo!() }

    fn loadb(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error> { todo!() }
    fn loadhw(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error> { todo!()}
    fn loadw(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error> { todo!() }
    fn loaddw(&mut self, addr: &Self::RegValue) -> Result<Self::RegValue, Self::Error> { todo!() }
    
    fn storeb(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error> { todo!() }
    fn storehw(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error> { todo!() }
    fn storew(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error> { todo!() }
    fn storedw(&mut self, addr: &Self::RegValue, value: &Self::RegValue) -> Result<(), Self::Error> { todo!() }
    
    fn read(&self, addr: &Self::RegValue, size: u8) -> Result<Self::RegValue, Self::Error> { todo!() }
    fn readb(&self, addr: &Self::RegValue) -> Self::RegValue { todo!() }
    fn readhw(&self, addr: &Self::RegValue) -> Self::RegValue { todo!() }
    fn readw(&self, addr: &Self::RegValue) -> Self::RegValue { todo!() }
    fn readdw(&self, addr: &Self::RegValue) -> Self::RegValue { todo!() }

    fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Self::Error> { todo!() }
    fn write_byte(&mut self, addr: u64, val: u64) { todo!() }
    fn write_halfword(&mut self, addr: u64, val: u64) { todo!() }
    fn write_word(&mut self, addr: u64, val: u64) { todo!() }
    fn write_doubleword(&mut self, addr: u64, val: u64) { todo!() }
    fn into_u64(&self, val: &Self::RegValue) -> u64 { todo!() }
}

impl Default for Dram {
    fn default() -> Dram {
        Dram {
            mem: vec![0; MAX_MEM],
            flags: vec![0; INDICES],
            size: 0
        }
    }
}

#[inline(always)]
pub fn memset(arr: &mut [u8], val: u8) {
    let p = arr.as_mut_ptr();
    unsafe {
        std::ptr::write_bytes(p, val, arr.len())
    }
}