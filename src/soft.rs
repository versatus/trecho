use crate::register::Register;

pub struct SoftThread {
    pub registers: [u32; 33],
    pc: u32
    pub program: Vec<u8>,
    remainder: u32, // TODO: Implement float instructions to allow for floats without remainder.
    eq_flag: bool,
}