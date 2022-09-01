#![allow(unused, unused_mut, dead_code)]
use crate::extensions::{Ext, I64, I32};
use std::collections::HashMap;
use crate::register::Register;
use crate::encoding::{OpCodeType, EncodingTable, InstructionDecoder, Unpacked};
use crate::encoding_types::{OpCode, Inst};

pub const SEVEN_BIT_MASK: u32 = 0b1111111 as u32;

// Enum with all instruction variants. Need all instructions for all extensions
// allow encoding table, based on extension to determine whether or not the
// instruction is ever used. i.e. Some I64 instructions are not included in
// I32 instructions, however, we do not want a separate Instruction enum for each
// When we implement decoding, based on the Extension set of the type of machine
// We will know which OpCodes are Invalid, because they will return an Invalid
// variant of the OpCodeType.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    // Invalid instruction, undefined
    Undefined,
    // Load upper immediate
    Lui { rd: Register, imm: i32 },
    // add upper immediate to program counter
    Auipc { rd: Register, imm: i32 },
    // jump and link
    Jal { rd: Register, imm: i32 },
    // jump and link register
    Jalr { rd: Register, rs1: Register, imm: i32 },
    // equal
    Beq { rd: Register, rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // not equal
    Bne { rd: Register, rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // less than
    Blt { rd: Register, rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // greater or equal
    Bge { rd: Register, rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // less than unsigned
    Bltu { rd: Register, rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // greater than equal unsigned
    Bgeu { rd: Register, rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // load bit
    Lb { rd: Register, rs1: Register, imm: i32, func3: u32 },
    // load halfword
    Lh { rd: Register, rs1: Register, imm: i32, func3: u32 },
    // load word
    Lw { rd: Register, rs1: Register, imm: i32, func3: u32 },
    // load bit unsigned
    Lbu { rd: Register, rs1: Register, imm: i32, func3: u32 },
    // load halfword unsigned
    Lhu { rd: Register, rs1: Register, imm: i32, func3: u32 },
    // save bit
    Sb { rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // save halfword
    Sh { rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // save word
    Sw { rs1: Register, rs2: Register, imm: i32, func3: u32 },
    // add integer
    Addi { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Slti { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Sltiu { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Xori { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Ori { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Andi { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Slli { rd: Register, rs1: Register, shamt: u32, func3: u32, func7: u32 },
    Srli { rd: Register, rs1: Register, shamt: u32, func3: u32, func7: u32 },
    Srai { rd: Register, rs1: Register, shamt: u32, func3: u32, func7: u32 },
    Add { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Sub { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Sll { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Slt { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Sltu { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Xor { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Srl { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Sra { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Or { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    And { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Fence { rd: Register, rs1: Register, fm: u32, pred: u32, succ: u32, func3: u32 },
    ECall,
    EBreak,
    Lwu { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Ld { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Sd { rs1: Register, rs2: Register, imm: i32, func3: u32 },
    Addiw { rd: Register, rs1: Register, imm: i32, func3: u32 },
    Slliw { rd: Register, rs1: Register, shamt: u32, func3: u32, func7: u32 },
    Srliw { rd: Register, rs1: Register, shamt: u32, func3: u32, func7: u32 },
    Sraiw { rd: Register, rs1: Register, shamt: u32, func3: u32, func7: u32 },
    Addw { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Subw { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Sllw { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Srlw { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    Sraw { rd: Register, rs1: Register, rs2: Register, func3: u32, func7: u32 },
    FenceI { rd: Register, rs1: Register, imm: i32 },
    Csrrw { rd: Register, rs1: Register, csr: i32 },
    Csrrs { rd: Register, rs1: Register, csr: i32 },
    Csrrc { rd: Register, rs1: Register, csr: i32 },
    Csrrwi { rd: Register, rs1: Register, csr: i32 },
    Csrrsi { rd: Register, rs1: Register, csr: i32 },
    Csrrci { rd: Register, rs1: Register, csr: i32 },
    Mul { rd: Register, rs1: Register, rs2: Register },
    Mulh { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Div { rd: Register, rs1: Register, rs2: Register },
    Divu { rd: Register, rs1: Register, rs2: Register },
    Rem { rd: Register, rs1: Register, rs2: Register },
    Remu { rd: Register, rs1: Register, rs2: Register },
    Mulw { rd: Register, rs1: Register, rs2: Register },
    Divw { rd: Register, rs1: Register, rs2: Register },
    Divuw { rd: Register, rs1: Register, rs2: Register },
    Remw { rd: Register, rs1: Register, rs2: Register },
    RemuW { rd: Register, rs1: Register, rs2: Register },
    LrW { rd: Register, rs1: Register, aq: i32, rl: i32 },
    ScW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoswapW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoaddW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoxorW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoandW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoorW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmominW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    Amomax { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmominuW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmomaxuW { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    LrD { rd: Register, rs1: Register, aq: i32, rl: i32 },
    ScD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoswapD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoaddD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoxorD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoandD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmoorD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmominD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmomaxD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmominuD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    AmomaxuD { rd: Register, rs1: Register, rs2: Register, aq: i32, rl: i32 },
    Flw { rd: Register, rs1: Register, imm: i32 },
    Fsw { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    FmaddS { rd: Register, rs1: Register, rs2: Register, rs3: Register, rm: u32 },
    FmsubS { rd: Register, rs1: Register, rs2: Register, rs3: Register, rm: u32 },
    FnmsubS { rd: Register, rs1: Register, rs2: Register, rs3: Register, rm: u32 },
    FnmaddS { rd: Register, rs1: Register, rs2: Register, rs3: Register, rm: u32 },
    FaddS { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsubS { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FmulS { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FdivS { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsqrtS { rd: Register, rs1: Register, rm: u32 },
    FsgnjS { rd: Register, rs1: Register, rs2: Register },
    FsgnjnS { rd: Register, rs1: Register, rs2: Register },
    FsgnjxS { rd: Register, rs1: Register, rs2: Register },
    FminS { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FmaxS { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FcvtWS { rd: Register, rs1: Register, rm: u32 },
    FctvWUS { rd: Register, rs1: Register, rm: u32 },
    FmvXW { rd: Register, rs1: Register, rm: u32 },
    FeqS { rd: Register, rs1: Register, rs2: Register },
    FltS { rd: Register, rs1: Register, rs2: Register },
    FleS { rd: Register, rs1: Register, rs2: Register },
    FclassS { rd: Register, rs1: Register },
    FcvtSW { rd: Register, rs1: Register },
    FcvtSWU { rd: Register, rs1: Register },
    FmvWX { rd: Register, rs1: Register },
    FcvtLS { rd: Register, rs1: Register, rm: u32 },
    FcvtLUS { rd: Register, rs1: Register, rm: u32 },
    FcvtSL { rd: Register, rs1: Register, rm: u32 },
    FcvtSLU { rd: Register, rs1: Register, rm: u32 },
    Fld { rd: Register, rs1: Register, imm: i32 },
    Fsd { rs1: Register, rs2: Register, imm: i32 },
    FmaddD {rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FmsubD {rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FnmsubD {rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FnmaddD {rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FaddD { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsubD { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FdivD { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsqrtD { rd: Register, rs1: Register, rm: u32 },
    FsgnjD { rd: Register, rs1: Register, rs2: Register },
    FsgnjnD { rd: Register, rs1: Register, rs2: Register },
    FsgnjxD { rd: Register, rs1: Register, rs2: Register },
    FminD { rd: Register, rs1: Register, rs2: Register },
    FmaxD { rd: Register, rs1: Register, rs2: Register },
    FcvtSD { rd: Register, rs1: Register, rm: u32 },
    FcvtDS { rd: Register, rs1: Register, rm: u32 },
    FeqD { rd: Register, rs1: Register, rs2: Register },
    FltD { rd: Register, rs1: Register, rs2: Register },
    FleD { rd: Register, rs1: Register, rs2: Register },
    FclassD { rd: Register, rs1: Register },
    FcvtWD { rd: Register, rs1: Register, rm: u32 },
    FcvtWUD { rd: Register, rs1: Register, rm: u32 },
    FcvtDW { rd: Register, rs1: Register, rm: u32 },
    FcvtDWU { rd: Register, rs1: Register, rm: u32 },
    FcvtLD { rd: Register, rs1: Register, rm: u32 },
    FcvtLUD { rd: Register, rs1: Register, rm: u32 },
    FmvXD { rd: Register, rs1: Register },
    FcvtDL { rd: Register, rs1: Register, rm: u32 },
    FcvtDLU { rd: Register, rs1: Register, rm: u32 },
    FmvDX { rd: Register, rs1: Register },
    Flq { rd: Register, rs1: Register, imm: i32 },
    Fsq { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    FmaddQ { rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FmsubQ { rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FnmsubQ { rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FnmaddQ { rd: Register, rs1: Register, rs2: Register, rs3: Register },
    FaddQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsubQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FmulQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FdivQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsqrtQ { rd: Register, rs1: Register, rm: u32 },
    FsgnjQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsgnjnQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FsgnjxQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FminQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FmaxQ { rd: Register, rs1: Register, rs2: Register, rm: u32 },
    FcvtSQ { rd: Register, rs1: Register, rm: u32 },
    FcvtQS { rd: Register, rs1: Register, rm: u32 },
    FcvtDQ { rd: Register, rs1: Register, rm: u32 },
    FcvtQD { rd: Register, rs1: Register, rm: u32 },
    FeqQ { rd: Register, rs1: Register, rs2: Register },
    FltQ { rd: Register, rs1: Register, rs2: Register },
    FleQ { rd: Register, rs1: Register, rs2: Register },
    FclassQ { rd: Register, rs1: Register },
    FcvtWQ { rd: Register, rs1: Register, rm: u32 },
    FcvtWUQ { rd: Register, rs1: Register, rm: u32 },
    FcvtQW { rd: Register, rs1: Register, rm: u32 },
    FcvtQWU { rd: Register, rs1: Register, rm: u32 },
    FcvtLQ { rd: Register, rs1: Register, rm: u32 },
    FcvtLUQ { rd: Register, rs1: Register, rm: u32 },
    FcvtQL { rd: Register, rs1: Register, rm: u32 },
    FcvtQLU { rd: Register, rs1: Register, rm: u32 },
}

impl From<Inst> for Instruction {
    fn from(inst: Inst) -> Instruction {
        let unpacked: Unpacked = Instruction::unpack(inst);
        match unpacked.opcode {
            0b0110111 => {
                return Instruction::Lui { rd: unpacked.rd.unwrap().into(), imm: unpacked.imm.unwrap() }
            },
            0b0010111 => {
                return Instruction::Auipc { rd: unpacked.rd.unwrap().into(), imm: unpacked.imm.unwrap() }
            },
            0b1101111 => {
                return Instruction::Jal { rd: unpacked.rd.unwrap().into(), imm: unpacked.imm.unwrap() }
            },
            0b1100111 => {
                return Instruction::Jalr { 
                    rd: unpacked.rd.unwrap().into(), 
                    rs1: unpacked.rs1.unwrap().into(),
                    imm: unpacked.imm.unwrap(), 
                }
            },
            0b1100011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => { 
                        return Instruction::Beq {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3 
                        } 
                    },
                    0b001 => { 
                        return Instruction::Bne { 
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        } 
                    },
                    0b100 => {
                        return Instruction::Blt {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b101 => {
                        return Instruction::Bge {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b110 => {
                        return Instruction::Bltu {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b111 => {
                        return Instruction::Bgeu {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b0000011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Lb { 
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b001 => {
                        return Instruction::Lh {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b010 => {
                        return Instruction::Lw {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b100 => {
                        return Instruction::Lbu {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b101 => {
                        return Instruction::Lhu {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b110 => {
                        return Instruction::Lwu {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b011 => {
                        return Instruction::Ld {
                            rd: unpacked.rd.unwrap().into(), 
                            rs1: unpacked.rs1.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b0100011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Sb {
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b001 => {
                        return Instruction::Sh {
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b010 => {
                        return Instruction::Sw {
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    0b011 => {
                        return Instruction::Sd {
                            rs1: unpacked.rs1.unwrap().into(), 
                            rs2: unpacked.rs2.unwrap().into(), 
                            imm: unpacked.imm.unwrap(), 
                            func3: func3
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b0010011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Addi {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3
                        }
                    },
                    0b010 => {
                        return Instruction::Slti {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3
                        }
                    },
                    0b011 => {
                        return Instruction::Sltiu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3
                        }                        
                    },
                    0b100 => {
                        return Instruction::Xori {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3
                        }                        
                    },
                    0b110 => {
                        return Instruction::Ori {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3
                        }                        
                    },
                    0b111 => {
                        return Instruction::Andi {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3
                        }                        
                    },
                    0b001 => {
                        return Instruction::Slli {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            shamt: unpacked.shamt.unwrap(),
                            func3: func3,
                            func7: unpacked.func7.unwrap()
                        }
                    },
                    0b101 => {
                        let func7 = unpacked.func7.unwrap();
                        match func7 {
                            0b0000000 => {
                                return Instruction::Srli {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    shamt: unpacked.shamt.unwrap(),
                                    func3: func3,
                                    func7: func7
                                }
                            },
                            0b0100000 => {
                                return Instruction::Srai {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    shamt: unpacked.shamt.unwrap(),
                                    func3: func3,
                                    func7: func7
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b0110011 => {
                let func3 = unpacked.func3.unwrap();
                let func7 = unpacked.func7.unwrap();
                match func3 {
                    0b000 => {
                        match func7 {
                            0b0000000 => {
                                return Instruction::Add { 
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7, 
                                }
                            },
                            0b0100000 => {
                                return Instruction::Sub {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                }
                            },
                            _ => {
                                return Instruction::Undefined
                            }
                        }
                    },
                    0b001 => {
                        return Instruction::Sll {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                            
                        }
                    },
                    0b010 => {
                        return Instruction::Slt {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                            
                        }
                    },
                    0b011 => {
                        return Instruction::Sltu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                            
                        }
                    },
                    0b100 => {
                        return Instruction::Xor {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                            
                        }
                    },
                    0b101 => {
                        match func7 {
                            0b0000000 => {
                                return Instruction::Srl {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,                                    
                                }
                            },
                            0b0100000 => {
                                return Instruction::Sra {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,                                    
                                }
                            },
                            _ => {
                                return Instruction::Undefined
                            }
                        }
                    },
                    0b110 => {
                        return Instruction::Or {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                            
                        }
                    },
                    0b111 => {
                        return Instruction::And {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                            
                        }
                    }
                    _ => { return Instruction::Undefined }
                }
            },
            0b0001111 => {
                return Instruction::Fence {
                    rd: unpacked.rd.unwrap().into(),
                    rs1: unpacked.rs1.unwrap().into(),
                    fm: unpacked.fm.unwrap().into(),
                    pred: unpacked.pred.unwrap().into(),
                    succ: unpacked.succ.unwrap().into(),
                    func3: unpacked.func3.unwrap(),
                }
            },
            0b1110011 => {
                let func3 = unpacked.func3.unwrap();
                let imm = unpacked.imm.unwrap();
                match imm {
                    0b000000000000 => {
                        assert!(func3 == 0b000);
                        assert!(unpacked.rs1.unwrap() == 0b00000);
                        assert!(unpacked.rd.unwrap() == 0b00000);
                        return Instruction::ECall
                    },
                    0b000000000001 => {
                        assert!(func3 == 0b000);
                        assert!(unpacked.rs1.unwrap() == 0b00000);
                        assert!(unpacked.rd.unwrap() == 0b00000);
                        return Instruction::EBreak
                    },
                    _ => {
                        return Instruction::Undefined
                    }
                }
            },
            0b0011011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Addiw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            func3: func3,
                            imm: unpacked.imm.unwrap(),
                        }
                    },
                    0b001 => {
                        return Instruction::Slliw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            func3: func3,
                            func7: unpacked.func7.unwrap(),
                            shamt: unpacked.shamt.unwrap(),
                        }
                    },
                    0b101 => {
                        let func7 = unpacked.func7.unwrap();
                        match func7 {
                            0b0000000 => {
                                return Instruction::Srliw {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    func3: func3,
                                    func7: unpacked.func7.unwrap(),
                                    shamt: unpacked.shamt.unwrap(),                                    
                                }
                            },
                            0b0100000 => {
                                return Instruction::Sraiw {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    func3: func3,
                                    func7: unpacked.func7.unwrap(),
                                    shamt: unpacked.shamt.unwrap(),                                    
                                }
                            },
                            _ => {
                                return Instruction::Undefined
                            }
                        }
                    },
                    _ => {
                        return Instruction::Undefined
                    }
                }
            },
            0b0111011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        let func7 = unpacked.func7.unwrap();
                        match func7 {
                            0b0000000 => { 
                                return Instruction::Addw {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                }
                            },
                            0b0100000 => {
                                return Instruction::Subw {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b001 => {
                        return Instruction::Sllw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: unpacked.func7.unwrap(),
                        }
                    },
                    0b101 => {
                        let func7 = unpacked.func7.unwrap();
                        match func7 {
                            0b0000000 => {
                                return Instruction::Srlw {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                }
                            }
                            0b0100000 => {
                                return Instruction::Sraw {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    }
                    _ => { return Instruction::Undefined }
                }
            }
            _ => { return Instruction::Undefined }
        }
    }
}

impl InstructionDecoder for Instruction {
    type Return = Self;

    fn opcode(inst: Inst) -> OpCode {
        (inst & SEVEN_BIT_MASK) as u8
    }

    fn unpack(inst: Inst) -> Unpacked {
        inst.into()
    }

    fn decode(inst: Inst, enc_table: &EncodingTable<dyn Ext>) -> Self::Return {
        let opcode_type = enc_table.get(Instruction::opcode(inst));
        if let OpCodeType::Invalid = opcode_type {
            return Instruction::Undefined
        }

        inst.into()
    }
}