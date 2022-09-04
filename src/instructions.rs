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
    #[strum(props(Base = "I32", Ext = "I32"))]
    Lui {
        rd: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Auipc {
        rd: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Jal {
        rd: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Jalr {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Beq {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Bne {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Blt {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Bge {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Bltu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Bgeu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Lb {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Lh {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Lw {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Lbu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Lhu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sb {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sh {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sw {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Addi {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Slti {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sltiu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Xori {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Ori {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Andi {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Slli {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Srli {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Srai {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Add {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sub {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sll {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Slt {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sltu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Xor {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Srl {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Sra {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Or {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    And {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Fence {
        rd: Register,
        rs1: Register,
        fm: u32,
        pred: u32,
        succ: u32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    ECall,
    #[strum(props(Base = "I32", Ext = "I32"))]
    EBreak,
    #[strum(props(Base = "I64", Ext = "I64"))]
    Lwu {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Ld {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Sd {
        rs1: Register,
        rs2: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Addiw {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Slliw {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Srliw {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Sraiw {
        rd: Register,
        rs1: Register,
        shamt: u32,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Addw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Subw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Sllw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Srlw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I64", Ext = "I64"))]
    Sraw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    FenceI {
        rd: Register,
        rs1: Register,
        imm: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Csrrw {
        rd: Register,
        rs1: Register,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Csrrs {
        rd: Register,
        rs1: Register,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Csrrc {
        rd: Register,
        rs1: Register,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Csrrwi {
        rd: Register,
        uimm: u32,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Csrrsi {
        rd: Register,
        uimm: u32,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "I32"))]
    Csrrci {
        rd: Register,
        uimm: u32,
        csr: i32,
        func3: u32,
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Mul {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Mulh {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Mulhsu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Mulhu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Div {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Divu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,        
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Rem {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32, 
    },
    #[strum(props(Base = "I32", Ext = "M32"))]
    Remu {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "I64", Ext = "M64"))]
    Mulw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "I64", Ext = "M64"))]
    Divw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "I64", Ext = "M64"))]
    Divuw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "I64", Ext = "M64"))]
    Remw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "I64", Ext = "M64"))]
    RemuW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        func3: u32,
        func7: u32,         
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    LrW {
        rd: Register,
        rs1: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    ScW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmoswapW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmoaddW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmoxorW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmoandW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmoorW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmominW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    Amomax {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmominuW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "A32"))]
    AmomaxuW {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    LrD {
        rd: Register,
        rs1: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    ScD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmoswapD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmoaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmoxorD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmoandD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmoorD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmominD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmomaxD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmominuD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I64", Ext = "A64"))]
    AmomaxuD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        aq: i32,
        rl: i32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    Flw {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    Fsw {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FmaddS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FmsubS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FnmsubS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FnmaddS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FaddS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FsubS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FmulS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FdivS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FsqrtS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FsgnjS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FsgnjnS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FsgnjxS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FminS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FmaxS {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FcvtWS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FctvWUS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FmvXW {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FeqS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FltS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FleS {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FclassS {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FcvtSW {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FcvtSWU {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I32", Ext = "F32"))]
    FmvWX {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I64", Ext = "F64"))]
    FcvtLS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "F64"))]
    FcvtLUS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "F64"))]
    FcvtSL {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "F64"))]
    FcvtSLU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    Fld {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    Fsd {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FmaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FmsubD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FnmsubD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FnmaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FaddD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FsubD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FdivD {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FsqrtD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FsgnjD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FsgnjnD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FsgnjxD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FminD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FmaxD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FcvtSD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FcvtDS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FeqD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FltD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FleD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FclassD {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FcvtWD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FcvtWUD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FcvtDW {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "D32"))]
    FcvtDWU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "D64"))]
    FcvtLD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "D64"))]
    FcvtLUD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "D64"))]
    FmvXD {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I64", Ext = "D64"))]
    FcvtDL {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "D64"))]
    FcvtDLU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I64", Ext = "D64"))]
    FmvDX {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    Flq {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    Fsq {
        rd: Register,
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FmaddQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FmsubQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FnmsubQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FnmaddQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FaddQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FsubQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FmulQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FdivQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FsqrtQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FsgnjQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FsgnjnQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FsgnjxQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FminQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FmaxQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtSQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtQS {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtDQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtQD {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FeqQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FltQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FleQ {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FclassQ {
        rd: Register,
        rs1: Register,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtWQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtWUQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtQW {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "I32", Ext = "Q32"))]
    FcvtQWU {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "Q64", Ext = "Q64"))]
    FcvtLQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "Q64", Ext = "Q64"))]
    FcvtLUQ {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "Q64", Ext = "Q64"))]
    FcvtQL {
        rd: Register,
        rs1: Register,
        rm: u32,
    },
    #[strum(props(Base = "Q64", Ext = "Q64"))]
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
                            rs1: unpacked.rd.unwrap().into(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,
                        }
                    },
                    0b010 => {
                        return Instruction::Csrrs {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rd.unwrap().into(),
                            csr: unpacked.csr.unwrap(),
                            func3: func3,                            
                        }
                    },
                    0b011 => {
                        return Instruction::Csrrc {
                            rd: unpacked.rd.unwrap().into(),
                            rs1: unpacked.rd.unwrap().into(),
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

        let g64 = Extension::G64;
        let ext: &'static str = enc_table.get_ext().into();
        let base: &'static str = enc_table.get_base().into();
        let instruction: Instruction = inst.into();
        let instruction_base = instruction.get_str("Base").unwrap();
        let instruction_ext = instruction.get_str("Ext").unwrap();

        if ext == g64.into_str() {
            return instruction
        }

        if instruction_base != base || instruction_ext != ext {
            return Instruction::Undefined
        }

        instruction
    }
}
