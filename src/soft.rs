#![allow(unused, unused_mut, dead_code)]
use crate::register::Register;
use crate::instructions::Instruction;
use crate::encoding::{EncodingTable, InstructionDecoder};
use crate::extensions::{Base, Extension};
use crate::encoding_types::Inst;

pub struct SoftThread {
    pub registers: [u64; 33],
    pc: u64,
    pub program: Vec<u8>,
    remainder: u32,
    eq_flag: bool,
    enc_table: EncodingTable,
}

impl SoftThread {
    pub const MEM_SIZE: u64 = 1024 * 1024 * 128;
    pub fn new(enc_table: EncodingTable) -> SoftThread {
        let mut soft = SoftThread {
            registers: [0; 33],
            pc: 0,
            program: vec![],
            remainder: 0,
            eq_flag: false,
            enc_table,
        };

        soft.registers[2] = SoftThread::MEM_SIZE;
        soft.registers[0] = 0;

        soft
    }

    pub(crate) fn fetch(&self) -> Inst {
        let idx: usize = self.pc as usize;
        let mut bytes: [u8; 4] = [self.program[idx + 3], self.program[idx + 2], self.program[idx + 1], self.program[idx]];
        let inst: Inst = u32::from_le_bytes(bytes);
        return inst
    }

    pub(crate) fn execute(&mut self) {
        let instruction: Instruction = Instruction::decode(self.fetch(), &self.enc_table);
        match instruction {
            Instruction::Lui { .. } => { unimplemented!() },
            Instruction::Auipc { .. } => { unimplemented!() },
            Instruction::Jal { .. } => { unimplemented!() },
            Instruction::Jalr { .. } => { unimplemented!() },
            Instruction::Beq { .. } => { unimplemented!() },
            Instruction::Bne { .. } => { unimplemented!() },
            Instruction::Bltu { .. } => { unimplemented!() },
            Instruction::Bgeu { .. } => { unimplemented!() },
            Instruction::Lb { .. } => { unimplemented!() },
            Instruction::Lh { .. } => { unimplemented!() },
            Instruction::Lw { .. } => { unimplemented!() },
            Instruction::Lbu { .. } => { unimplemented!() },
            Instruction::Lhu { .. } => { unimplemented!() },
            Instruction::Sb { .. } => { unimplemented!() },
            Instruction::Sh { .. } => { unimplemented!() },
            Instruction::Sw { .. } => { unimplemented!() },
            Instruction::Addi { rd, rs1, imm, .. } => {
                let imm = (imm as i64) as u64;
                if let Some(res) = self.registers[rs1 as usize].checked_add(imm) {
                    self.registers[rd as usize] = res;
                }
            },
            Instruction::Slti { .. } => { unimplemented!() },
            Instruction::Sltiu { .. } => { unimplemented!() },
            Instruction::Xori { .. } => { unimplemented!() },
            Instruction::Ori { .. } => { unimplemented!() },
            Instruction::Andi { .. } => { unimplemented!() },
            Instruction::Slli { .. } => { unimplemented!() },
            Instruction::Srli { .. } => { unimplemented!() },
            Instruction::Srai { .. } => { unimplemented!() },    
            Instruction::Add { rd, rs1, rs2, ..} => {
                if let Some(res) = self.registers[rs1 as usize].checked_add(self.registers[rs2 as usize]) {
                    self.registers[rd as usize] = res;
                }
            },
            Instruction::Sub { rd, rs1, rs2, .. } => { 
                if let Some(res) = self.registers[rs1 as usize].checked_sub(self.registers[rs2 as usize]) {
                    self.registers[rd as usize] = res;
                } 
            },
            Instruction::Sll { .. } => { unimplemented!() },
            Instruction::Slt { .. } => { unimplemented!() },
            Instruction::Sltu { .. } => { unimplemented!() },
            Instruction::Xor { .. } => { unimplemented!() },
            Instruction::Srl { .. } => { unimplemented!() },
            Instruction::Sra { .. } => { unimplemented!() },
            Instruction::Or { .. } => { unimplemented!() },
            Instruction::And { .. } => { unimplemented!() },
            Instruction::Fence { .. } => { unimplemented!() },
            Instruction::ECall => { unimplemented!() },
            Instruction::EBreak => { unimplemented!() },
            Instruction::Lwu { .. } => { unimplemented!() },
            Instruction::Ld { .. } => { unimplemented!() },
            Instruction::Sd { .. } => { unimplemented!() },
            Instruction::Addiw { .. } => { unimplemented!() },
            Instruction::Slliw { .. } => { unimplemented!() },
            Instruction::Sraiw { .. } => { unimplemented!() },
            Instruction::Addw { .. } => { unimplemented!() },
            Instruction::Subw { .. } => { unimplemented!() },
            Instruction::Sllw { .. } => { unimplemented!() },
            Instruction::Srlw { .. } => { unimplemented!() },
            Instruction::Sraw { .. } => { unimplemented!() },
            Instruction::FenceI { .. } => { unimplemented!() }
            _ => { unimplemented!() }
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