#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;
use crate::extensions::{Extension};
use crate::exceptions::Exception;
use crate::memory::{Memory, Dram};
use crate::register::RegisterValue;
use crate::state::StateObject;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::hash::Hash;

pub const STACKSIZE: u64 = 4096u64;
pub type CpuResult = Result<(), Exception>;


#[derive(Debug)]
pub struct Cpu {
    pub core: SoftThread<u64, f64, Dram>,
    ext: Extension,
    //TODO: Add buffer so that programs that are to long can be loaded via buffer.
    //TODO: Replace core with multi core structure
    //TODO: Add task scheduler to communicate tasks to multiple cores.
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()    
    }

    pub fn run(&mut self) -> CpuResult {
        while self.core.pc < (self.core.program.len() as u64) {
            self.core.execute();
        }
        Ok(())
    }
    
    pub fn load_from_file(&mut self, path: String) -> CpuResult {
        //TODO: Read the binary file in from the path, parse and convert into Vec<u8>
        // store in self.core.program if the length of the program in u8 bytes is < `n`, else read into a
        // buffer, and maintain a cursor position for the file, so that we can go back to this
        // position and read the remaining data. `n` should be no greater than 4096;
        unimplemented!();
    }

    pub fn load_from_state<S: StateObject>(&mut self, state: S, addr: S::Address) -> CpuResult {
        // TODO: Get the code (if any) from the state address requested and load it
        // into the self.core.program field for execution, otherwise return an error;
        if let Ok(program) = state.get_code(&addr) {
    
            let res = self.core.load_program(program.into());
            match res {
                Ok(()) => {
                    return Ok(());
                }
                Err(e) => {
                    //TODO: Load into buffer
                    unimplemented!();
                }
            }
        }

        return Err(Exception::InvalidAddr)
    }
}


impl Default for Cpu {
    fn default() -> Cpu {

        let mut softs = SoftThread::<u64, f64, Dram>::default();
        
        Cpu {
            core: softs,
            ext: Extension::G
        }
    }
}

