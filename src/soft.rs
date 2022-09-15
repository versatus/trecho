#![allow(unused, unused_mut, dead_code)]
use crate::encoding::{EncodingTable, InstructionDecoder};
use crate::encoding_types::Inst;
use crate::extensions::{Base, Extension};
use crate::instructions::Instruction;
use crate::register::Register;
use crate::memory::{Dram, MEM_SIZE};
use crate::machine::{Machine, Support};


pub struct SoftThread {
    pub registers: [u64; 33],
    pc: u64,
    pub program: Vec<u8>,
    remainder: u32,
    eq_flag: bool,
    enc_table: EncodingTable,
    bus: Dram
}

impl SoftThread {
    pub fn new(enc_table: EncodingTable) -> SoftThread {
        let mut soft = SoftThread {
            registers: [0; 33],
            pc: 0,
            program: vec![],
            remainder: 0,
            eq_flag: false,
            enc_table,
            bus: Dram::new()
        };

        soft.registers[2] = MEM_SIZE;
        soft.registers[0] = 0;

        soft
    }

    pub(crate) fn read_reg(&self, idx: u64) -> u64 {
        self.registers[idx as usize]
    }

    pub(crate) fn fetch(&self) -> Inst {
        let idx: usize = self.pc as usize;
        let mut bytes: [u8; 4] = [
            self.program[idx + 3],
            self.program[idx + 2],
            self.program[idx + 1],
            self.program[idx],
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
                if let Ok(val) = self.bus.read(addr, 8) {
                    self.registers[rd as usize] = ((val as i8) as i64) as u64;
                }
            },
            Instruction::Lh { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(addr, 16) {
                    self.registers[rd as usize] = ((val as i16) as i64) as u64;
                }
            },
            Instruction::Lw { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(addr, 32) {
                    self.registers[rd as usize] = ((val as i32) as i64) as u64
                }
            },
            Instruction::Lbu { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(addr, 8) {
                    self.registers[rd as usize] = val;
                }
            },
            Instruction::Lhu { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(addr, 16) {
                    self.registers[rd as usize] = val;
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
                self.registers[rd as usize] = self.registers[rs1 as usize] << shamt;
            },
            Instruction::Srli { rd, rs1, shamt, .. } => {
                self.registers[rd as usize] = self.registers[rs1 as usize].wrapping_shr(shamt);
            },
            Instruction::Srai { rd, rs1, shamt, .. } => {
                self.registers[rd as usize] = (self.registers[rs1 as usize] as i64).wrapping_shr(shamt) as u64;
            },
            Instruction::Add { rd, rs1, rs2, .. } => {
                if let Some(res) =
                    self.registers[rs1 as usize].checked_add(self.registers[rs2 as usize])
                {
                    self.registers[rd as usize] = res;
                }
            },
            Instruction::Sub { rd, rs1, rs2, .. } => {
                if let Some(res) =
                    self.registers[rs1 as usize].checked_sub(self.registers[rs2 as usize])
                {
                    self.registers[rd as usize] = res;
                }
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
                if let Ok(val) = self.bus.read(addr, 32) {
                    self.registers[rd as usize] = val;
                }
            },
            Instruction::Ld { rd, rs1, imm, .. } => {
                let addr = self.registers[rs1 as usize].wrapping_add((imm as i64) as u64);
                if let Ok(val) = self.bus.read(addr, 64) {
                    self.registers[rd as usize] = val;
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

impl Default for SoftThread {
    fn default() -> SoftThread {
        let enc_table = EncodingTable::default();
        SoftThread::new(enc_table)
    }
}
