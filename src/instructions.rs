#![allow(unused, unused_mut, dead_code)]
use crate::extensions::{Ext, I64, I32};
use std::collections::HashMap;
use crate::register::Register;
use crate::encoding::{OpCodeType, EncodingTable};
use crate::encoding_types::{OpCode, Inst};
use crate::decoding::Decoder;

// Enum with all instruction variants. Need all instructions for all extensions
// allow encoding table, based on extension to determine whether or not the
// instruction is ever used. i.e. Some I64 instructions are not included in
// I32 instructions, however, we do not want a separate Instruction enum for each
// When we implement decoding, based on the Extension set of the type of machine
// We will know which OpCodes are Invalid, because they will return an Invalid
// variant of the OpCodeType.
#[derive(Clone, Copy, Debug)]
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
    Jalr { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // equal
    Beq { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // not equal
    Bne { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // less than
    Blt { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // greater or equal
    Bge { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // less than unsigned
    Bltu { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // greater than equal unsigned
    Bgeu { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // load bit
    Lb { rd: Register, rs1: Register, imm: i32 },
    // load halfword
    Lh { rd: Register, rs1: Register, imm: i32 },
    // load bit unsigned
    Lbu { rd: Register, rs1: Register, imm: i32 },
    // load halfword unsigned
    Lhu { rd: Register, rs1: Register, imm: i32 },
    // save bit
    Sb { rd: Register, rs1: Register, imm: i32 },
    // save halfword
    Sh { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // save word
    Sw { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // add integer
    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register, rs1: Register, imm: i32 },
    Slli { rd: Register, rs1: Register, imm: i32 },
    Srli { rd: Register, rs1: Register, shamt: u32 },
    Srai { rd: Register, rs1: Register, shamt: u32 },
    Add { rd: Register, rs1: Register, rs2: Register },
    Sub { rd: Register, rs1: Register, rs2: Register },
    Sll { rd: Register, rs1: Register, rs2: Register },
    Slt { rd: Register, rs1: Register, rs2: Register },
    Sltu { rd: Register, rs1: Register, rs2: Register },
    Xor { rd: Register, rs1: Register, rs2: Register },
    Srl { rd: Register, rs1: Register, rs2: Register },
    Sra { rd: Register, rs1: Register, rs2: Register },
    Or { rd: Register, rs1: Register, rs2: Register },
    And { rd: Register, rs1: Register, rs2: Register },
    Fence { rd: Register, rs1: Register, fm: u32, pred: u32, succ: u32 },
    Ecall,
    EBreak,
    Lwu { rd: Register, rs1: Register, imm: i32 },
    Ld { rd: Register, rs1: Register, imm: i32 },
    Sd { rs1: Register, rs2: Register, imm: i32 },
    Addiw { rd: Register, rs1: Register, imm: i32 },
    Slliw { rd: Register, rs1: Register, shamt: i32 },
    Srliw { rd: Register, rs1: Register, shamt: i32 },
    Sraiw { rd: Register, rs1: Register, shamt: i32 },
    Addw { rd: Register, rs1: Register, rs2: Register },
    Subw { rd: Register, rs1: Register, rs2: Register },
    Sllw { rd: Register, rs1: Register, rs2: Register },
    Srlw { rd: Register, rs1: Register, rs2: Register },
    Sraw { rd: Register, rs1: Register, rs2: Register },
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
        Instruction::decode(inst)
    }
}

impl Decoder for Instruction {
    fn decode_i64(inst: Inst, enc_table: EncodingTable<I64>) -> Instruction {
        Instruction::Undefined
    }

    fn decode_i32(inst: Inst, enc_table: EncodingTable<I32>) -> Instruction {
        Instruction::Undefined
    }

    fn opcode(inst: Inst) -> OpCode {
        inst & 0b1111111
    }
}