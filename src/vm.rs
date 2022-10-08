#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;
use crate::extensions::{Extension};
use crate::memory::{Memory, Dram};
use crate::register::RegisterValue;
use std::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug)]
pub struct Cpu {
    pub core: SoftThread<u64, f64, Dram>,
    ext: Extension,
    //TODO: Add Queue and Buffer to load programs into core(s);
    //TODO: Replace core with multi core structure
    //TODO: Add task scheduler to communicate tasks to multiple cores.
}

#[derive(Debug, Clone, PartialEq)]
pub enum CpuError {
    Core
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()    
    }

    pub fn run(&mut self) -> Result {
        while self.core.pc < (self.core.program.len() as u64) {
            self.core.execute();
        }
        Ok(())
    }
}



impl Display for CpuError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}


impl Error for CpuError {}

impl Default for Cpu {
    fn default() -> Cpu {

        let mut softs = SoftThread::<u64, f64, Dram>::default();
        
        Cpu {
            core: softs,
            ext: Extension::G
        }
    }
}

