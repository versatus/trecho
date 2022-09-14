
pub trait Machine {
    type Reg;
    type Mem;
    type Error;

    fn pc(&mut self) -> &Self::Reg;
    fn update_pc(&self, pc: Self::Reg);
    fn commit_pc(&mut self);
    fn memory(&self) -> &Self::Mem;
    fn memory_mut(&mut self) -> &mut Self::Mem;
    fn registers(&self) -> &[Self::Reg];
    fn set_register(&mut self, idx: usize, value: Self::Reg);
    fn base(&self) -> u8;
    fn ext(&self) -> u8;
    fn version(&self) -> u32;
    fn ecall(&mut self) -> Result<(), Self::Error>;
    fn ebreak(&mut self) -> Result<(), Self::Error>;
}

pub trait Support: Machine {
    type Bytes;

    fn cycles(&self) -> u64;
    fn set_cycles(&mut self, cycles: u64);
    fn max_cycles(&self) -> u64;
    fn running(&self) -> bool;
    fn set_running(&mut self, running: bool);
    fn reset(&mut self, max_cycles: u64);
    fn reset_signal(&mut self) -> bool;
    fn add_cycles(&mut self, cycles: u64) -> Result<(), <Self as Machine>::Error>;
    fn add_cycles_no_check(&mut self, cycles: u64) -> Result<(), <Self as Machine>::Error>;
    fn load_inner_elf(&mut self, program: &Self::Bytes, update_pc: bool) -> Result<(), <Self as Machine>::Error>;
    fn load_elf(&mut self, program: &Self::Bytes, update_pc: bool) -> Result<u64, <Self as Machine>::Error>;
    fn init_stack(&mut self, args: &[Self::Bytes], start: u64, size: u64) -> Result<u64, <Self as Machine>::Error>;
    //TODO: Make Feature Enabled 
    fn code(&self) -> &Self::Bytes;
}
