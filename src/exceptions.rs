use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Exception {
    AddressMisaligned,
    AccessFault,
    Invalid(u64),
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAMOAccessFault,
    StoreAMOAddressMisaligned,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault(u64),
    LoadPageFault(u64),
    StoreAMOPageFault(u64),
    General,
}

#[derive(Debug)]
pub enum Trap {
    Contained,
    Requested,
    Invisible,
    Fatal
}

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Exception {}