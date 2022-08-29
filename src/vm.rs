#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;

pub struct Machine {
    pub cores: Vec<SoftThread>,
}