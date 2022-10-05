#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;
use crate::extensions::{Extension};
use crate::memory::Memory;
use crate::register::RegisterValue;

pub struct Cpu<R: RegisterValue, F, M: Memory> {
    pub cores: Vec<SoftThread<R, F, M>>,
    ext: Extension,
}