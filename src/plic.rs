pub trait Plic: Default {
    const SRC_PRIORITY_START: u64;
    const SRC_PRIORITY_END: u64;
    const PENDING_START: u64;
    const PENDING_END: u64;
    const ENABLE_START: u64;
    const ENABLE_END: u64;
    const TRESH_CLAIM_START: u64;
    const THRES_CLAIM_END: u64;
    const WORD_SIZE: u64;
    const CTX_OFFSET: u64;
    const SRC_NUM: u64;
    type Exception: std::error::Error;

    fn update_pending(&mut self, irq: u64);
    fn clear_pending(&mut self, irq: u64);
    fn update_claim(&mut self, irq: u64);
    fn is_enable(&self, ctx: u64, irq: u64);
    fn read(&self, addr: u64, size: u8) -> Result<u64, Self::Exception>;
    fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception>;
    fn new() -> Self {
        Self::default()
    }
}
