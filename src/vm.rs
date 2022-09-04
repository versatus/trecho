#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;
use crate::extensions::{Extension};

pub struct Machine {
    pub cores: Vec<SoftThread>,
    ext: Extension,
}