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
use std::fs::{File, metadata};
use std::io::Read;


pub const STACKSIZE: u64 = 4096u64;
pub const INST_LEN: u64 = 4u64;
pub type CpuResult = Result<(), Exception>;

#[derive(Debug)]
pub struct ProgramBuffer {
    pub cursor: usize,
    pub buf: Vec<u8>
}

#[derive(Debug)]
pub struct Cpu {
    pub core: SoftThread<u64, f64, Dram>,
    ext: Extension,
    pb: ProgramBuffer,
    //TODO: Add queue so that the VM can run programs sequentially.
    //TODO: Replace core with multi core structure
    //TODO: Add task scheduler to communicate tasks to multiple cores from queue.
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
        let mut f = File::open(&path).expect("file not found");
        let meta = metadata(&path).expect("unable to read metadata");
        if meta.len() > (STACKSIZE * INST_LEN) {
            let mut buffer = vec![];
            f.read_to_end(&mut buffer);
            self.pb.buf = buffer;
            self.pb.cursor = 0;
            return Err(Exception::LoadFromBuffer);
        } else {
            let mut buffer = vec![0; meta.len() as usize];
            f.read(&mut buffer).expect("buffer overflow");
            self.core.load_program(buffer)?;
        }
        Ok(())
    }

    pub fn load_from_state<S: StateObject>(&mut self, state: S, addr: S::Address) -> CpuResult {
        if let Ok(program) = state.get_code(&addr) {
            let program: Vec<u8> = program.into();
            if program.len() > ((STACKSIZE * INST_LEN) as usize) {
                self.core.load_program(program[..(STACKSIZE * INST_LEN) as usize].into());
                self.pb.buf = program[((STACKSIZE * INST_LEN) as usize)..].to_vec();
                self.pb.cursor = (STACKSIZE * INST_LEN) as usize;
                return Err(Exception::LoadFromBuffer);
            }

            self.core.load_program(program.into())?;
        }

        return Err(Exception::InvalidAddr)
    }
}

impl Default for ProgramBuffer {
    fn default() -> ProgramBuffer {
        ProgramBuffer {
            cursor: 0,
            buf: vec![],
        } 
    }
}


impl Default for Cpu {
    fn default() -> Cpu {
        let mut softs = SoftThread::<u64, f64, Dram>::default(); 
        Cpu {
            core: softs,
            ext: Extension::G,
            pb: ProgramBuffer::default()
        }
    }
}


