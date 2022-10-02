#![allow(unused, unused_mut, dead_code)]
use crate::encoding::{EncodingTable, InstructionDecoder};
use crate::encoding_types::Inst;
use crate::extensions::{Base, Extension};
use crate::instructions::Instruction;
use crate::register::{Register, RegisterValue};
use crate::memory::{Dram, MEM_SIZE};
use crate::machine::{Machine, Support};
use crate::memory::Memory;

pub struct SoftThread<R, F, M> {
    pub registers: [R; 33],
    pub f_registers: [F; 33],
    pub pc: R,
    pub program: Vec<u8>,
    pub remainder: u32,
    eq_flag: bool,
    enc_table: EncodingTable,
    pub bus: M,
    pub csr: [R; 4096],
    pub res: Vec<u64>,
}

impl SoftThread<u64, f64, Dram> {
    pub fn new(enc_table: EncodingTable) -> SoftThread<u64, f64, Dram> {
        let mut soft = SoftThread {
            registers: [0; 33],
            f_registers: [0.0; 33]
            pc: 0,
            program: vec![],
            remainder: 0,
            eq_flag: false,
            enc_table,
            csr: [0; 4096],
            bus: Dram::default(),
            res: vec![]
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
        println!("{:?}", instruction);
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
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_add(&self.registers[rs2 as usize])
            },
            Instruction::Sub { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_sub(&self.registers[rs2 as usize]);
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
                    let csr_val = (csr_val as u64).zero_extend(&32);
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.registers[rs1 as usize]
                }
            },
            Instruction::Csrrs { csr, rs1, rd, .. } => {
                if rs1 != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    let csr_val = (csr_val as u64).zero_extend(&32);
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] | self.registers[rs1 as usize];    
                }                
            },
            Instruction::Csrrc { csr, rs1, rd, .. } => {
                if rs1 != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    let csr_val = (csr_val as u64).zero_extend(&32);
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] & self.registers[rs1 as usize];
                }
            },
            Instruction::Csrrwi { rd, csr, uimm, .. } => {
                if rd != Register::X0 {
                    let csr_val = self.csr[csr as usize];
                    let imm = (uimm as u64).zero_extend(&32);
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = imm;
                }
            },
            Instruction::Csrrsi { rd, csr, uimm, .. } => {
                if uimm != Register::X0 as u32 {
                    let csr_val = self.csr[csr as usize];
                    let imm = (uimm as u64).zero_extend(&32);
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] | imm;
                }
            },
            Instruction::Csrrci { rd, csr, uimm, .. } => {
                if uimm != Register::X0 as u32 {
                    let csr_val = self.csr[csr as usize];
                    let imm = (uimm as u64).zero_extend(&32);
                    self.registers[rd as usize] = csr_val;
                    self.csr[csr as usize] = self.csr[csr as usize] & imm;
                }
            },
            Instruction::Mul { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_mul(&self.registers[rs2 as usize]);
            },
            Instruction::Mulh { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_mul_high_signed(&self.registers[rs2 as usize]);
            },
            Instruction::Mulhsu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_mul_high_signed_unsigned(&self.registers[rs2 as usize]);
            },
            Instruction::Mulhu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_mul_high_unsigned(&self.registers[rs2 as usize]);
            },
            Instruction::Div { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_div_signed(&self.registers[rs2 as usize]);
            },
            Instruction::Divu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_div(&self.registers[rs2 as usize]);
            },
            Instruction::Rem { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_rem_signed(&self.registers[rs2 as usize]);
            },
            Instruction::Remu { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_rem(&self.registers[rs2 as usize]);
            },
            Instruction::Mulw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_mul(&self.registers[rs2 as usize]);
            },
            Instruction::Divw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_div_signed(&self.registers[rs2 as usize]);
            },
            Instruction::Divuw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_div(&self.registers[rs2 as usize]);
            },
            Instruction::Remw { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_rem_signed(&self.registers[rs2 as usize]);
            },
            Instruction::RemuW { rd, rs1, rs2, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].oflow_rem(&self.registers[rs2 as usize]);
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
                
                let res = self.bus.read(&addr, 32);
                if let Ok(val) = res {
                    let val = ((val as i32) as i64) as u64;
                    self.registers[rd as usize] = val;
                    self.res.push(self.registers[rs1 as usize]);
                }
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
                    
                if self.res.contains(&addr) {
                    self.res.retain(|x| *x != addr);
                    let word = self.registers[rs2 as usize];
                    self.bus.write(addr, word, 32);
                    self.registers[rd as usize] = 0;
                } else {
                    self.res.retain(|x| *x != addr);
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
                
                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = ((temp as i32) as i64) as u64;
                    let val = self.registers[rs2 as usize];                    
                    let _ = self.bus.write(addr, val, 32);
                    self.registers[rd as usize] = temp;  

                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = ((temp as i32) as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = temp + val;
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp; 
                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = ((temp as i32) as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = temp ^ val;
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp;
                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = ((temp as i32) as i64) as u64;     
                    let val = self.registers[rs2 as usize];
                    let res = temp & val;
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp;
                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = ((temp as i32) as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = temp | val;
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp;
                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = ((temp as i32) as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = std::cmp::min(temp, val);   
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp;
                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = ((temp as i32) as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = std::cmp::max(temp, val);
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp;
                }
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

                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let temp = temp;
                    let val = self.registers[rs2 as usize];
                    let res = std::cmp::min(temp, val);
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp;
                }
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

                if let Ok(temp) = self.bus.read(&addr, 32) {
                    let val = self.registers[rs2 as usize];
                    let res = std::cmp::max(temp, val);
                    let _ = self.bus.write(addr, res, 32);
                    self.registers[rd as usize] = temp;
                }
            },
            Instruction::LrD { rd, rs1, .. } => {
                // See LrD, but instead of reading word
                // from address at rs1, read double word.
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    // TODO: Reject
                    todo!();
                }

                if let Ok(temp) = self.bus.read(&addr, 64) {
                    let val = (temp as i64) as u64;    
                    self.registers[rd as usize] = val;
                    self.res.push(self.registers[rs1 as usize]);
                } 
            },
            Instruction::ScD { rd, rs1, rs2, .. } => {
                // See ScW, but instead of conditionally
                // saving a word, save a double word.
                let addr = self.registers[rs1 as usize];
                
                if addr % 8 != 0 {
                    //TODO: Reject
                    todo!();
                }
                    
                if self.res.contains(&addr) {
                    self.res.retain(|x| *x != addr);
                    let dword = self.registers[rs2 as usize];
                    let _ = self.bus.write(addr, dword, 64);
                    self.registers[rd as usize] = 0;
                } else {
                    self.res.retain(|x| *x != addr);
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
                
                if let Ok(temp) = self.bus.read(&addr, 64) {
                    let temp: u64 = (temp as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let _ = self.bus.write(addr, val, 64);
                    self.registers[rd as usize] = temp;
                }
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

                if let Ok(temp) = self.bus.read(&addr, 64) {
                    let temp = (temp as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = temp + val;
                    let _ = self.bus.write(addr, res, 64);
                    self.registers[rd as usize] = temp;
                }
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

                if let Ok(temp) = self.bus.read(&addr, 64) {
                    let temp = (temp as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = temp ^ val;
                    let _ = self.bus.write(addr, res, 64);
                    self.registers[rd as usize] = temp;
                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 64) {
                    let temp = (temp as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = temp & val;
                    let _ = self.bus.write(addr, res, 64);
                    self.registers[rd as usize] = temp;
                }
                
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
                
                if let Ok(temp) = self.bus.read(&addr, 64) {
                    let temp = (temp as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let res = temp | val;
                    let _ = self.bus.write(addr, res, 64);
                    self.registers[rd as usize] = temp;                    
                }
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
                
                let res = self.bus.read(&addr, 64);
                if let Ok(temp) = res {
                    let temp = (temp as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let fin = std::cmp::min(temp, val);
                    let _ = self.bus.write(addr, fin, 64);
                    self.registers[rd as usize] = temp;
                }
            },
            Instruction::AmomaxD { rd, rs1, rs2, .. } => {
                let addr = self.registers[rs1 as usize];
                
                if addr % 4 != 0 {
                    // Reject
                    todo!();
                }
                
                let res = self.bus.read(&addr, 64);
                if let Ok(temp) = res {
                    let temp = (temp as i64) as u64;
                    let val = self.registers[rs2 as usize];
                    let fin = std::cmp::max(temp, val);
                    let _ = self.bus.write(addr, fin, 64);
                    self.registers[rd as usize] = temp;
                }
                
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
                
                let res = self.bus.read(&addr, 64);
                if let Ok(temp) = res {
                    let val = self.registers[rs2 as usize];
                    let fin = std::cmp::min(temp, val);
                    let _ = self.bus.write(addr, fin, 64);
                    self.registers[rd as usize] = temp;
                }
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
                
                if let Ok(temp) = self.bus.read(&addr, 64) {
                    let val = self.registers[rs2 as usize];
                    let fin = std::cmp::max(temp, val);
                    let _ = self.bus.write(addr, fin, 64);
                    self.registers[rd as usize] = temp;
                }
            },
            Instruction::Flw { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                if let Some(bits) = self.bus.read(&addr, 32) {
                    let val = f32::from_bits(bits);
                    self.f_registers[rd as usize] = val as f64;
                }
            },
            Instruction::Fsw { rd, rs1, rs2, imm, .. } => {
                // store value in f_register rs2 as bits into memory at address in rs1 + imm
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                let val = self.f_registers[rs2 as usize].to_bits() as u64;
                self.bus.write(addr, val, 32);
            },
            Instruction::FmaddS { rd, rs1, rs2, rs3, rm, .. } => {
                // multiply value in f_register[rs1] by value in f_register[rs2]
                // add value in rs3
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FmsubS { rd, rs1, rs2, rs3, rm, .. } => {
                // multiply value in f_register[rs1] by value in f_register[rs2]
                // subtract value in rs3
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = -self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, -rs3_val);
            },
            Instruction::FnmsubS { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = -self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = -self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FnmaddS { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = -self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FaddS { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val + rs2_val;
            },
            Instruction::FsubS { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val - rs2_val;
            },
            Instruction::FmulS { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val * rs2_val;
            },
            Instruction::FdivS { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val / rs2_val;
            },
            Instruction::FsqrtS { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.f_registers[rd as usize].sqrt());
            },
            Instruction::FsgnjS { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val.copysign(rs2_val);
            },
            Instruction::FsgnjnS { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = -self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val.copysign(rs2_val);
            },
            Instruction::FsgnjxS { rd, rs1, rs2, .. } => {
                let sign_1 = (self.f_registers[rs1 as usize] as f32).to_bits() & 0x8000_0000;
                let sign_2 = (self.f_registers[rs2 as usize] as f32).to_bits() & 0x8000_0000;
                let other = (self.f_registers[rs1 as usize] as f32).to_bits() & 0x7fff_ffff;
                self.f_registers[rd as usize] = (f32::from_bits((sign_1 ^ sign_2) | other)) as f64;
            },
            Instruction::FminS { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val.min(rs2_val);
            },
            Instruction::FmaxS { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val.max(rs2_val);
            },
            Instruction::FcvtWS { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].round() as i32) as u64;
            },
            Instruction::FcvtWUS { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = ((self.f_registers[rs1 as usize].round() as u32) as i32) as u64;
            },
            Instruction::FmvXW { rd, rs1, .. } => {
                let rs1_val = (((self.f_registers[rs1 as usize].to_bits() & 0xffffffff) as i32) as i64) as u64;
                self.registers[rd as usize] = rs1_val;
            },
            Instruction::FeqS { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.registers[rd as usize] = if rs1_val == rs2_val { 1 } else { 0 };
            },
            Instruction::FltS { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.registers[rd as usize] = if rs1_val < rs2_val { 1 } else { 0 };
            },
            Instruction::FleS { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.registers[rd as usize] = if rs1_val <= rs2_val { 1 } else { 0 };
            },
            Instruction::FclassS { rd, rs1, .. } => {
                todo!();
            },
            Instruction::FcvtSW { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = ((self.registers[rs1 as usize] as i32) as f32) as f64;
            },
            Instruction::FcvtSWU { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = ((self.registers[rs1 as usize] as u32) as f32) as f64;
            },
            Instruction::FmvWX { rd, rs1, .. } => {
                let rs1_val = self.registers[rs1 as usize];
                self.f_registers[rd as usize] = f64::from_bits(self.registers[rs1 as usize] & 0xffff_ffff);
            },
            Instruction::FcvtLS { rd, rs1, rm, ..} => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize] as f32).round() as u64;
            },
            Instruction::FcvtLUS { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize] as f32).round() as u64;
            },
            Instruction::FcvtSL { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.registers[rs1 as usize] as f32) as f64;
            },
            Instruction::FcvtSLU { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = ((self.registers[rs1 as usize] as u64) as f32) as f64;
            },
            Instruction::Fld { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize];
                let val = f64::from_bits(self.bus.read(&addr, 64));
                self.f_registers[rd as usize] = val;
            },
            Instruction::Fsd { rs1, rs2, imm, .. } => {
                let addr = self.registers[rs1 as usize];
                let val = self.f_registers[rs2 as usize];
                self.bus.write(addr, val.to_bits() as u64, 64);
            },
            Instruction::FmaddD { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FmsubD { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = -self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FnmsubD { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = -self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = -self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FnmaddD { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = -self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FaddD { rd, rs1, rs2, rm, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize] + self.f_registers[rs2 as usize]; 
            },
            Instruction::FsubD { rd, rs1, rs2, rm, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize] - self.f_registers[rs2 as usize];
            },
            Instruction::FmulD { rd, rs1, rs2, rm, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize] * self.f_registers[rs2 as usize];
            },
            Instruction::FdivD { rd, rs1, rs2, rm, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize] / self.f_registers[rs2 as usize];
            },
            Instruction::FsqrtD { rd, rs1, rs2, rm, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize].sqrt(self.f_registers[rs2 as usize]);
            },
            Instruction::FsgnjD { rd, rs1, rs2, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize].copysign(self.f_registers[rs2 as usize]);
            },
            Instruction::FsgnjnD { rd, rs1, rs2, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize].copysign(-self.f_registers[rs2 as usize]);
            },
            Instruction::FsgnjxD { rd, rs1, rs2, .. } => {
                let sign_1 = self.f_registers[rs1 as usize].to_bits() & 0x8000_0000_0000_0000;
                let sign_2 = self.f_registers[rs1 as usize].to_bits() & 0x8000_0000_0000_0000;
                let other = self.f_registers[rs1 as usize].to_bits() & 0x7fff_ffff_ffff_ffff;
                self.f_registers[rd as usize] = f64::from_bits((sign_1 ^ sign_2) | other);
            },
            Instruction::FminD { rd, rs1, rs2, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize].min(self.f_registers[rs2 as usize]);
            },
            Instruction::FmaxD { rd, rs1, rs2, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize].max(self.f_registers[rs2 as usize]);
            },
            Instruction::FcvtSD { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize];
            },
            Instruction::FcvtDS { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.f_registers[rs1 as usize] as f32) as f64;
            },
            Instruction::FeqD { rd, rs1, rs2 .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rs1 as usize] = if  rs1_val == rs2_val { 1 } else { 0 };
            },
            Instruction::FltD { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rs1 as usize] = if  rs1_val < rs2_val { 1 } else { 0 };
            },
            Instruction::FleD { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rs1 as usize] = if  rs1_val <= rs2_val { 1 } else { 0 };
            },
            Instruction::FclassD { rd, rs1, rm, ..} => {},
            Instruction::FcvtWD { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].round() as i32) as u64;
            },
            Instruction::FcvtWUD { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = ((self.f_registers[rs1 as usize].round() as u32) as i32) as u64;
            },
            Instruction::FcvtDW { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.registers[rs1 as usize] as i32) as f64;
            },
            Instruction::FcvtDWU { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.registers[rs1 as usize] as u32) as f64;
            },
            Instruction::FcvtLD { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].round()) as u64;
            },
            Instruction::FcvtLUD { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].round()) as u64;
            },
            Instruction::FmvXD { rd, rs1, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].to_bits());
            },
            Instruction::FcvtDL { rd, rs1, .. } => {
                self.f_registers[rd as usize] = self.registers[rs1 as usize] as f64;
            },
            Instruction::FcvtDLU { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = self.registers[rs1 as usize] as f64;
            },
            Instruction::FmvDX { rd, rs1, .. } => {
                self.registers[rd as usize] = self.f_registers[rs1 as usize].to_bits();
            },
            Instruction::Flq { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize];
                let val = f64::from_bits(self.bus.read(&addr, 64));
                self.f_registers[rd as usize] = val;
            },
            Instruction::Fsq { rd, rs1, rs2, imm, .. } => {
                let addr = self.registers[rs1 as usize];
                let val = self.f_registers[rs2 as usize].to_bits() as u64;
                self.bus.write(addr, val, 64);
            },
            Instruction::FmaddQ { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FmsubQ { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = -self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FnmsubQ { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = -self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = -self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1.mul_add(rs2_val, rs3_val);
            },
            Instruction::FnmaddQ { rd, rs1, rs2, rs3, rm, .. } => {
                let rs1_val = -self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                let rs3_val = self.f_registers[rs3 as usize];
                self.f_registers[rd as usize] = rs1_val.mul_add(rs2_val, rs3_val);
            },
            Instruction::FaddQ { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val + rs2_val;
            },
            Instruction::FsubQ { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val - rs2_val;
            },
            Instruction::FmulQ { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val * rs2_val;
            },
            Instruction::FdivQ { rd, rs1, rs2, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val / rs2_val;
            },
            Instruction::FsqrtQ { rd, rs1, rm, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val.sqrt(rs2_val);
            },
            Instruction::FsgnjQ { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val.copysign(rs2_val);
            },
            Instruction::FsgnjnQ { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = -self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = rs1_val.copysign(rs2_val);
            },
            Instruction::FsgnjxQ { rd, rs1, rs2, .. } => {
                let sign_1 = self.f_registers[rs1 as usize].to_bits() & 0x8000_0000_0000_0000;
                let sign_2 = self.f_registers[rs2 as usize].to_bits() & 0x8000_0000_0000_0000;
                let other = self.f_registers[rs1 as usize].to_bits() & 0x7fff_ffff_ffff_ffff;
                self.f_registers[rd as usize] = f64::from_bits((sign_1 ^ sign_2) | other);
            },
            Instruction::FminQ { rd, rs1, rs2, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize].min(self.f_registers[rs2 as usize]);
            },
            Instruction::FmaxQ { rd, rs1, rs2, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize].max(self.f_registers[rs2 as usize]);
            },
            Instruction::FcvtSQ { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = self.f_registers[rs1 as usize];
            },
            Instruction::FcvtQS { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.f_registers[rs1 as usize]);
            },
            Instruction::FcvtDQ { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.f_registers[rs1 as usize] as f32) as f64;
            },
            Instruction::FcvtQD { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.f_registers[rs1 as usize] as f32) as f64;
            },
            Instruction::FeqQ { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = if rs1_val == rs2_val { 1 } else { 0 };
            },
            Instruction::FltQ { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = if rs1_val < rs2_val { 1 } else { 0 };
            },
            Instruction::FleQ { rd, rs1, rs2, .. } => {
                let rs1_val = self.f_registers[rs1 as usize];
                let rs2_val = self.f_registers[rs2 as usize];
                self.f_registers[rd as usize] = if rs1_val <= rs2_val { 1 } else { 0 };
            },
            Instruction::FclassQ { rd, rs1, .. } => {
                todo!();
            },
            Instruction::FcvtWQ { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].round() as i32) as u64;
            },
            Instruction::FcvtWUQ { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = ((self.f_registers[rs1 as usize].round() as u32) as i32) as u64;
            },
            Instruction::FcvtQW { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.registers[rs1 as usize] as i32) as f64;
            },
            Instruction::FcvtQWU { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = (self.registers[rs1 as usize] as u32) as f64;
            },
            Instruction::FcvtLQ { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].round()) as u64;
            },
            Instruction::FcvtLUQ { rd, rs1, rm, .. } => {
                self.registers[rd as usize] = (self.f_registers[rs1 as usize].round()) as u64;
            },
            Instruction::FcvtQL { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = self.registers[rs1 as usize] as f64;
            },
            Instruction::FcvtQLU { rd, rs1, rm, .. } => {
                self.f_registers[rd as usize] = self.registers[rs1 as usize] as f64;
            },
            _ => { /* Return an error here, and some other places */ }
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
