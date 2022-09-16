#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;
use crate::extensions::{Extension};
use crate::memory::Memory;

pub struct Cpu<M: Memory> {
    pub cores: Vec<SoftThread<M>>,
    ext: Extension,
}