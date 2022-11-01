use crate::state::StateObject;
use std::error::Error;

/// Core Local Interruptor (CLINT)
pub struct CoreLocalInterruptor {
    /// Machine Mode Software Interrupt Pending Register (0x0000 for hart 0)
    /// used to assert a software interrupt for a CPU
    msip: u32,
    /// Memory Mapped machine mode timer compare register (0x4000 for hart 0)
    /// used to trigger an interrupt when mtimecmp is greater than or equal
    /// to mtime.
    mtimecmp: u64,
    /// Machine mode timer register (0xbff8 for hart 0) which runs at constant
    /// frequency
    mtime: u64,
}

impl CoreLocalInterruptor {
    pub const CLINT_BASE: u64 = 0x200_0000;
    pub const MSIP_START: u64 = Self::CLINT_BASE;
    pub const MSIP_END: u64 = Self::MSIP_START + 0x4;
    pub const MTIMECMP_START: u64 = Self::CLINT_BASE + 0x4000;
    pub const MTIMECMP_END: u64 = Self::MTIMECMP_START + 0x8;
    pub const MTIME_START: u64 = Self::CLINT_BASE + 0xbff8;
    pub const MTIME_END: u64 = Self::MTIME_START + 0x8;
    
    pub fn increment<S: StateObject>(&mut self, state: &mut }
