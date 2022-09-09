#![allow(unused, unused_mut, dead_code)]
use crate::encoding::{EncodingTable, InstructionDecoder, OpCodeType, Unpacked};
use crate::encoding_types::{Inst, OpCode};
use crate::extensions::{Base, Extension};
use crate::register::Register;
use std::collections::HashMap;
use strum::{IntoEnumIterator, EnumProperty};
use strum_macros;

pub const SEVEN_BIT_MASK: u32 = 0b1111111 as u32;

// Enum with all instruction variants. Need all instructions for all extensions
// allow encoding table, based on extension to determine whether or not the
// instruction is ever used. i.e. Some I64 instructions are not included in
// I32 instructions, however, we do not want a separate Instruction enum for each
// When we implement decoding, based on the Extension set of the type of machine
// We will know which OpCodes are Invalid, because they will return an Invalid
// variant of the OpCodeType.
#[derive(Clone, Copy, Debug, PartialEq, strum_macros::EnumIter, strum_macros::EnumProperty)]
pub enum Instruction {
    #[strum(props(Base = "None", Ext = "None"))]
    Undefined,
    #[strum(props(Base = "32", Ext = "I"))]
    Lui {
        rd: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Auipc {
        rd: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Jal {
        rd: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Jalr {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Beq {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Bne {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Blt {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Bge {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Bltu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Bgeu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Lb {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Lh {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Lw {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Lbu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Lhu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sb {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sh {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sw {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Addi {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Slti {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sltiu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Xori {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Ori {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Andi {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Slli {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Srli {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Srai {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Add {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sub {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sll {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Slt {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sltu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Xor {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Srl {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Sra {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Or {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    And {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Fence {
        rd: Register,
        rs1: Register,
        fm: u32,
        pred: u32,
        succ: u32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    ECall,
    #[strum(props(Base = "32", Ext = "I"))]
    EBreak,
    #[strum(props(Base = "64", Ext = "I"))]
    Lwu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Ld {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Sd {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Addiw {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Slliw {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Srliw {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Sraiw {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Addw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Subw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Sllw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Srlw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "64", Ext = "I"))]
    Sraw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    FenceI {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Csrrw {
        rd: Register,
        rs1: Register,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Csrrs {
        rd: Register,
        rs1: Register,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Csrrc {
        rd: Register,
        rs1: Register,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Csrrwi {
        rd: Register,
        uimm: u32,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Csrrsi {
        rd: Register,
        uimm: u32,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "I"))]
    Csrrci {
        rd: Register,
        uimm: u32,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Mul {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Mulh {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Mulhsu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Mulhu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Div {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Divu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Rem {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32, 
    },
    #[strum(props(Base = "32", Ext = "M"))]
    Remu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "64", Ext = "M"))]
    Mulw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "64", Ext = "M"))]
    Divw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "64", Ext = "M"))]
    Divuw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "64", Ext = "M"))]
    Remw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "64", Ext = "M"))]
    RemuW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "32", Ext = "A"))]
    LrW {
        rd: Register,
        rs1: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    ScW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmoswapW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmoaddW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmoxorW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmoandW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmoorW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmominW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmomaxW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmominuW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "A"))]
    AmomaxuW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    LrD {
        rd: Register,
        rs1: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    ScD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmoswapD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmoaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmoxorD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmoandD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmoorD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmominD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmomaxD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmominuD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "64", Ext = "A"))]
    AmomaxuD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: u8,
        rl: u8,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    Flw {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    Fsw {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FmaddS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FmsubS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FnmsubS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FnmaddS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FaddS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FsubS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FmulS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FdivS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FsqrtS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FsgnjS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FsgnjnS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FsgnjxS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FminS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FmaxS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FcvtWS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FcvtWUS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FmvXW {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FeqS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FltS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FleS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FclassS {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FcvtSW {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FcvtSWU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "F"))]
    FmvWX {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "64", Ext = "F"))]
    FcvtLS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "F"))]
    FcvtLUS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "F"))]
    FcvtSL {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "F"))]
    FcvtSLU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    Fld {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    Fsd {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FmaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FmsubD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FnmsubD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FnmaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FsubD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FmulD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FdivD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FsqrtD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FsgnjD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FsgnjnD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FsgnjxD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FminD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FmaxD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FcvtSD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FcvtDS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FeqD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FltD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FleD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FclassD {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FcvtWD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FcvtWUD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FcvtDW {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "D"))]
    FcvtDWU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "D"))]
    FcvtLD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "D"))]
    FcvtLUD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "D"))]
    FmvXD {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "64", Ext = "D"))]
    FcvtDL {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "D"))]
    FcvtDLU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "D"))]
    FmvDX {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    Flq {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    Fsq {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FmaddQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FmsubQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FnmsubQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FnmaddQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FaddQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FsubQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FmulQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FdivQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FsqrtQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FsgnjQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FsgnjnQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FsgnjxQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FminQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FmaxQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtSQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtQS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtDQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtQD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FeqQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FltQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FleQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FclassQ {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtWQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtWUQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtQW {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "32", Ext = "Q"))]
    FcvtQWU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "Q"))]
    FcvtLQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "Q"))]
    FcvtLUQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "Q"))]
    FcvtQL {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "64", Ext = "Q"))]
    FcvtQLU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
}

impl From<Inst> for Instruction {
    fn from(inst: Inst) -> Instruction {
        let unpacked: Unpacked = Instruction::unpack(inst);
        match unpacked.opcode {
            0b0110111 => {
                return Instruction::Lui {
                    rd: unpacked.rd.unwrap().into(),
                    imm: unpacked.imm.unwrap(),
                }
            }
            0b0010111 => {
                return Instruction::Auipc {
                    rd: unpacked.rd.unwrap().into(),
                    imm: unpacked.imm.unwrap(),
                }
            }
            0b1101111 => {
                return Instruction::Jal {
                    rd: unpacked.rd.unwrap().into(),
                    imm: unpacked.imm.unwrap(),
                }
            }
            0b1100111 => {
                return Instruction::Jalr {
                    rd: unpacked.rd.unwrap().into(),
                    rs1: unpacked.rs1.unwrap().into(),
                    imm: unpacked.imm.unwrap(),
                }
            }
            0b1100011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Beq {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b001 => {
                        return Instruction::Bne {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b100 => {
                        return Instruction::Blt {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b101 => {
                        return Instruction::Bge {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b110 => {
                        return Instruction::Bltu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b111 => {
                        return Instruction::Bgeu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    _ => return Instruction::Undefined,
                }
            }
            0b0000011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Lb {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b001 => {
                        return Instruction::Lh {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b010 => {
                        return Instruction::Lw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b100 => {
                        return Instruction::Lbu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b101 => {
                        return Instruction::Lhu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b110 => {
                        return Instruction::Lwu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b011 => {
                        return Instruction::Ld {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    _ => return Instruction::Undefined,
                }
            }
            0b0100011 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Sb {
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b001 => {
                        return Instruction::Sh {
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b010 => {
                        return Instruction::Sw {
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b011 => {
                        return Instruction::Sd {
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    _ => return Instruction::Undefined,
                }
            }
            0b0010011 => {
                let func3 = unpacked.func3.unwrap();
                let func7 = unpacked.func7.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Addi {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b010 => {
                        return Instruction::Slti {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b011 => {
                        return Instruction::Sltiu {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b100 => {
                        return Instruction::Xori {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b110 => {
                        return Instruction::Ori {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b111 => {
                        return Instruction::Andi {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    0b001 => {
                        if func7 != 0b0000000 {
                            return Instruction::Undefined;
                        }

                        return Instruction::Slli {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            shamt: unpacked.shamt.unwrap(),
                            func3: func3,
                            func7: unpacked.func7.unwrap(),
                        };
                    }
                    0b101 => match func7 {
                        0b0000000 => {
                            return Instruction::Srli {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                shamt: unpacked.shamt.unwrap(),
                                func3: func3,
                                func7: func7,
                            }
                        }
                        0b0100000 => {
                            return Instruction::Srai {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                shamt: unpacked.shamt.unwrap(),
                                func3: func3,
                                func7: func7,
                            }
                        }
                        _ => return Instruction::Undefined,
                    },
                    _ => return Instruction::Undefined,
                }
            }
            0b0110011 => {
                let func3 = unpacked.func3.unwrap();
                let func7 = unpacked.func7.unwrap();
                match func3 {
                    0b000 => match func7 {
                        0b0000000 => {
                            return Instruction::Add {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                rs2: unpacked.rs2.unwrap().into(),
                                func3: func3,
                                func7: func7,
                            }
                        }
                        0b0100000 => {
                            return Instruction::Sub {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                rs2: unpacked.rs2.unwrap().into(),
                                func3: func3,
                                func7: func7,
                            }
                        }
                        0b0000001 => {
                            return Instruction::Mul {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                rs2: unpacked.rs2.unwrap().into(),
                                func3: func3,
                                func7: func7
                            }
                        }
                        _ => return Instruction::Undefined,
                    },
                    0b001 => {
                        match func7 {
                            0b0000000 => {
                                return Instruction::Sll {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                }
                            },
                            0b0000001 => {
                                return Instruction::Mulh {
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
                    0b010 => {
                        match func7 {
                            0b0000000 => {
                                return Instruction::Slt {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                };
                            },
                            0b0000001 => {
                                return Instruction::Mulhsu {
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
                    0b011 => {
                        match func7 {
                            0b0000000 => {
                                return Instruction::Sltu {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                };
                            },
                            0b0000001 => {
                                return Instruction::Mulhu {
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
                    0b100 => {
                        match func7 { 
                            0b0000000 => {
                                return Instruction::Xor {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                };
                            },
                            0b0000001 => {
                                return Instruction::Div {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,                                    
                                };
                            },
                            _ => { return Instruction::Undefined }
                        }
                    }
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
                            0b0000001 => {
                                return Instruction::Divu {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,                                    
                                }
                            }
                            _ => return Instruction::Undefined,
                        }
                    },
                    0b110 => {
                        match func7 {
                            0b0000000 => {
                                return Instruction::Or {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                };
                            },
                            0b0000001 => {
                                return Instruction::Rem {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,                                    
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    }
                    0b111 => {
                        match func7 {
                            0b0000000 => {
                                return Instruction::And {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,
                                };
                            },
                            0b0000001 => {
                                return Instruction::Remu {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,                                    
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    }
                    _ => return Instruction::Undefined,
                }
            }
            0b0001111 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b000 => {
                        return Instruction::Fence {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            fm: unpacked.fm.unwrap().into(),
                            pred: unpacked.pred.unwrap().into(),
                            succ: unpacked.succ.unwrap().into(),
                            func3: func3,
                        }
                    }
                    0b001 => {
                        return Instruction::FenceI {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap(),
                            func3: func3,
                        }
                    }
                    _ => { return Instruction::Undefined }
                }
            }
            0b1110011 => {
                let func3 = unpacked.func3.unwrap();
                let imm = unpacked.imm.unwrap();
                match func3 {
                    0b000 => {
                        match imm {
                            0b000000000000 => {
                                assert!(func3 == 0b000);
                                assert!(unpacked.rs1.unwrap() == 0b00000);
                                assert!(unpacked.rd.unwrap() == 0b00000);
                                return Instruction::ECall;
                            }
                            0b000000000001 => {
                                assert!(func3 == 0b000);
                                assert!(unpacked.rs1.unwrap() == 0b00000);
                                assert!(unpacked.rd.unwrap() == 0b00000);
                                return Instruction::EBreak;
                            }
                            _ => return Instruction::Undefined,
                        }
                    },
                    0b001 => {
                        return Instruction::Csrrw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,
                        }
                    },
                    0b010 => {
                        return Instruction::Csrrs {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,                            
                        }
                    },
                    0b011 => {
                        return Instruction::Csrrc {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,                            
                        }
                    },
                    0b101 => {
                        return Instruction::Csrrwi {
                            rd: unpacked.rd.unwrap().into(),
                            uimm: unpacked.uimm.unwrap(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,
                        }
                    },
                    0b110 => {
                        return Instruction::Csrrsi {
                            rd: unpacked.rd.unwrap().into(),
                            uimm: unpacked.uimm.unwrap(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,                            
                        }
                    },
                    0b111 => {
                        return Instruction::Csrrci {
                            rd: unpacked.rd.unwrap().into(),
                            uimm: unpacked.uimm.unwrap(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,                            
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            }
            0b0011011 => {
                let func3 = unpacked.func3.unwrap();
                let func7 = unpacked.func7.unwrap();
                match func3 {
                    0b000 => {
                        if func7 != 0b0000000 {
                            return Instruction::Undefined;
                        }

                        return Instruction::Addiw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            func3: func3,
                            imm: unpacked.imm.unwrap(),
                        };
                    }
                    0b001 => {
                        if func7 != 0b0000000 {
                            return Instruction::Undefined;
                        }

                        return Instruction::Slliw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            func3: func3,
                            func7: unpacked.func7.unwrap(),
                            shamt: unpacked.shamt.unwrap(),
                        };
                    }
                    0b101 => match func7 {
                        0b0000000 => {
                            return Instruction::Srliw {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                func3: func3,
                                func7: unpacked.func7.unwrap(),
                                shamt: unpacked.shamt.unwrap(),
                            }
                        }
                        0b0100000 => {
                            return Instruction::Sraiw {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                func3: func3,
                                func7: unpacked.func7.unwrap(),
                                shamt: unpacked.shamt.unwrap(),
                            }
                        }
                        _ => return Instruction::Undefined,
                    },
                    _ => return Instruction::Undefined,
                }
            }
            0b0111011 => {
                let func3 = unpacked.func3.unwrap();
                let func7 = unpacked.func7.unwrap();
                match func3 {
                    0b000 => match func7 {
                        0b0000000 => {
                            return Instruction::Addw {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                rs2: unpacked.rs2.unwrap().into(),
                                func3: func3,
                                func7: func7,
                            }
                        }
                        0b0100000 => {
                            return Instruction::Subw {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                rs2: unpacked.rs2.unwrap().into(),
                                func3: func3,
                                func7: func7,
                            }
                        },
                        0b0000001 => {
                            return Instruction::Mulw {
                                rd: unpacked.rd.unwrap().into(),
                                rs1: unpacked.rs1.unwrap().into(),
                                rs2: unpacked.rs2.unwrap().into(),
                                func3: func3,
                                func7: func7,                                
                            }
                        }
                        _ => return Instruction::Undefined,
                    },
                    0b100 => {
                        if func7 != 0b0000001 {
                            return Instruction::Undefined
                        }

                        return Instruction::Divw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                               
                        }
                    }
                    0b001 => {
                        if func7 != 0b0000000 {
                            return Instruction::Undefined;
                        }
                        return Instruction::Sllw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: unpacked.func7.unwrap(),
                        };
                    }
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
                            },
                            0b0000001 => {
                                return Instruction::Divuw {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    func3: func3,
                                    func7: func7,                                    
                                }
                            }
                            _ => return Instruction::Undefined,
                        }
                    },
                    0b110 => {
                        if func7 != 0b0000001 {
                            return Instruction::Undefined
                        }
                        
                        return Instruction::Remw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                               
                        }
                    },
                    0b111 => {
                        if func7 != 0b0000001 {
                            return Instruction::Undefined
                        }

                        return Instruction::RemuW {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            func3: func3,
                            func7: func7,                            
                        }
                    }
                    _ => return Instruction::Undefined,
                }
            }
            0b0101111 => {
                let func3 = unpacked.func3.unwrap();
                let func5 = unpacked.rs3.unwrap();
                match func5 {
                    0b00010 => {
                        let rs2 = unpacked.rs2.unwrap();
                        if rs2 != 0b00000 {
                            return Instruction::Undefined
                        }

                        match func3 {
                            0b010 => {
                                return Instruction::LrW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),
                                }
                            },
                            0b011 => {
                                return Instruction::LrD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),
                                }
                            },
                            _ => {
                                return Instruction::Undefined
                            }
                        }
                    },
                    0b00011 => {
                        match func3 {
                            0b010 => {
                                return Instruction::ScW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),
                                }
                            },
                            0b011 => {
                                return Instruction::ScD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }

                    },
                    0b00001 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmoswapW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),                            
                                }
                            },
                            0b011 => {
                                return Instruction::AmoswapD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),                            
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b00000 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmoaddW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),                            
                                }
                            },
                            0b011 => {
                                return Instruction::AmoaddD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),                            
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }           
                    },
                    0b00100 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmoxorW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),                            
                                }
                            },
                            0b011 => {
                                return Instruction::AmoxorD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(),                            
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b01100 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmoandW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            0b011 => {
                                return Instruction::AmoandD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b01000 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmoorW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            0b011 => {
                                return Instruction::AmoorD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b10000 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmominW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            0b011 => {
                                return Instruction::AmominD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b10100 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmomaxW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            0b011 => {
                                return Instruction::AmomaxD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b11000 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmominuW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            0b011 => {
                                return Instruction::AmominuD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b11100 => {
                        match func3 {
                            0b010 => {
                                return Instruction::AmomaxuW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            }
                            0b011 => {
                                return Instruction::AmomaxuD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                    aq: unpacked.aq.unwrap(),
                                    rl: unpacked.rl.unwrap(), 
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b0000111 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b010 => {
                        return Instruction::Flw {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap()
                        }
                    },
                    0b011 => {
                        return Instruction::Fld {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap()
                        }
                    },
                    0b100 => {
                        return Instruction::Flq {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            imm: unpacked.imm.unwrap()
                        }
                    }
                    _ => { return Instruction::Undefined }
                }
            },
            0b0100111 => {
                let func3 = unpacked.func3.unwrap();
                match func3 {
                    0b010 => {
                        return Instruction::Fsw {
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap()
                        }
                    },
                    0b011 => {
                        return Instruction::Fsd {
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap()
                        }
                    },
                    0b100 => {
                        return Instruction::Fsq {
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            imm: unpacked.imm.unwrap()
                        }
                    }
                    _ => { return Instruction::Undefined}
                }
            },
            0b1000011 => {
                let func2 = unpacked.func2.unwrap();
                match func2 {
                    0b00 => {
                        return Instruction::FmaddS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b01 => {
                        return Instruction::FmaddD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b11 => {
                        return Instruction::FmaddQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b1000111 => {
                let func2 = unpacked.func2.unwrap();
                match func2 {
                    0b00 => {
                        return Instruction::FmsubS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b01 => {
                        return Instruction::FmsubD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b11 => {
                        return Instruction::FmsubQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    _ => { return Instruction::Undefined}
                }
            }
            0b1001011 => {
                let func2 = unpacked.func2.unwrap();
                match func2 {
                    0b00 => {
                        return Instruction::FnmsubS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b01 => {
                        return Instruction::FnmsubD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b11 => {
                        return Instruction::FnmsubQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b1001111 => {
                let func2 = unpacked.func2.unwrap();
                match func2 {
                    0b00 => {
                        return Instruction::FnmaddS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b01 => {
                        return Instruction::FnmaddD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    0b11 => {
                        return Instruction::FnmaddQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rs3: unpacked.rs3.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32,
                        }
                    },
                    _ => { return Instruction::Undefined }
                }
            },
            0b1010011 => {
                let func7 = unpacked.func7.unwrap();
                let func3 = unpacked.func3.unwrap();
                match func7 {
                    0b0000000 => {
                        return Instruction::FaddS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0000001 => {
                        return Instruction::FaddD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32                            
                        }
                    },
                    0b0000011 => {
                        return Instruction::FaddQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32                            
                        }
                    },
                    0b0000100 => {
                        return Instruction::FsubS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0000101 => {
                        return Instruction::FsubD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0000111 => {
                        return Instruction::FsubQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0001000 => {
                        return Instruction::FmulS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0001001 => {
                        return Instruction::FmulD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0001011 => {
                        return Instruction::FmulQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0001100 => {
                        return Instruction::FdivS {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0001101 => {
                        return Instruction::FdivD {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0001111 => {
                        return Instruction::FdivQ {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rs1.unwrap().into(),
                            rs2: unpacked.rs2.unwrap().into(),
                            rm: unpacked.rm.unwrap() as u32
                        }
                    },
                    0b0101100 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FsqrtS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0010000 => {
                        match func3 {
                            0b000 => {
                                return Instruction::FsgnjS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            0b001 => {
                                return Instruction::FsgnjnS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            0b010 => {
                                return Instruction::FsgnjxS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0010100 => {
                        match func3 {
                            0b000 => {
                                return Instruction::FminS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            0b001 => {
                                return Instruction::FmaxS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1100000 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtWS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32
                                }
                            },
                            0b00001 => {
                                return Instruction::FcvtWUS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32
                                }
                            },
                            0b00010 => {
                                return Instruction::FcvtLS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtLUS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1110000 => {
                        let func3 = unpacked.func3.unwrap();
                        let func5 = unpacked.rs2.unwrap();
                        match func3 {
                            0b000 => {
                                match func5 {
                                    0b00000 => {
                                        return Instruction::FmvXW {
                                            rd: unpacked.rd.unwrap().into(),
                                            rs1: unpacked.rs1.unwrap().into(),
                                        }
                                    },
                                    _ => { return Instruction::Undefined }
                                }
                            },
                            0b001 => {
                                match func5 {
                                    0b00000 => {
                                        return Instruction::FclassS {
                                            rd: unpacked.rd.unwrap().into(),
                                            rs1: unpacked.rs1.unwrap().into(),
                                        }
                                    },
                                    _ => { return Instruction::Undefined }
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1010000 => {
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b000 => {
                                return Instruction::FleS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()
                                }
                            },
                            0b010 => {
                                return Instruction::FeqS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()
                                }
                            },
                            0b001 => {
                                return Instruction::FltS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()                                    
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1101000 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtSW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00001 => {
                                return Instruction::FcvtSWU {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,                                    
                                }
                            },
                            0b00010 => {
                                return Instruction::FcvtSL {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtSLU {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1111000 => {
                        let func3 = unpacked.func3.unwrap();
                        let func5 = unpacked.rs2.unwrap();
                        match func3 {
                            0b000 => {
                                match func5 {
                                    0b00000 => {
                                        return Instruction::FmvWX {
                                            rd: unpacked.rd.unwrap().into(),
                                            rs1: unpacked.rs1.unwrap().into(),
                                        }
                                    },
                                    _ => { return Instruction::Undefined }
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0101101 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FsqrtD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0010001 => {
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b000 => {
                                return Instruction::FsgnjD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            0b001 => {
                                return Instruction::FsgnjnD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            0b010 => {
                                return Instruction::FsgnjxD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0010101 => {
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b000 => {
                                return Instruction::FminD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()
                                }
                            },
                            0b001 => {
                                return Instruction::FmaxD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()
                                }
                            },
                            _ => { Instruction::Undefined }
                        }
                    },
                    0b0100000 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00001 => {
                                return Instruction::FcvtSD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtSQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,                                    
                                }
                            }
                            _ => { Instruction::Undefined }
                        }
                    },
                    0b0100001 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtDS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtDQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    }
                    0b1010001 => {
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b010 => {
                                return Instruction::FeqD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()
                                }
                            },
                            0b001 => {
                                return Instruction::FltD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()
                                }
                            },
                            0b000 => {
                                return Instruction::FleD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into()
                                }
                            },
                            _ => { Instruction::Undefined }
                        }
                    },
                    0b1110001 => {
                        let func3 = unpacked.func3.unwrap();
                        let func5 = unpacked.rs2.unwrap();
                        match func3 {
                            0b001 => {
                                match func5 {
                                    0b00000 => {
                                        return Instruction::FclassD {
                                            rd: unpacked.rd.unwrap().into(),
                                            rs1: unpacked.rs1.unwrap().into()
                                        }
                                    },
                                    _ => { return Instruction::Undefined }
                                }
                            },
                            0b000 => {
                                match func5 {
                                    0b00000 => {
                                        return Instruction::FmvXD {
                                            rd: unpacked.rd.unwrap().into(),
                                            rs1: unpacked.rs1.unwrap().into()
                                        }
                                    },
                                    _ => { return Instruction::Undefined }
                                }
                            },
                            _ => { Instruction::Undefined }
                        }
                    },
                    0b1100001 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtWD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00001 => {
                                return Instruction::FcvtWUD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00010 => {
                                return Instruction::FcvtLD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtLUD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            _ => { Instruction::Undefined }
                        }
                    },
                    0b1101001 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtDW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00001 => {
                                return Instruction::FcvtDWU {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00010 => {
                                return Instruction::FcvtDL {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtDLU {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            _ => { Instruction::Undefined }
                        }
                    },
                    0b1111001 => {
                        let func5 = unpacked.rs2.unwrap();
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b000 => {
                                match func5 {
                                    0b00000 => {
                                        return Instruction::FmvDX {
                                            rd: unpacked.rd.unwrap().into(),
                                            rs1: unpacked.rs1.unwrap().into()
                                        }
                                    }
                                    _ => { return Instruction::Undefined }
                                }
                            },
                            _ => { Instruction::Undefined }
                        }
                    },
                    0b0101111 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FsqrtQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0010011 => {
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b000 => {
                                return Instruction::FsgnjQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),

                                }
                            },
                            0b001 => {
                                return Instruction::FsgnjnQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),

                                }
                            },
                            0b010 => {
                                return Instruction::FsgnjxQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),

                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0010111 => {
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b000 => {
                                return Instruction::FminQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            0b001 => {
                                return Instruction::FmaxQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b0100011 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtQS {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00001 => {
                                return Instruction::FcvtQD {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1010011 => {
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b010 => {
                                return Instruction::FeqQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            0b001 => {
                                return Instruction::FltQ{
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),                                    
                                }
                            },
                            0b000 => {
                                return Instruction::FleQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rs2: unpacked.rs2.unwrap().into(),
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1110011 => {
                        let func5 = unpacked.rs2.unwrap();
                        let func3 = unpacked.func3.unwrap();
                        match func3 {
                            0b001 => {
                                match func5 {
                                    0b00000 => {
                                        return Instruction::FclassQ {
                                            rd: unpacked.rd.unwrap().into(),
                                            rs1: unpacked.rs1.unwrap().into(),
                                        }
                                    },
                                    _ => { return Instruction::Undefined }
                                }
                            },
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1100011 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtWQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00001 => {
                                return Instruction::FcvtWUQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00010 => {
                                return Instruction::FcvtLQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtLUQ {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    },
                    0b1101011 => {
                        let func5 = unpacked.rs2.unwrap();
                        match func5 {
                            0b00000 => {
                                return Instruction::FcvtQW {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00001 => {
                                return Instruction::FcvtQWU {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00010 => {
                                return Instruction::FcvtQL {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            },
                            0b00011 => {
                                return Instruction::FcvtQLU {
                                    rd: unpacked.rd.unwrap().into(),
                                    rs1: unpacked.rs1.unwrap().into(),
                                    rm: unpacked.rm.unwrap() as u32,
                                }
                            }
                            _ => { return Instruction::Undefined }
                        }
                    },
                    
                    _ => { return Instruction::Undefined }
                }
            },

            _ => return Instruction::Undefined,
        }
    }
}

impl InstructionDecoder for Instruction {
    type Return = Self;
    type Input = u32;

    fn opcode(inst: Inst) -> OpCode {
        (inst & SEVEN_BIT_MASK) as u8
    }

    fn unpack(inst: Inst) -> Unpacked {
        inst.into()
    }

    fn decode(inst: Inst, enc_table: &EncodingTable) -> Self::Return {
        let opcode_type = enc_table.get_opcode_type(Instruction::opcode(inst));
        if let OpCodeType::Invalid = opcode_type {
            return Instruction::Undefined;
        }

        let instruction: Instruction = inst.into();
        let instruction_base: Base = instruction.get_str("Base").unwrap().into();
        let instruction_ext: Extension = instruction.get_str("Ext").unwrap().into();

        match enc_table.get_base() {
            Base::I32 => {
                match instruction_base {
                    Base::I32 => {
                        match enc_table.get_ext() {
                            Extension::I => {
                                match instruction_ext {
                                    Extension::I => return instruction,
                                    _ => return Instruction::Undefined
                                }
                            }
                            Extension::M => {
                                match instruction_ext {
                                    Extension::I => return instruction,
                                    Extension::M => return instruction,
                                    _ => return Instruction::Undefined
                                }
                            }
                            Extension::A => {
                                match instruction_ext {
                                    Extension::I => return instruction,
                                    Extension::A => return instruction,
                                    _ => return Instruction::Undefined
                                }
                            }
                            Extension::F => {
                                match instruction_ext {
                                    Extension::I => return instruction,
                                    Extension::F => return instruction,
                                    _ => return Instruction::Undefined
                                }
                            }
                            Extension::D => {
                                match instruction_ext {
                                    Extension::I => return instruction,
                                    Extension::F => return instruction,
                                    Extension::D => return instruction,
                                    _ => return Instruction::Undefined
                                }
                            }
                            Extension::G => {
                                return instruction
                            }
                        }
                    }
                    Base::I64 => {
                        return Instruction::Undefined
                    }
                }

            }
            Base::I64 => {
                match enc_table.get_ext() {
                    Extension::I => {
                        match instruction_ext {
                            Extension::I => return instruction,
                            _ => return Instruction::Undefined
                        }
                    }
                    Extension::M => {
                        match instruction_ext {
                            Extension::I => return instruction,
                            Extension::M => return instruction,
                            _ => return Instruction::Undefined
                        }
                    }
                    Extension::A => {
                        match instruction_ext {
                            Extension::I => return instruction,
                            Extension::A => return instruction,
                            _ => return Instruction::Undefined
                        }
                    }
                    Extension::F => {
                        match instruction_ext {
                            Extension::I => return instruction,
                            Extension::F => return instruction,
                            _ => return Instruction::Undefined
                        }
                    }
                    Extension::D => {
                        match instruction_ext {
                            Extension::I => return instruction,
                            Extension::F => return instruction,
                            Extension::D => return instruction,
                            _ => return Instruction::Undefined
                        }
                    }
                    Extension::G => {
                        return instruction
                    }
                }
            }
        }
    }
}
