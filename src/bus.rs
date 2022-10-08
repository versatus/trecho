use crate::memory::{Memory, ReadOnlyMemory};
use crate::{clint::Clint, plic::Plic, uart::Uart, vio:::Vio};

#[derive(Clone, Debug)]
pub struct Bus<C, P, U, I, M, R>
where
    C: Clone + Clint,
    P: Clone + Plic,
    U: Clone + Uart,
    I: Clone + Vio,
    M: Clone + Memory,
    R: Clone + ReadOnlyMemory
{
    pub clint: C,
    pub plic: P,
    pub uart: U,
    pub io: I,
    pub dram: M,
    rom: R,
}


impl<C, P, U, I, M, R> Bus<C, P, U, I, M, R> {
    pub fn new() -> Self {
        Bus {
            clint: C::default(),
            plic: P::default(),
            uart: U::default(),
            io: I::default(),
            dram: M::default(),
            rom: R::default()
        }
    }
} 

