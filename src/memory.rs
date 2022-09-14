use crate::exceptions::Exception;

pub const BASE: u64 = 0x8000_0000;
pub const BYTE: u8 = 8;
pub const DOUBLEWORD: u8 = 64;
pub const HALFWORD: u8 = 16;
pub const WORD: u8 = 32;
pub const MEM_SIZE: u64 = 1024 * 1024 * 128;

pub struct Memory {
    pub mem: Vec<u8>,
    size: u64,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; MEM_SIZE as usize],
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
