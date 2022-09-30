#![allow(unused, unused_mut, dead_code)]
use crate::encoding::{EncodingTable, InstructionDecoder};
use crate::encoding_types::Inst;
use crate::extensions::{Base, Extension};
use crate::instructions::Instruction;
use crate::register::{Register, RegisterValue};
use crate::memory::{Dram, MEM_SIZE};
use crate::machine::{Machine, Support};
use crate::memory::Memory;

pub struct SoftThread<R, M> {
    pub registers: [R; 33],
    pc: R,
    pub program: Vec<u8>,
    remainder: u32,
    eq_flag: bool,
    enc_table: EncodingTable,
    bus: M,
    csr: [R; 4096]
    res: Vec<u64>,
}

impl SoftThread<u64, Dram> {
    pub fn new(enc_table: EncodingTable) -> SoftThread<u64, Dram> {
        let mut soft = SoftThread {
            registers: [0; 33],
            pc: 0,
            program: vec![],
            remainder: 0,
            eq_flag: false,
            enc_table,
            bus: Dram::default()
            
        };

        soft.registers[2] = MEM_SIZE;
        soft.registers[0] = 0;

        soft
    }

    pub(crate) fn read_reg(&self, idx: u64) -> u64 {
        self.registers[idx as usize]
    }

    pub(crate) fn fetch(&self) -> Inst {
        let mut bytes: [u8; 4] = [
            self.program[(self.pc + 3) as usize],
            self.program[(self.pc + 2) as usize],
            self.program[(self.pc + 1) as usize],
            self.program[self.pc as usize],
        ];
        let inst: Inst = u32::from_le_bytes(bytes);
        return inst;
    }

    pub(crate) fn execute(&mut self) {
        let instruction: Instruction = Instruction::decode(self.fetch(), &self.enc_table);
        match instruction {
            Instruction::Lui { rd, imm } => {
                //load upper immediate
                self.registers[rd as usize] = (imm as i64) as u64
            },
            Instruction::Auipc { rd, imm } => {
                //add upper immediate to program counter
                if let Some(res) = self.pc.checked_add((imm as i64) as u64) {
                    self.registers[rd as usize] = res
                }
            },
            Instruction::Jal { rd, imm } => {
                // Jump and link
                self.registers[rd as usize] = self.pc.wrapping_add(4);
                self.pc = self.pc.wrapping_add((imm as i64) as u64);
            },
            Instruction::Jalr { rd, rs1, imm } => {
                // Jump and link register
                let t = self.pc.wrapping_add(4);
                self.pc = (self.registers[rs1 as usize].wrapping_add((imm as i64) as u64) & !1);
                self.registers[rd as usize] = t;
            },
            Instruction::Beq { rs1, rs2, imm, .. } => {
                // Branch if equal
                if self.registers[rs1 as usize] == self.registers[rs2 as usize] {
                    self.pc = self.pc.wrapping_add((imm as i64) as u64);
                }
            },
            Instruction::Bne { rs1, rs2, imm, .. } => {
                // Branch if not equal
                if self.registers[rs1 as usize] != self.registers[rs2 as usize] {
                    self.pc = self.pc.wrapping_add((imm as i64) as u64);
                }
            },
            Instruction::Blt { rs1, rs2, imm, .. } => {
                // Branch if less than
                if (self.registers[rs1 as usize] as i64) < (self.registers[rs2 as usize] as i64) {
                    self.pc = self.pc.wrapping_add((imm as i64) as u64);
                }
            },
            Instruction::Bge { rs1, rs2, imm, .. } => {
                // Branch if greater or equal
                if (self.registers[rs1 as usize] as i64) >= (self.registers[rs2 as usize] as i64) {
                    self.pc = self.pc.wrapping_add((imm as i64) as u64);
                }
            },
            Instruction::Bltu { rs1, rs2, imm, .. } => {
                // Branch if less than unsigned
                if self.registers[rs1 as usize] < self.registers[rs2 as usize] {
                    self.pc = self.pc.wrapping_add((imm as i64) as u64);
                }
            },
            Instruction::Bgeu { rs1, rs2, imm, .. } => {
                // Branch if greater than unsigned
                if self.registers[rs1 as usize] >= self.registers[rs2 as usize] {
                    self.pc = self.pc.wrapping_add((imm as i64) as u64);
                }
            },
            Instruction::Lb { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(&addr.into(), 8) {
                    self.registers[rd as usize] = ((self.bus.into_u64(&val)) as i64) as u64;
                }
            },
            Instruction::Lh { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(&addr.into(), 16) {
                    self.registers[rd as usize] = ((self.bus.into_u64(&val)) as i64) as u64;
                }
            },
            Instruction::Lw { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(&addr.into(), 32) {
                    self.registers[rd as usize] = ((self.bus.into_u64(&val) as i32) as i64) as u64
                }
            },
            Instruction::Lbu { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(&addr.into(), 8) {
                    self.registers[rd as usize] = self.bus.into_u64(&val);
                }
            },
            Instruction::Lhu { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(&addr.into(), 16) {
                    self.registers[rd as usize] = self.bus.into_u64(&val);
                }
            },
            Instruction::Sb { rs1, rs2, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                let _ = self.bus.write(addr, self.registers[rs2 as usize], 8);
            },
            Instruction::Sh { rs1, rs2, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                let _ = self.bus.write(addr, self.registers[rs2 as usize], 16);
            },
            Instruction::Sw { rs1, rs2, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                let _ = self.bus.write(addr, self.registers[rs2 as usize], 32);
            },
            Instruction::Addi { rd, rs1, imm, .. } => {
                let imm = (imm as i64) as u64;
                if let Some(res) = self.registers[rs1 as usize].checked_add(imm) {
                    self.registers[rd as usize] = res;
                }
            },
            Instruction::Slti { rd, rs1, imm, .. } => {
                self.registers[rd as usize] = if (self.registers[rs1 as usize] as i64) < (imm as i64) {
                    1
                } else {
                    0
                };
            },
            Instruction::Sltiu { rd, rs1, imm, .. } => {
                self.registers[rd as usize] = if self.registers[rs1 as usize] < ((imm as i64) as u64) { 1 } else { 0 };
            },
            Instruction::Xori { rd, rs1, imm, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize] ^ ((imm as i64) as u64);
            },
            Instruction::Ori { rd, rs1, imm, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize] | ((imm as i64) as u64);
            },
            Instruction::Andi { rd, rs1, imm, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize] & ((imm as i64) as u64);
            },
            Instruction::Slli { rd, rs1, shamt, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].wrapping_shl(shamt);
            },
            Instruction::Srli { rd, rs1, shamt, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].wrapping_shr(shamt);
            },
            Instruction::Srai { rd, rs1, shamt, .. } => {
                self.registers[rd as usize] = (self.registers[rs1 as usize] as i64).wrapping_shr(shamt) as u64;
            },
            Instruction::Add { rd, rs1, rs2, .. } => {
                self.registers[rs1 as usize].overflowing_add(self.registers[rs2 as usize])
            },
            Instruction::Sub { rd, rs1, rs2, .. } => {
                self.registers[rs1 as usize].overflowing_sub(self.registers[rs2 as usize])  
            },
            Instruction::Sll { rd, rs1, rs2, .. } => {
                let shamt = ((self.registers[rs2 as usize] & 0x3f) as u64) as u32;
                self.registers[rd as usize] = self.registers[rs1 as usize].wrapping_shl(shamt);
            },
            Instruction::Slt { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = if ((self.registers[rs1 as usize] as i64) < (self.registers[rs2 as usize] as i64)) {
                    1 
                } else {
                    0
                };
            },
            Instruction::Sltu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = if self.registers[rs1 as usize] < self.registers[rs2 as usize] {
                    1
                } else {
                    0
                };
            },
            Instruction::Xor { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize] ^ self.registers[rs2 as usize]
            },
            Instruction::Srl { rd, rs1, rs2, .. } => {
                let shamt = ((self.registers[rs2 as usize] & 0x3f) as u64) as u32;
                self.registers[rd as usize] = self.registers[rs1 as usize].wrapping_shr(shamt);
            },
            Instruction::Sra { rd, rs1, rs2, .. } => {
                let shamt = ((self.registers[rs2 as usize] & 0x3f) as u64) as u32;
                self.registers[rd as usize] = (self.registers[rs1 as usize] as i64).wrapping_shr(shamt) as u64;
            },
            Instruction::Or { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize] | self.registers[rs2 as usize];
            },
            Instruction::And { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize] & self.registers[rs2 as usize];
            },
            Instruction::Fence { .. } => { todo!() }
            Instruction::ECall => { 
                // TODO: Call self.ecall() once machine is impl on SoftThread
                todo!()
            },
            Instruction::EBreak => {
                // TODO: Call ebreak() on debugger once debugger is added into SoftThread
                todo!()
            },
            Instruction::Lwu { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(&addr.into(), 32) {
                    self.registers[rd as usize] = self.bus.into_u64(&val);
                }
            },
            Instruction::Ld { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(&addr.into(), 64) {
                    self.registers[rd as usize] = self.bus.into_u64(&val);
                }
            },
            Instruction::Sd { rs1, rs2, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                let _ = self.bus.write(addr, self.registers[rs2 as usize], 64);
            },
            Instruction::Addiw { rd, rs1, imm, .. } => {
                self.registers[rd as usize] = ((self.registers[rs1 as usize].wrapping_add(((imm as i64) as u64)) as i32) as i64) as u64;
            },
            Instruction::Slliw { rd, rs1, shamt, .. } => {
                self.registers[rd as usize] = ((self.registers[rs1 as usize].wrapping_shl(shamt) as i32) as i64) as u64;
            },
            Instruction::Sraiw { rd, rs1, shamt, .. } => {
                self.registers[rd as usize] = ((self.registers[rs1 as usize] as i32).wrapping_shr(shamt) as i64) as u64;
            },
            Instruction::Addw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = ((self.registers[rs1 as usize].wrapping_add(self.registers[rs2 as usize]) as i32) as i64) as u64;
            },
            Instruction::Subw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = ((self.registers[rs1 as usize].wrapping_sub(self.registers[rs2 as usize]) as i32) as i64) as u64;
            },
            Instruction::Sllw { rd, rs1, rs2, .. } => {
                let shamt = ((self.registers[rs2 as usize] & 0x3f) as u64) as u32;
                self.registers[rd as usize] = ((self.registers[rs1 as usize] as u32).wrapping_shl(shamt) as i32) as u64;
            },
            Instruction::Srlw { rd, rs1, rs2, .. } => {
                let shamt = ((self.registers[rs2 as usize] & 0x3f) as u64) as u32;
                self.registers[rd as usize] = ((self.registers[rs1 as usize] as u32).wrapping_shr(shamt) as i32) as u64;
            },
            Instruction::Sraw { rd, rs1, rs2, .. } => {
                let shamt = ((self.registers[rs2 as usize] & 0x3f) as u64) as u32;
                self.registers[rd as usize] = ((self.registers[rs1 as usize] as i32) >> (shamt as i32)) as u64;
            },
            Instruction::FenceI { .. } => { todo!() },
            Instruction::Csrrw { csr, rs1, rd, .. } => {
                if rd != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    csr_val.zero_extend();
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.registers[rs1 as usize]
                }
            },
            Instruction::Csrrs { csr, rs1, rd, .. } => {
                if rs1 != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    csr_val.zero_extend();
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] | self.registers[rs1 as usize];    
                }                
            },
            Instruction::Csrrc { csr, rs1, rd, .. } => {
                if rs1 != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    csr_val.zero_extend();
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] & self.registers[rs1 as usize];
                }
            },
            Instruction::Csrrwi { rd, csr, uimm, .. } => {
                if rd != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    let imm = uimm.zero_extend();
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = imm;
                }
            },
            Instruction::Csrrsi { rd, csr, uimm, .. } => {
                if uimm != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    let imm = uimm.zero_extend();
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] | imm;
                }
            },
            Instruction::Csrrci { rd, csr, uimm, .. } => {
                if uimm != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    let imm = uimm.zero_extend();
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] & imm;
                }
            },
            Instruction::Mul { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_mul(self.registers[rs2 as usize]);
            },
            Instruction::Mulh { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_mul_high_signed(self.registers[rs2 as usize]);
            },
            Instruction::Mulhsu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_mul_high_signed_unsigned(self.registers[rs2 as usize]);
            },
            Instruction::Mulhu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_mul_high_unsigned(self.registers[rs2 as usize]);
            },
            Instruction::Div { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_div_signed(self.registers[rs2 as usize]);
            },
            Instruction::Divu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_div(self.registers[rs2 as usize]);
            },
            Instruction::Rem { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_rem_signed(self.registers[rs2 as usize]);
            },
            Instruction::Remu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_rem(self.registers[rs2 as usize]);
            },
            Instruction::Mulw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_mul(self.registers[rs2 as usize]);
            },
            Instruction::Divw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_div_signed(self.registers[rs2 as usize]);
            },
            Instruction::Divuw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_div(self.registers[rs2 as usize]);
            },
            Instruction::Remw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_rem_signed(self.registers[rs2 as usize]);
            },
            Instruction::RemuW { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].overflowing_rem(self.registers[rs2 as usize]);
            },
            // For W instructions below ALL words being read from
            // memory must be naturally aligned to 32 bits
            // i.e. mod 4 == 0;
            // For D instructions belwo ALL doublewords being
            // read from memory most be naturally aligned to
            // 64 bit words, i.e. mod 8 == 0;
            Instruction::LrW { rd, rs1, .. } => {

                // Test whether vec or hashset is best suited for this.
                let addr = self.registers[rs1 as usize];

                if addr % 4 != 0 {
                    // TODO: Reject
                    todo!();
                }
                
                let val = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                
                self.registers[rd as usize] = val;
                self.reservation.push(self.registers[rs1 as usize]);

            },
            Instruction::ScW { rd, rs1, rs2, .. } => {
                // if an address reservation is still value
                // and contains the bytes being written
                // then write the word in rs2 to addr in
                // rs1, and set rd to zero.
                // otherwise write a nonzero value to rd.
                // Invalidate any reservation held be this
                // thread.
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    //TODO: Reject
                    todo!();
                }
                    
                if self.reservation.contains(addr) {
                    self.reservation.retain(|x| x != addr);
                    let word = self.registers[rs2 as usize];
                    self.bus.write(addr, word, 32);
                    self.registers[rd as usize] = 0;
                } else {
                    self.reservation.retain(|x| x != addr);
                    self.registers[rd as usize] = 1;
                }
            },
            Instruction::AmoswapW { rd, rs1, rs2, ..} => {
                // read a word from the address in rs1
                // write the value in rs2 register to
                // address in rs1, take value from rs1 and
                // sign extend then store in rd
                let addr = self.registers[rs1 as usize];

                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                let val = self.registers[rs2 as usize];
                
                self.bus.write(addr, val, 32);
                self.registers[rd as usize] = temp;  
            },
            Instruction::AmoaddW { rd, rs1, rs2, ..} => {
                // read word from address in rs1
                // add the value from rs2 to the word
                // read at rs1 address and save result
                // in memory at address in rs1. Write
                // previous value in address at rs1
                // to rd.
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp + val;
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp; 
            },
            Instruction::AmoxorW { rd, rs1, rs2, .. } => {
                // read word from address in rs1
                // xor the word against the value in rs2
                // save the original value found at address
                // in rs1 to rd. Save the xor value in the
                // memory at the address from rs1.
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp ^ val;
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmoandW { rd, rs1, rs2, .. } => {
                // read word from address in rs1
                // bitwise and word against the value in rs2
                // save the original value found at address
                // in rs1 to rd. Save the bitwise and'd value
                // in the memory at the address from rs1.
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp & val;
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmoorW { rd, rs1, rs2, .. } => {
                // read word from address in rs1
                // bitwise or word against value in rs2
                // save the original value found at address
                // in rs1 to rd. save the bitwise or'd value
                // in the memory at the address from rs1.
                let addr = self.registers[rs1 as usize];

                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp | val;
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmominW { rd, rs1, rs2, .. } => {
                // read word from address in rs1
                // compare the value of the word to the
                // value in rs2 and save the lowest value
                // to memory at the address in rs1.
                // store the original word at address in rs1
                // to rd. 
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::min(temp, val);
                
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmomaxW { rd, rs1, rs2, .. } => {
                // read word from address in rs1
                // compare the value of the word to the
                // value in rs2. Store the highest value
                // in memory at the address in rs1.
                // store the original word atw address in rs1
                // to rd.
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = ((self.bus.read(addr, 32) as i32) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::max(temp, val);
                
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmominuW { rd, rs1, rs2, .. } => {
                // read word from address in rs1
                // compare the unsigned value to an unsigned
                // value in rs2. Store the lowest value to
                // memory at the address in rs1
                // store the original word at address in rs1
                // to rd.
                let addr = self.registers[rs1 as usize];
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                let temp = self.bus.read(addr, 32) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::min(temp, val);
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmomaxuW { rd, rs1, rs2, .. } => {
                // read word from address in rs1
                // compare the unsigned value to an unsigned
                // value in rs2. Store the higheste value to
                // memory at the address in rs1
                // store the original word at address in rs1
                // to rd.
                let addr = self.registers[rs1 as usize];
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                let temp = self.bus.read(addr, 32) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::max(temp, val);
                self.bus.write(addr, res, 32);
                self.registers[rd as usize] = temp;
            },
            Instruction::LrD { rd, rs1 .. } => {
                // See LrD, but instead of reading word
                // from address at rs1, read double word.
                let addr = self.registers[rs1 as usize];
                let val = (self.bus.read(addr, 64) as i64) as u64;
                if addr % 8 != 0 {
                    // TODO: Reject
                    todo!();
                }
                self.registers[rd as usize] = val;
                self.reservation.push(self.registers[rs1 as usize]);
            },
            Instruction::ScD { rd, rs1, rs2, .. } => {
                // See ScW, but instead of conditionally
                // saving a word, save a double word.
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    //TODO: Reject
                    todo!();
                }
                    
                if self.reservation.contains(addr) {
                    self.reservation.retain(|x| x != addr);
                    let dword = self.registers[rs2 as usize];
                    self.bus.write(addr, dword, 64);
                    self.registers[rd as usize] = 0;
                } else {
                    self.reservation.retain(|x| x != addr);
                    self.registers[rd as usize] = 1;
                }
            },
            Instruction::AmoswapD { rd, rs1, rs2, ..} => {
                // read a doubleword from the address in rs1
                // write the value in rs2 register to
                // address in rs1, take value from rs1 and
                // sign extend then store in rd
                let addr = self.registers[rs1 as usize];

                if addr % 8 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = (self.bus.read(addr, 64) as i64) as u64;
                let val = self.registers[rs2 as usize];
                
                self.bus.write(addr, val, 64);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmoaddD { rd, rs1, rs2, ..} => {
                // read doubleword from address in rs1
                // add the value from rs2 to the doubleword
                // read at rs1 address and save result
                // in memory at address in rs1. Write
                // previous value in address at rs1
                // to rd.
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = (self.bus.read(addr, 64) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp + val;
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmoxorD { rd, rs1, rs2, .. } => {
                // read doubleword from address in rs1
                // xor the doubleword against the value in rs2
                // save the original value found at address
                // in rs1 to rd. Save the xor value in the
                // memory at the address from rs1.
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = (self.bus.read(addr, 64) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp ^ val;
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmoandD { rd, rs1, rs2, .. } => {
                // read doubleword from address in rs1
                // bitwise and doubleword against the value in rs2
                // save the original value found at address
                // in rs1 to rd. Save the bitwise and'd value
                // in the memory at the address from rs1.
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = (self.bus.read(addr, 64) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp & val;
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmoorD { rd, rs1, rs2, .. } => {
                // read doubleword from address in rs1
                // bitwise or doubleword against value in rs2
                // save the original value found at address
                // in rs1 to rd. save the bitwise or'd value
                // in the memory at the address from rs1.
                let addr = self.registers[rs1 as usize];

                if addr % 8 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = (self.bus.read(addr, 64) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = temp | val;
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmominD { rd, rs1, rs2, .. } => {
                // read doubleword from address in rs1
                // compare the value of the doubleword to the
                // value in rs2 and save the lowest value
                // to memory at the address in rs1.
                // store the original doubleword at address in rs1
                // to rd. 
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = (self.bus.read(addr, 64) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::min(temp, val);
                
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmomaxD { rd, rs1, rs2, .. } => {
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = (self.bus.read(addr, 64) as i64) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::max(temp, val);
                
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
                
            },
            Instruction::AmominuD { rd, rs1, rs2, .. } => {
                // read doubleword from address in rs1
                // compare the unsigned value to an unsigned
                // value in rs2. Store the lowest value to
                // memory at the address in rs1
                // store the original doubleword at address in rs1
                // to rd.
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = self.bus.read(addr, 64) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::min(temp, val);
                
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
            },
            Instruction::AmomaxuD { rd, rs1, rs2, .. } => {
                // read doubleword from address in rs1
                // compare the unsigned value to an unsigned
                // value in rs2. Store the higheste value to
                // memory at the address in rs1
                // store the original doubleword at address in rs1
                // to rd.
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let temp = self.bus.read(addr, 64) as u64;
                let val = self.registers[rs2 as usize];
                let res = std::cmp::max(temp, val);
                
                self.bus.write(addr, res, 64);
                self.registers[rd as usize] = temp;
            },
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn load_program(&mut self, code: Vec<u8>) {
        self.program = code;
        //TODO: Need to check size of program.
        // Need to be able to load from file
    }
}



impl Default for SoftThread<u64, Dram> {
    fn default() -> SoftThread<u64, Dram> {
        let enc_table = EncodingTable::default();
        SoftThread::<u64, Dram>::new(enc_table)
    }
}
