use std::fmt;
use std::ops::RangeInclusive;


pub struct Csr {
    registers: [u64; 4096] 
}

impl CsrRegister {
    /// Custom type of address for CSR Addresses
    pub type Address = u16;
    /// Custom Type for Field Range for CSRs 
    pub type FieldRange = RangeInclusive<usize>;
    pub const MXLEN: usize = 64;
    /// Max number of csrs is 2**12 = 4096
    pub const CSR_SIZE: usize = 4096;
    /*==========================*/
    /* User-level CSR Addresses */
    /*==========================*/ 
    /// User Status Register
    pub const USER_STATUS: Self::Address = 0x000;
    /// User Trap handler Base Address
    pub const USER_TRAP_BASE: Self::Address = 0x005;
    /// User Exception Program Counter
    pub const USER_EXCEPTION_PC: Self::Address = 0x041;
    /// User cause of trap
    pub const TRAP_CAUSE_USER: Self::Address = 0x042;
    /// Bad address or instruction
    pub const BAD_ADD_OR_INST: Self::Address = 0x043;
    /*==========================*/
    /* Floating Point User CSRs */
    /*==========================*/
    /// Accrued FP exceptions
    pub const FLOAT_FLAGS: Self::Address = 0x001;
    /// Dynamic Rounding Mode for Floating Point Numbers
    pub const FLOAT_ROUND_MODE: Self::Address = 0x002;
    /// Control/Status Register for FPs (RoundingMode + Flags) 
    pub const FLOAT_CSR: Self::Address = 0x003;
    /*===========================*/
    /*    User Counter/Timer     */
    /*===========================*/
    pub const TIME: Self::Address = 0xc01;
    
    /*===========================*/
    /*  Supervisor CSR Addresses */
    /*===========================*/
    /// Supervisor trap hanlding Addresses
    /// Supervisor Status Registers
    pub const SUPER_STATUS: Self::Address = 0x100;
    /// Supervisor Exception delegation register
    pub const SUPER_EXCEPTION_DELEGATION: Self::Address = 0x102;
    /// Supervisor Interrupt Delegation Register 
    pub const SUPER_INT_DELEGATION: Self::Address = 0x103;
    /// Supervisor Interrupt-Enable Register 
    pub const SUPER_INT_ENABLE: Self::Address = 0x104;
    /// Supervisor trap handler base address 
    pub const SUPER_TRAP_HANDLER_BASE: Self::Address = 0x105;
    /// Scratch register for supervisor trap handling
    pub const SUPER_SCRATCH: Self::Address = 0x140;
    /// Supervisor Exception PC 
    pub const SUPER_ECP: Self::Address = 0x141;
    /// Supervisor Cause of Trap
    pub const SUPER_CAUSE: Self::Address = 0x142;
    /// Supervisor Bad Address or instruction
    pub const SUPER_BAD_ADD_OR_INST: Self::Address = 0x143;
    /// Supervisor Pending Interrupt
    pub const SUPER_INT_PENDING: Self::Address = 0x144;

    /*==========================*/
    /*Supervisor Protection &   */
    /*Translation               */
    /*==========================*/
    /// Supervisor Address Translation & Protection
    pub const SATP: Self::Address = 0x180;
    
    /// Supervisor Status Fields
    pub const SUPER_STATUS_SIE_MASK: u64 = 0x2; // SuperStatus[1]
    pub const SUPER_STATUS_SPIE_MASK: u64 = 0x20; // SuperStatus[5]
    pub const SUPER_STATUS_UBE_MASK: u64 = 0x40; // SuperStatus[6]
    pub const SUPER_STATUS_SPP_MASK: u64 = 0x100; // SuperStatus[8]
    pub const SUPER_STATUS_FS_MASK: u64 = 0x6000; // SuperStatus[14:13]
    pub const SUPER_STATUS_XS_MASK: u64 = 0x18000; //SuperStatus[16:16]
    pub const SUPER_STATUS_SUM_MASK: u64 = 0x40000; //SuperStatus[18]
    pub const SUPER_STATUS_MXR_MASK: u64 = 0x80000; //SuperStatus[19]
    pub const SUPER_STATUS_UXL_MASK: u64 = 0x3_0000_0000; //SuperStatus[33:32]
    pub const SUPER_STATUS_SD_MASK: u64 = 0x8000_0000_0000_0000; //SuperStatus[63] 
    pub const SUPER_STATUS_MASK: u64 = SUPER_STATUS_SIE_MASK
        | SUPER_STATUS_SPIE_MASK
        | SUPER_STATUS_UBE_MASK
        | SUPER_STATUS_SPP_MASK
        | SUPER_STATUS_FS_MASK
        | SUPER_STATUS_XS_MASK
        | SUPER_STATUS_SUM_MASK
        | SUPER_STATUS_MXR_MASK
        | SUPER_STATUS_UXL_MASK
        | SUPER_STATUS_SD_MASK;

    /// Global interrupt-enable bit for supervisor Mode
    pub const XSTATUS_SIE: Self::FieldRange = 1..=1;
    /// Previous Interrupt Enable bit 
    pub const XSTATUS_SPIE: Self::FieldRange = 5..=5;
    /// Previous Privilege mode for supervisor Mode
    pub const XSTATUS_SPP: Self::FieldRange = 8..=8;

    /*========================*/
    /*  Machine-Level CSRs    */
    /*========================*/


    /*========================*/
    /*  Machine Information   */
    /*  Registers             */
    /*========================*/
    /// Vendor ID
    pub const MACHINE_VENDOR_ID: Self::Address = 0xf11;
    /// Architecture ID 
    pub const MACHINE_ARCH_ID: Self::Address = 0xf12;
    /// Implementation ID 
    pub const MACHINE_IMP_ID: Self::Address = 0xf13;
    /// Machine Hardware Thread ID 
    pub const MACHINE_HART_ID: Self::Address = 0xf14;
    

    /*========================*/
    /* Machine Trap Registers */
    /*========================*/
    /// Machine Status Register 
    pub const MACHINE_STATUS: Self::Address = 0x300;
    /// Machine ISA and Ext 
    pub const MACHINE_ISA_EXT: Self::Address = 0x301;
    /// Machine Exception Deflation Register 
    pub const MACHINE_EXCEPTION_DEF_REG: Self::Address = 0x302;
    /// Machine Interrupt Deflation Register 
    pub const MACHINE_INTERRUPT_DEF_REG: Self::Address = 0x303;
    /// Machine Interrupt Enable Register 
    pub const MACHINE_INTERRUPT_ENABLE_REG: Self::Address = 0x304;
    /// Machine trap handler base address 
    pub const MACHINE_TRAP_HANDLER_BASE: Self::Address = 0x305;
    /// Machine Counter Enabler 
    pub const MACHINE_COUNTER_ENABLER: Self::Address = 0x306;
    
    /*=========================*/
    /* Machine Trap Handling   */
    /*=========================*/
    /// Scratch register for Machine Trap Handlers
    pub const MACHINE_SCRATCH_REGISTER: Self::Address = 0x340;
    /// Machine exception program counter 
    pub const MACHINE_EXCEPTION_PC: Self::Address = 0x341;
    /// Machine Cause of Trap 
    pub const MACHINE_TRAP_CAUSE: Self::Address = 0x342;
    /// Machine Bad Address or Instruction
    pub const MACHINE_BAD_ADD_OR_INT: Self::Address = 0x343;
    /// Machine interrupt pending. 
    pub const MACHINE_INTERRUPT_PENDING: Self::Address = 0x344;

    /*========================*/
    /* Machine Status Fields  */
    /*========================*/
    /// Global Interrupt Enable Bit for machine mode 
    pub const MACHINE_STATUS_INTERRUPT_ENABLE: Self::FieldRange = 3..=3;
    /// Global Previous Iterrupt Enable Bit for Machine Mode 
    pub const MACHINE_STATUS_PREV_INT_ENABLE: Self::FieldRange = 7..=7;
    /// Previous Privilege Mode for Machine mode
    pub const MACHINE_PREVIOUS_PRIV_MODE: Self::FieldRange = 11..=12;
    /// Modify Privilege Bit 
    pub const MACHINE_PRIV: Self::FieldRange = 17..=17;

    /*=========================*/
    /* Machine Interrupt       */
    /* Pending                 */
    /*=========================*/
    /// Supervisor software interrupt
    pub const SUPER_SOFT_INT_BIT: u64 = 1 << 1;
    /// Machine Software interrupt
    pub const MACHINE_SOFT_INT_BIT: u64 = 1 << 3;
    /// Supervisor timer interrupt 
    pub const SUPER_TIMER_INT_BIT: u64 = 1 << 5;
    /// Machine Timer Interrupt 
    pub const MACHINE_TIMER_INT_BIT: u64 = 1 << 7;
    /// Super external interrupt 
    pub const SUPER_EXT_INTERRUPT: u64 = 1 << 9;
    /// Machine external interrupt 
    pub const MACHINE_EXT_INTERRUPT: u64 = 1 << 11;


    pub fn new() -> Self {
        let mut register = [0; Self::CSR_SIZE];
        let misa = (2 << 62) | (1 << 20) | (1 < 18) | (1 << 12) |
            (1 << 8) | (1 << 5) | (1 << 3) | (1 << 2) | 1;
        registers[Self::MACHINE_ISA_EXT as usize] = misa;

        Self { registers }
    }

    pub fn increment_time(&mut self) {
        self.registers[TIME as usize] = self.registers[TIME as usize].wrapping_add(1);
    }
    
    pub fn read(&self, addr: Self::Address) -> u64 {
        
        match addr {
            Self::SUPER_STATUS => {
                self.registers[Self::MACHINE_STATUS as usize] & 
                SUPER_STATUS_MASK
            },
            Self::SUPER_INT_ENABLE => {
                self.registers[Self::MACHINE_INTERRUPT_ENABLE_REG as usize] &
                self.regiisters[Self::MACHINE_INTERRUPT_DEF_REG as usize]
            }
            _ => {
                self.registers[addr as usize]
            }
        }

    }

    pub fn write(&mut self, addr: Self::Address, val: u64) {
        match addr {
            Self::MACHINE_VENDOR_ID => {}
            Self::MACHINE_ARCH_ID => {}
            Self::MACHINE_IMP_ID => {}
            Self::MACHINE_HART_ID => {}
            Self::SUPER_STATUS => {
                self.registers[
                    Self::MACHINE_STATUS as usize
                ] = (self.registers[
                        Self::MACHINE_STATUS as usize
                    ] & !SUPER_STATUS_MASK) | (val & SUPER_STATUS_MASK);
            }
            Self::SUPER_INT_ENABLE => {
                (self.registers[
                 Self::MACHINE_INTERRUPT_ENABLE_REG as usize
                ] & !self.registers[
                Self::MACHINE_INTERRUPT_DEF_REG as usize
                ]) | (val & self.registers[
                Self::MACHINE_INTERRUPT_DEF_REG as usize
                ]);
            }
            Self::SUPER_INT_PENDING => {
                let mask = Self::SUPER_SOFT_INT_BITf & self.registers[
                    Self::MACHINE_INTERRUPT_DEF_REG as usize
                ];
                self.registers[Self::MACHINE_INTERRUPT_ENABLE_REG as usize] = (
                    self.registers[Self::MACHINE_INTERRUPT_PENDING as usize] &
                    !mask) | (val & mask);
            }
            _ => self.registers[addr as usize] = val
        }
    }
}

