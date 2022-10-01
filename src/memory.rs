#![allow(unused, unused_mut, dead_code)]
use crate::exceptions::Exception;
use crate::register::RegisterValue;
use std::fmt::{Display, Formatter};
use std::error::Error;
use crate::consts::{MAX_MEM, INDICES, INDEX_SHIFTS, DIRTY};

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
    fn get_indices(addr: u64, size: u64) -> Result<(u64, u64), Self::Error>;

    fn execute_readhw(&mut self, addr: u64) -> Self::RegValue;
    fn execute_readw(&mut self, addr: u64) -> Self::RegValue;

    fn read(&self, addr: &Self::RegValue, size: u8) -> Result<Self::RegValue, Self::Error>;
    fn readb(&self, addr: &Self::RegValue) -> Self::RegValue;
    fn readhw(&self, addr: &Self::RegValue) -> Self::RegValue;
    fn readw(&self, addr: &Self::RegValue) -> Self::RegValue;
    fn readdw(&self, addr: &Self::RegValue) -> Self::RegValue;

    fn write_array(&mut self, addr: Self::RegValue, val: Self::Bytes) -> Result<(), Self::Error>;
    fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Self::Error>;
    fn writeb(&mut self, addr: Self::RegValue, val: Self::RegValue);
    fn writehw(&mut self, addr: Self::RegValue, val: Self::RegValue);
    fn writew(&mut self, addr: Self::RegValue, val: Self::RegValue);
    fn writedw(&mut self, addr: u64, val: u64);
    fn into_u64(&self, val: &Self::RegValue) -> u64;
    fn into_i64(&self, val: &Self::RegValue) -> i64;
    fn into_u32(&self, val: &Self::RegValue) -> u32;
    fn into_i32(&self, val: &Self::RegValue) -> i32;


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
}

#[derive(Debug, Clone)]
pub enum MemError {
    OutOfBounds,
    LoadAccessFault,
    StoreAMOAccessFault
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
            self.writeb(addr, actual);
            written += actual;
        }
        if let Some(src) = source {
            let actual = std::cmp::min(size - written, src.len() as u64);
            if actual > 0 {
                self.write_array((addr + written), src[0..actual as usize].to_vec());
                written += actual;
            }
        }
        if written < size {
            self.writeb(addr + written, size - written);
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

    fn get_indices(addr: u64, size: u64) -> Result<(u64, u64), Self::Error> {
        let (end, overflow) = addr.overflowing_add(size);
        if overflow {
            return Err(MemError::OutOfBounds);
        }

        if end > MAX_MEM as u64 {
            return Err(MemError::OutOfBounds);
        }

        let idx = addr >> INDEX_SHIFTS;
        let idx_end = (end - 1) >> INDEX_SHIFTS;

        Ok((idx, idx_end))
    }

    fn execute_readhw(&mut self, addr: u64) -> Self::RegValue { 
        self.readhw(&addr)
    }

    fn execute_readw(&mut self, addr: u64) -> Self::RegValue {
        self.readw(&addr)
    }
    
    fn read(&self, addr: &Self::RegValue, size: u8) -> Result<Self::RegValue, Self::Error> {
        match size {
            BYTE => {
                Ok(self.readb(addr))
            },
            HALFWORD => {
                Ok(self.readhw(addr))
            },
            WORD => {
                Ok(self.readw(addr))
            },
            DOUBLEWORD => {
                Ok(self.readdw(addr))
            },
            _ => return Err(MemError::LoadAccessFault)
        }
    }

    fn readb(&self, addr: &Self::RegValue) -> Self::RegValue {
        let idx: usize = *addr as usize;
        self.mem[idx] as u64
    }
    fn readhw(&self, addr: &Self::RegValue) -> Self::RegValue {
        let idx: usize = *addr as usize;
        return (self.mem[idx] as u64) | ((self.mem[idx + 1] as u64) << 8);
    }
    fn readw(&self, addr: &Self::RegValue) -> Self::RegValue {
        let idx: usize = *addr as usize;
        return (self.mem[idx] as u64) | 
            ((self.mem[idx + 1] as u64) << 8) | 
            ((self.mem[idx + 2] as u64) << 16) | 
            ((self.mem[idx + 3] as u64) << 24);
    }

    fn readdw(&self, addr: &Self::RegValue) -> Self::RegValue {
        let idx: usize = *addr as usize;
        return (self.mem[idx] as u64) |
            ((self.mem[idx + 1] as u64) << 8) |
            ((self.mem[idx + 2] as u64) << 16) |
            ((self.mem[idx + 3] as u64) << 24) |
            ((self.mem[idx + 4] as u64) << 32) |
            ((self.mem[idx + 5] as u64) << 40) |
            ((self.mem[idx + 6] as u64) << 48) |
            ((self.mem[idx + 7] as u64) << 56)        
    }

    fn write_array(&mut self, addr: Self::RegValue, value: Self::Bytes) -> Result<(), Self::Error> {
        let size = value.len() as u64;
        if size == 0 {
            return Ok(());
        }
        let indices = Self::get_indices(addr, size)?;
        self.set_flag(addr, DIRTY);
        let arr = &mut self.mem[addr as usize..(addr + size) as usize];
        arr.copy_from_slice(&value);
        Ok(())
    }

    fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Self::Error> {
        match size {
            BYTE => { self.writeb(addr, value) },
            HALFWORD => { self.writehw(addr, value) },
            WORD => { self.writew(addr, value) },
            DOUBLEWORD => { self.writedw(addr, value) },
            _ => return Err(MemError::StoreAMOAccessFault),
        }
        Ok(())    
    }
    fn writeb(&mut self, addr: u64, val: u64) {
        let idx: usize = addr as usize;
        self.mem[idx] = val as u8;
    }

    fn writehw(&mut self, addr: u64, val: u64) {
        let idx: usize = addr as usize;
        self.mem[idx] = (val & 0xff) as u8;
        self.mem[idx + 1] = ((val >> 8) & 0xff) as u8;
    }
    
    fn writew(&mut self, addr: u64, val: u64) {
        let idx: usize = addr as usize;
        self.mem[idx] = (val & 0xff) as u8;
        self.mem[idx + 1] = ((val >> 8) & 0xff) as u8;
        self.mem[idx + 2] = ((val >> 16) & 0xff) as u8;
        self.mem[idx + 3] = ((val >> 24) & 0xff) as u8;
    }
    
    fn writedw(&mut self, addr: u64, val: u64) {
        let idx: usize = addr as usize;
        self.mem[idx] = (val & 0xff) as u8;
        self.mem[idx + 1] = ((val >> 8) & 0xff) as u8;
        self.mem[idx + 2] = ((val >> 16) & 0xff) as u8;
        self.mem[idx + 3] = ((val >> 24) & 0xff) as u8;
        self.mem[idx + 4] = ((val >> 32) & 0xff) as u8;
        self.mem[idx + 5] = ((val >> 40) & 0xff) as u8;
        self.mem[idx + 6] = ((val >> 48) & 0xff) as u8;
        self.mem[idx + 7] = ((val >> 56) & 0xff) as u8;

    }
    
    fn into_u64(&self, val: &Self::RegValue) -> u64 {
        *val as u64
    }

    fn into_i64(&self, val: &Self::RegValue) -> i64 {
        *val as i64
    }

    fn into_u32(&self, val: &Self::RegValue) -> u32 {
        *val as u32
    }

    fn into_i32(&self, val: &Self::RegValue) -> i32 {
        *val as i32
    }
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