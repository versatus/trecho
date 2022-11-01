use std::fmt::Debug;
use std::error::Error;

pub trait Scheduler: Debug + Default {
    type Result;
    type Exception: Error;
    type State;
    type Queue;
    type Cores;

    // Get the state of all the cores in the VM and see which one is 'least occupied'
    // then submit the next program in the queue (or load into buffer) for execution.
    fn schedule(
        &mut self, 
        state: Self::State, 
        queue: Self::Queue, 
        cores: &mut Self::Cores
        ) -> Result<Self::Result, Self::Exception>;
}


