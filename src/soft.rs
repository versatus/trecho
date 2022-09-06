#![allow(unused, unused_mut, dead_code)]
use crate::register::Register;

pub struct SoftThread {
    pub registers: [u32; 33],
    pc: u32,
    pub program: Vec<u8>,
    remainder: u32,
    eq_flag: bool,
}