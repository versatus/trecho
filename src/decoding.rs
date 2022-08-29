use crate::encoding_types::{Inst, OpCode};
use crate::extensions::{I32, I64};

pub trait Decoder {
    fn decode_i64(inst: Inst, enc_table: EncodingTable<I64>) -> Instruction;
    fn decode_i32(inst: Inst, enc_table: EncodingTable<I32>) -> Instruction;
    //TODO: decode all other instruction extensions
    fn opcode(inst: Inst) -> OpCode {
        inst & 0b1111111
    }
}