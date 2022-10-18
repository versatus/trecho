#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;
use crate::extensions::{Extension};
use crate::exceptions::Exception;
use crate::memory::{Memory, Dram};
use crate::register::RegisterValue;
use crate::state::StateObject;
use crate::scheduler::Scheduler;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::hash::Hash;
use std::fs::{File, metadata};
use std::io::Read;
use std::collections::VecDeque;
use std::fmt;

pub const STACKSIZE: u64 = 4096u64;
pub const INST_LEN: u64 = 4u64;
pub type CpuResult = Result<(), Exception>;


#[derive(Debug)]
pub struct ProgramBuffer {
    pub cursor: usize,
    pub buf: Vec<u8>
}

pub struct Cpu<S: Scheduler> {
    pub core: SoftThread<u64, f64, Dram>,
    ext: Extension,
    pb: ProgramBuffer,
    queue: VecDeque<Box<dyn Fn() -> Vec<u8>>>,
    scheduler: S
    //TODO: Replace core with multi core structure
    //TODO: Add task scheduler to communicate tasks to multiple cores from queue.
}

impl<S: Scheduler> Cpu<S> {
    pub fn new() -> Cpu<S> {
        Cpu::default()    
    }

    pub fn run(&mut self) -> CpuResult {
        //TODO: Check if there's anything in ProgramBuffer if so we need
        //to load from the cursor position, to the either the end of the program
        //or the max stack size, move the cursor forward to the next starting position
        //if there's anything remaining, if not then run to end of program before loading
        //next program.
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

    pub fn load_from_state<T: StateObject>(&mut self, state: T, addr: T::Address) -> CpuResult {
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

impl<S: Scheduler> Default for Cpu<S> {
    fn default() -> Cpu<S> {
        let mut softs = SoftThread::<u64, f64, Dram>::default(); 
        Cpu {
            core: softs,
            ext: Extension::G,
            pb: ProgramBuffer::default(),
            queue: VecDeque::new(),
            scheduler: S::default(),
        }
    }
}

impl<S: Scheduler> fmt::Display for Cpu<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cpu {{")?;
        writeln!(f, "core: {:?},", self.core)?;
        writeln!(f, "ext: {:?},", self.ext)?;
        writeln!(f, "pb: {:?},", self.pb)?;
        writeln!(f, "queue {:?}", self.queue.len())?;
        writeln!(f, "}}")?;

        Ok(())
    }     
}

impl<S: Scheduler> fmt::Debug for Cpu<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cpu {{")?;
        writeln!(f, "core: {:?},", self.core)?;
        writeln!(f, "ext: {:?},", self.ext)?;
        writeln!(f, "pb: {:?},", self.pb)?;
        writeln!(f, "queue {:?}", self.queue.len())?;
        writeln!(f, "}}")?;

        Ok(())
    }     
}


