#![allow(unused, unused_mut, dead_code)]
use crate::soft::SoftThread;
use crate::extensions::{I32, I64, Ext};

pub struct Machine<E: Ext> {
    pub cores: Vec<SoftThread>,
    ext: E,
}