pub trait Clint: Default {
    const MSIP_START: u64;
    const MSIP_END: u64;
    const MTIMECMP_START: u64;
    const MTIMECMP_END: u64;
    const MTIME_START: u64;
    const MTIME_END: u64;
    type Msip;
    type Mtimecmp;
    type Mtime;
    type State;
    type Exception: std::error::Error;
    
    fn increment(&mut self, state: &mut Self::State);
    fn read(&self, addr: u64, size: u8) -> Result<u64, Self::Exception>;
    fn write(&mut self, addr; u64, value: u64, size: u8) -> Result<(), Self::Exception>;
    fn new() -> Self {
        Self::default()
    }
}

