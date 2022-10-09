pub trait Uart: Default {
    const UART_IRQ: u64;
    const UART_RECEIVING_HOLDING_REGISTER: u64;
    const UART_TRANSMIT_HOLDING_REGISTER: u64;
    const UART_INTERRUPT_ENABLE_REGISTER: u64;
    const UART_FIFO_CONTROL_REGISTER: u64;
    const UART_INTERRUPT_STATUS_REGISTER: u64;
    const UART_LINE_STATUS_REGISTER: u64;
    const UART_LINE_STATUS_REGISTER_RECEIVER: u64;
    const UART_LINE_STATUS_REGISTER_SENDER: u64;

    type Interrupting;
    type ReceiverTransmitter;
    type Exception;

    fn is_interrupting(&self) -> bool;
    fn read(&mut self, index: u64, size: u8) -> Result<u64, Self::Exception>;
    fn write(&mut self, index: u64, value: u8, size: u8) -> Result<(), Exception>; 
    fn new() -> Self {
        Self::default()
    }
}

