pub mod encoding;
pub mod encoding_types;
pub mod extensions;
pub mod instructions;
pub mod register;
pub mod soft;
pub mod vm;
pub mod memory;
pub mod exceptions;
pub mod machine;
pub mod consts;


#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::memory::Memory;
    use crate::encoding::{InstructionDecoder, OpCodeType, Unpacked, EncodingTable};
    use crate::extensions::{Extension, Base};
    use crate::encoding_types::*;
    use crate::instructions::Instruction;
    use crate::register::{HardWiredZero, Register, RegisterAbi, RegisterValue};
    use crate::soft::SoftThread;

    #[test]
    fn test_match_register() {
        let reg = Register::X0;

        match reg {
            Register::X0 => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_convert_valid_register_to_abi() {
        let reg = Register::X0;
        let abi: RegisterAbi = reg.into();
        assert_eq!(abi, RegisterAbi::Zero(HardWiredZero))
    }

    #[test]
    fn test_convert_valid_valid_opcode_to_opcode_type() {
        let opcode = 0b0010111 as u8;
        let opcode_type: OpCodeType = opcode.into();
        assert_eq!(opcode_type, OpCodeType::U);
    }

    #[test]
    fn test_convert_valid_invalid_opcode_to_opcode_type() {
        let opcode = 0b0010110 as u8;
        let opcode_type: OpCodeType = opcode.into();
        assert_eq!(opcode_type, OpCodeType::Invalid);
    }

    #[test]
    fn test_unpack_random_r_type_bits() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1011_0011 as u32;
        let unpacked: Unpacked = bits.into();
        assert!(unpacked.rd.is_some());
        assert!(unpacked.rs1.is_some());
        assert!(unpacked.rs2.is_some());
        assert!(unpacked.func3.is_some());
        assert!(unpacked.func7.is_some());
        assert!(unpacked.fm.is_some());
        assert!(unpacked.pred.is_some());
        assert!(unpacked.succ.is_some());
        assert_eq!(unpacked.opcode, 51);
    }
    #[test]
    fn test_unpack_random_i_type_bits() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1000_0011 as u32;
        let unpacked: Unpacked = bits.into();
        assert!(unpacked.rd.is_some());
        assert!(unpacked.rs1.is_some());
        assert!(unpacked.func3.is_some());
        assert!(unpacked.imm.is_some());
        assert_eq!(unpacked.opcode, 3);
    }

    #[test]
    fn test_unpack_random_s_type_bits() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1010_0011 as u32;
        let unpacked: Unpacked = bits.into();
        assert!(unpacked.imm.is_some());
        assert!(unpacked.rs1.is_some());
        assert!(unpacked.rs2.is_some());
        assert_eq!(unpacked.opcode, 35);
    }

    #[test]
    fn test_unpack_random_u_type_bits() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1011_0111 as u32;
        let unpacked: Unpacked = bits.into();
        assert!(unpacked.imm.is_some());
        assert!(unpacked.rd.is_some());
        assert_eq!(unpacked.opcode, 55);
    }
    #[test]
    fn test_unpack_random_b_type_bits() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1011_0111 as u32;
        let unpacked: Unpacked = bits.into();
        assert!(unpacked.imm.is_some());
        assert!(unpacked.rd.is_some());
        assert_eq!(unpacked.opcode, 55);
    }
    #[test]
    fn test_unpack_random_j_type_bits() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1110_1111 as u32;
        let unpacked: Unpacked = bits.into();
        assert!(unpacked.imm.is_some());
        assert!(unpacked.rd.is_some());
        assert_eq!(unpacked.opcode, 111);
    }

    #[test]
    fn test_convert_valid_lui_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1011_0111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Lui {
                rd: Register::X25,
                imm: -858996736
            }
        );
    }
    #[test]
    fn test_convert_valid_auipc_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1001_0111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Auipc {
                rd: Register::X25,
                imm: -858996736
            }
        );
    }

    #[test]
    fn test_convert_valid_jal_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1110_1111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Jal {
                rd: Register::X25,
                imm: -212584
            }
        );
    }

    #[test]
    fn test_convert_valid_jalr_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1000_0101_1110_0111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Jalr {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276
            }
        );
    }

    #[test]
    fn test_convert_valid_beq_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1000_0101_1110_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Beq {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -822,
                func3: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_bne_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1001_0101_1110_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Bne {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -822,
                func3: 1
            }
        );
    }

    #[test]
    fn test_convert_valid_blt_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1100_0101_1110_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Blt {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -822,
                func3: 4
            }
        );
    }

    #[test]
    fn test_convert_valid_bge_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1101_0101_1110_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Bge {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -822,
                func3: 5
            }
        );
    }

    #[test]
    fn test_convert_valid_bltu_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1110_0101_1110_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Bltu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -822,
                func3: 6
            }
        );
    }

    #[test]
    fn test_convert_valid_bgeu_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1111_0101_1110_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Bgeu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -822,
                func3: 7
            }
        );
    }

    #[test]
    fn test_convert_valid_lb_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1000_0101_1000_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Lb {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_lh_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1001_0101_1000_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Lh {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 1
            }
        );
    }

    #[test]
    fn test_convert_valid_lw_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1010_0101_1000_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Lw {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 2
            }
        );
    }

    #[test]
    fn test_convert_valid_lbu_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1100_0101_1000_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Lbu {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 4
            }
        );
    }

    #[test]
    fn test_convert_valid_lhu_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1101_0101_1000_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Lhu {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 5
            }
        )
    }

    #[test]
    fn test_convert_valid_sb_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1000_0101_1010_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sb {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: 111,
                func3: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_sh_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1001_0101_1010_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sh {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: 111,
                func3: 1
            }
        );
    }

    #[test]
    fn test_convert_valid_sw_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1010_0101_1010_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sw {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: 111,
                func3: 2
            }
        );
    }

    #[test]
    fn test_convert_valid_addi_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1000_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Addi {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_slti_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1010_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Slti {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 2
            }
        );
    }

    #[test]
    fn test_convert_valid_sltiu_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1011_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sltiu {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 3
            }
        );
    }

    #[test]
    fn test_convert_valid_xori_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1100_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Xori {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 4
            }
        );
    }

    #[test]
    fn test_convert_valid_ori_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1110_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Ori {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 6
            }
        );
    }

    #[test]
    fn test_convert_valid_andi_bits_into_instruction() {
        let bits: Inst = 0b1100_1100_1100_1010_1111_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Andi {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 7
            }
        );
    }

    #[test]
    fn test_convert_valid_slli_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1001_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Slli {
                rd: Register::X11,
                rs1: Register::X21,
                shamt: 12,
                func3: 1,
                func7: 0
            }
        );
    }
    #[test]
    fn test_convert_valid_srli_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1101_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Srli {
                rd: Register::X11,
                rs1: Register::X21,
                shamt: 12,
                func3: 5,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_srai_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_1100_1010_1101_0101_1001_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Srai {
                rd: Register::X11,
                rs1: Register::X21,
                shamt: 12,
                func3: 5,
                func7: 32
            }
        );
    }

    #[test]
    fn test_convert_valid_add_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1000_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Add {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_sub_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_1100_1010_1000_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sub {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 32
            }
        );
    }

    #[test]
    fn test_convert_valid_sll_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1001_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sll {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 1,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_slt_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1010_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Slt {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 2,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_sltu_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1011_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sltu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 3,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_xor_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1100_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Xor {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 4,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_srl_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1101_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Srl {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_sra_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_1100_1010_1101_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sra {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 32
            }
        );
    }

    #[test]
    fn test_convert_valid_or_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1110_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Or {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 6,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_and_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1111_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::And {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 7,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_fence_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1000_0101_1000_1111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Fence {
                rd: Register::X11,
                rs1: Register::X21,
                fm: 0,
                pred: 0,
                succ: 0,
                func3: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_ecall_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0000_0000_0000_0000_0111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(instruction, Instruction::ECall);
    }

    #[test]
    fn test_convert_valid_ebreak_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0001_0000_0000_0000_0111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(instruction, Instruction::EBreak);
    }

    #[test]
    fn test_convert_valid_lwu_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0000_1000_0110_0100_0000_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Lwu {
                rd: Register::X8,
                rs1: Register::X16,
                imm: 0,
                func3: 6
            }
        );
    }

    #[test]
    fn test_convert_valid_ld_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0000_1000_0011_0100_0000_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Ld {
                rd: Register::X8,
                rs1: Register::X16,
                imm: 0,
                func3: 3
            }
        );
    }

    #[test]
    fn test_convert_valid_sd_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0011_1000_0011_0100_0010_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sd {
                rs1: Register::X16,
                rs2: Register::X3,
                imm: 8,
                func3: 3
            }
        );
    }

    #[test]
    fn test_convert_valid_addiw_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0011_1000_0000_0100_0001_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Addiw {
                rd: Register::X8,
                rs1: Register::X16,
                imm: 3,
                func3: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_slliw_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0011_1000_0001_0100_0001_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Slliw {
                rd: Register::X8,
                rs1: Register::X16,
                shamt: 3,
                func3: 1,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_srliw_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0011_1000_0101_0100_0001_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Srliw {
                rd: Register::X8,
                rs1: Register::X16,
                shamt: 3,
                func3: 5,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_sraiw_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_0011_1000_0101_0100_0001_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sraiw {
                rd: Register::X8,
                rs1: Register::X16,
                shamt: 3,
                func3: 5,
                func7: 32
            }
        );
    }

    #[test]
    fn test_convert_valid_addw_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0011_1000_0000_0100_0011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Addw {
                rd: Register::X8,
                rs1: Register::X16,
                rs2: Register::X3,
                func3: 0,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_subw_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_0011_1000_0000_0100_0011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Subw {
                rd: Register::X8,
                rs1: Register::X16,
                rs2: Register::X3,
                func3: 0,
                func7: 32
            }
        );
    }

    #[test]
    fn test_convert_valid_sllw_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0011_1000_0001_0100_0011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sllw {
                rd: Register::X8,
                rs1: Register::X16,
                rs2: Register::X3,
                func3: 1,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_srlw_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0011_1000_0101_0100_0011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Srlw {
                rd: Register::X8,
                rs1: Register::X16,
                rs2: Register::X3,
                func3: 5,
                func7: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_sraw_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_0011_1000_0101_0100_0011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Sraw {
                rd: Register::X8,
                rs1: Register::X16,
                rs2: Register::X3,
                func3: 5,
                func7: 32
            }
        );
    }

    #[test]
    fn test_convert_valid_fence_i_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_1100_1010_1001_0101_1000_1111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FenceI {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 12,
                func3: 1
            }
        );
    }

    #[test]
    fn test_convert_valid_csrrw_bits_into_instruction() {
        let bits: Inst = 0b1010_1010_1100_1110_1001_0110_1111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Csrrw {
                rd: Register::X13,
                rs1: Register::X29,
                csr: 2732,
                func3: 1
            }
        );
    }

    #[test]
    fn test_convert_valid_csrrs_bits_into_instruction() {
        let bits: Inst = 0b1010_1010_1100_1010_1010_0101_1111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Csrrs {
                rd: Register::X11,
                rs1: Register::X21,
                csr: 2732,
                func3: 2
            }
        );
    }

    #[test]
    fn test_convert_valid_csrrc_bits_into_instruction() {
        let bits: Inst = 0b1010_1010_1100_1010_1011_0101_1111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Csrrc {
                rd: Register::X11,
                rs1: Register::X21,
                csr: 2732,
                func3: 3
            }
        );
    }

    #[test]
    fn test_convert_valid_csrrwi_bits_into_instruction() {
        let bits: Inst = 0b1010_1010_1100_1010_1101_0101_1111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Csrrwi {
                rd: Register::X11,
                uimm: 21,
                csr: 2732,
                func3: 5
            }
        );
    }

    #[test]
    fn test_convert_valid_csrrsi_bits_into_instruction() {
        let bits: Inst = 0b1010_1010_1100_1010_1110_0101_1111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Csrrsi {
                rd: Register::X11,
                uimm: 21,
                csr: 2732,
                func3: 6
            }
        );
    }

    #[test]
    fn test_convert_valid_csrrci_bits_into_instruction() {
        let bits: Inst = 0b1010_1010_1100_1010_1111_0101_1111_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Csrrci {
                rd: Register::X11,
                uimm: 21,
                csr: 2732,
                func3: 7
            }
        );
    }
    #[test]
    fn test_convert_valid_mul_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1000_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Mul {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_mulh_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1001_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Mulh {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 1,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_mulhsu_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1010_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Mulhsu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 2,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_mulhu_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1011_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Mulhu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 3,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_div_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1100_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Div {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 4,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_divu_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1101_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Divu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_rem_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1110_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Rem {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 6,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_remu_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1111_0101_1011_0011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Remu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 7,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_mulw_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1000_0101_1011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Mulw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_divw_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1100_0101_1011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Divw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 4,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_divuw_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1101_0101_1011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Divuw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_remw_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1110_0101_1011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Remw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 6,
                func7: 1
            }
        );
    }
    #[test]
    fn test_convert_valid_remuw_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_1100_1010_1111_0101_1011_1011 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::RemuW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 7,
                func7: 1
            }
        );
    }

    #[test]
    fn test_convert_valid_lrw_bits_into_instruction() {
        let bits: Inst = 0b0001_0100_0000_0101_0010_0110_0010_1111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::LrW {
                rd: Register::X12,
                rs1: Register::X10,
                aq: 1,
                rl: 0
            }
        );
    }

    #[test]
    fn test_convert_valid_scw_bits_into_instruction() {
        let bits: Inst = 0b0001_1010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::ScW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoswapw_bits_into_instruction() {
        let bits: Inst = 0b0000_1010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoswapW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoaddw_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoaddW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoxorw_bits_into_instruction() {
        let bits: Inst = 0b0010_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoxorW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoandw_bits_into_instruction() {
        let bits: Inst = 0b0110_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoandW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoorw_bits_into_instruction() {
        let bits: Inst = 0b0100_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoorW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amominw_bits_into_instruction() {
        let bits: Inst = 0b1000_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmominW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amomaxw_bits_into_instruction() {
        let bits: Inst = 0b1010_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmomaxW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amominuw_bits_into_instruction() {
        let bits: Inst = 0b1100_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmominuW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amomaxuw_bits_into_instruction() {
        let bits: Inst = 0b1110_0010_0000_0101_0010_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmomaxuW {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_lrd_bits_into_instruction() {
        let bits: Inst = 0b0001_0100_0000_0101_0011_0110_0010_1111 as u32;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::LrD {
                rd: Register::X12,
                rs1: Register::X10,
                aq: 1,
                rl: 0
            }
        );
    }
    #[test]
    fn test_convert_valid_scd_bits_into_instruction() {
        let bits: Inst = 0b0001_1010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::ScD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoswapd_bits_into_instruction() {
        let bits: Inst = 0b0000_1010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoswapD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoaddd_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoaddD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoxord_bits_into_instruction() {
        let bits: Inst = 0b0010_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoxorD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amoandd_bits_into_instruction() {
        let bits: Inst = 0b0110_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoandD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }
    #[test]
    fn test_convert_valid_amoord_bits_into_instruction() {
        let bits: Inst = 0b0100_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmoorD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }
    #[test]
    fn test_convert_valid_amomind_bits_into_instruction() {
        let bits: Inst = 0b1000_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmominD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amomaxd_bits_into_instruction() {
        let bits: Inst = 0b1010_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmomaxD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }
    
    #[test]
    fn test_convert_valid_amominud_bits_into_instruction() {
        let bits: Inst = 0b1100_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmominuD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_amomaxud_bits_into_instruction() {
        let bits: Inst = 0b1110_0010_0000_0101_0011_0011_0010_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::AmomaxuD {
                rd: Register::X6,
                rs1: Register::X10,
                rs2: Register::X0,
                aq: 0,
                rl: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_flw_bits_into_instruction() {
        let bits: Inst = 0b0101_1011_1000_0101_0010_0011_0000_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Flw {
                rd: Register::X6,
                rs1: Register::X10,
                imm: 1464
            }
        )
    }

    #[test]
    fn test_convert_valid_fsw_bits_into_instruction() {
        let bits: Inst = 0b0101_1011_1000_0101_0010_0011_0010_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Fsw {
                rs1: Register::X10,
                rs2: Register::X24,
                imm: 47
            }
        )
    }

    #[test]
    fn test_convert_valid_fmadds_bits_into_instruction() {
        let bits: Inst = 0b0101_0000_0110_1100_1010_1110_1100_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmaddS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fmsubs_bits_into_instruction() {
        let bits: Inst = 0b0101_0000_0110_1100_1010_1110_1100_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmsubS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fnmsubs_bits_into_instruction() {
        let bits: Inst = 0b0101_0000_0110_1100_1010_1110_1100_1011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FnmsubS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fnmadds_bits_into_instruction() {
        let bits: Inst = 0b0101_0000_0110_1100_1010_1110_1100_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FnmaddS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fadds_bits_into_instruction() {
        let bits: Inst = 0b0000_0000_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FaddS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsubs_bits_into_instruction() {
        let bits: Inst = 0b0000_1000_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsubS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fmuls_bits_into_instruction() {
        let bits: Inst = 0b0001_0000_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmulS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fdivs_bits_into_instruction() {
        let bits: Inst = 0b0001_1000_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FdivS {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsqrts_bits_into_instruction() {
        let bits: Inst = 0b0101_1000_0000_0101_1010_1000_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsqrtS {
                rd: Register::X16,
                rs1: Register::X11,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjs_bits_into_instruction() {
        let bits: Inst = 0b0010_0000_0110_0101_1000_1000_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjS {
                rd: Register::X16,
                rs1: Register::X11,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjns_bits_into_instruction() {
        let bits: Inst = 0b0010_0000_0110_0101_1001_1000_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjnS {
                rd: Register::X16,
                rs1: Register::X11,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjxs_bits_into_instruction() {
        let bits: Inst = 0b0010_0000_0110_0101_1010_1000_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjxS {
                rd: Register::X16,
                rs1: Register::X11,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fmins_bits_into_instruction() {
        let bits: Inst = 0b0010_1000_0110_0101_1000_1000_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FminS {
                rd: Register::X16,
                rs1: Register::X11,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fmaxs_bits_into_instruction() {
        let bits: Inst = 0b0010_1000_0110_0101_1001_1000_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmaxS {
                rd: Register::X16,
                rs1: Register::X11,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtws_bits_into_instruction() {
        let bits: Inst = 0b1100_0000_0000_0101_1011_0011_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtWS {
                rd: Register::X6,
                rs1: Register::X11,
                rm: 3
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtwus_bits_into_instruction() {
        let bits: Inst = 0b1100_0000_0001_0101_1011_0011_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtWUS {
                rd: Register::X6,
                rs1: Register::X11,
                rm: 3
            }
        )
    }

    #[test]
    fn test_convert_valid_fmvxw_bits_into_instruction() {
        let bits: Inst = 0b1110_0000_0000_0101_1000_0011_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmvXW {
                rd: Register::X6,
                rs1: Register::X11,
            }
        )
    }

    #[test]
    fn test_convert_valid_feqs_bits_into_instruction() {
        let bits: Inst = 0b1110_0000_0000_0101_1000_0011_0101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmvXW {
                rd: Register::X6,
                rs1: Register::X11,
            }
        )
    }

    #[test]
    fn test_convert_valid_flts_bits_into_instruction() {
        let bits: Inst = 0b1010_0000_0111_0110_1010_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FeqS {
                rd: Register::X3,
                rs1: Register::X13,
                rs2: Register::X7,
            }
        )
    }

    #[test]
    fn test_convert_valid_fclasss_bits_into_instruction() {
        let bits: Inst = 0b1110_0000_0000_0110_1001_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FclassS {
                rd: Register::X3,
                rs1: Register::X13,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtsw_bits_into_instruction() {
        let bits: Inst = 0b1101_0000_0000_0110_1001_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtSW {
                rd: Register::X3,
                rs1: Register::X13,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtswu_bits_into_instruction() {
        let bits: Inst = 0b1101_0000_0001_0110_1001_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtSWU {
                rd: Register::X3,
                rs1: Register::X13,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_convert_valid_fmvwx_bits_into_instruction() {
        let bits: Inst = 0b1111_0000_0000_0110_1000_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmvWX {
                rd: Register::X3,
                rs1: Register::X13,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtls_bits_into_instruction() {
        let bits: Inst = 0b1100_0000_0010_0110_1001_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtLS {
                rd: Register::X3,
                rs1: Register::X13,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtlus_bits_into_instruction() {
        let bits: Inst = 0b1100_0000_0011_0110_1001_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtLUS {
                rd: Register::X3,
                rs1: Register::X13,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtsl_bits_into_instruction() {
        let bits: Inst = 0b1101_0000_0010_0110_1001_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtSL {
                rd: Register::X3,
                rs1: Register::X13,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtslu_bits_into_instruction() {
        let bits: Inst = 0b1101_0000_0011_0110_1001_0001_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtSLU {
                rd: Register::X3,
                rs1: Register::X13,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_convert_valid_fld_bits_into_instruction() {
        let bits: Inst = 0b0101_1011_1000_0101_0011_0011_0000_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Fld {
                rd: Register::X6,
                rs1: Register::X10,
                imm: 1464
            }
        )
    }

    #[test]
    fn test_convert_valid_fsd_bits_into_instruction() {
        let bits: Inst = 0b0101_1011_1000_0101_0011_0011_0010_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Fsd {
                rs1: Register::X10,
                rs2: Register::X24,
                imm: 47
            }
        )
    }

    #[test]
    fn test_convert_valid_fmaddd_bits_into_instruction() {
        let bits: Inst = 0b0101_0010_0110_1100_1010_1110_1100_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmaddD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fmsubd_bits_into_instruction() {
        let bits: Inst = 0b0101_0010_0110_1100_1010_1110_1100_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmsubD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fnmsubd_bits_into_instruction() {
        let bits: Inst = 0b0101_0010_0110_1100_1010_1110_1100_1011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FnmsubD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fmnaddd_bits_into_instruction() {
        let bits: Inst = 0b0101_0010_0110_1100_1010_1110_1100_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FnmaddD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_faddd_bits_into_instruction() {
        let bits: Inst = 0b0000_0010_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FaddD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsubd_bits_into_instruction() {
        let bits: Inst = 0b0000_1010_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsubD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fmuld_bits_into_instruction() {
        let bits: Inst = 0b0001_0010_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmulD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fdivd_bits_into_instruction() {
        let bits: Inst = 0b0001_1010_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FdivD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_sqrtd_bits_into_instruction() {
        let bits: Inst = 0b0101_1010_0000_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsqrtD {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjd_bits_into_instruction() {
        let bits: Inst = 0b0010_0010_0110_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjnd_bits_into_instruction() {
        let bits: Inst = 0b0010_0010_0110_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjnD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjxd_bits_into_instruction() {
        let bits: Inst = 0b0010_0010_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjxD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fmind_bits_into_instruction() {
        let bits: Inst = 0b0010_1010_0110_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FminD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fmaxd_bits_into_instruction() {
        let bits: Inst = 0b0010_1010_0110_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmaxD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtsd_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_0001_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtSD {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtds_bits_into_instruction() {
        let bits: Inst = 0b0100_0010_0000_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtDS {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_feqd_bits_into_instruction() {
        let bits: Inst = 0b1010_0010_0011_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FeqD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X3,
            }
        )
    }

    #[test]
    fn test_convert_valid_fltd_bits_into_instruction() {
        let bits: Inst = 0b1010_0010_0011_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FltD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X3,
            }
        )
    }

    #[test]
    fn test_convert_valid_fled_bits_into_instruction() {
        let bits: Inst = 0b1010_0010_0011_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FleD {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X3,
            }
        )
    }

    #[test]
    fn test_convert_valid_fclassd_bits_into_instruction() {
        let bits: Inst = 0b1110_0010_0000_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FclassD {
                rd: Register::X29,
                rs1: Register::X25,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtwd_bits_into_instruction() {
        let bits: Inst = 0b1100_0010_0000_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtWD {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtwud_bits_into_instruction() {
        let bits: Inst = 0b1100_0010_0001_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtWUD {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtdw_bits_into_instruction() {
        let bits: Inst = 0b1101_0010_0000_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtDW {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtdwu_bits_into_instruction() {
        let bits: Inst = 0b1101_0010_0001_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtDWU {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtld_bits_into_instruction() {
        let bits: Inst = 0b1100_0010_0010_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtLD {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtlud_bits_into_instruction() {
        let bits: Inst = 0b1100_0010_0011_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtLUD {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fmvxd_bits_into_instruction() {
        let bits: Inst = 0b1110_0010_0000_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmvXD {
                rd: Register::X29,
                rs1: Register::X25,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtdl_bits_into_instruction() {
        let bits: Inst = 0b1101_0010_0010_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtDL {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtdlu_bits_into_instruction() {
        let bits: Inst = 0b1101_0010_0011_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtDLU {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 1
            }
        )
    }

    #[test]
    fn test_convert_valid_fmvdx_bits_into_instruction() {
        let bits: Inst = 0b1111_0010_0000_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmvDX {
                rd: Register::X29,
                rs1: Register::X25,
            }
        )
    }

    #[test]
    fn test_convert_valid_flq_bits_into_instruction() {
        let bits: Inst = 0b0101_1011_1000_0101_0100_0011_0000_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Flq {
                rd: Register::X6,
                rs1: Register::X10,
                imm: 1464
            }
        )
    }

    #[test]
    fn test_convert_valid_fsq_bits_into_instruction() {
        let bits: Inst = 0b0101_1011_1000_0101_0100_0011_0010_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::Fsq {
                rs1: Register::X10,
                rs2: Register::X24,
                imm: 47
            }
        )
    }

    #[test]
    fn test_convert_valid_fmaddq_bits_into_instruction() {
        let bits: Inst = 0b0101_0110_0110_1100_1100_1110_1100_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmaddQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 4
            }
        )
    }

    #[test]
    fn test_convert_valid_fmsubq_bits_into_instruction() {
        let bits: Inst = 0b0101_0110_0110_1100_1100_1110_1100_0111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmsubQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 4
            }
        )
    }
    #[test]
    fn test_convert_valid_fnmsubq_bits_into_instruction() {
        let bits: Inst = 0b0101_0110_0110_1100_1100_1110_1100_1011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FnmsubQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 4
            }
        )
    }

    #[test]
    fn test_convert_valid_fnmaddq_bits_into_instruction() {
        let bits: Inst = 0b0101_0110_0110_1100_1100_1110_1100_1111;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FnmaddQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rs3: Register::X10,
                rm: 4
            }
        )
    }

    #[test]
    fn test_convert_valid_faddq_bits_into_instruction() {
        let bits: Inst = 0b0000_0110_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FaddQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsubq_bits_into_instruction() {
        let bits: Inst = 0b0000_1110_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsubQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fmulq_bits_into_instruction() {
        let bits: Inst = 0b0001_0110_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmulQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fdivq_bits_into_instruction() {
        let bits: Inst = 0b0001_1110_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FdivQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsqrtq_bits_into_instruction() {
        let bits: Inst = 0b0101_1110_0000_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsqrtQ {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjq_bits_into_instruction() {
        let bits: Inst = 0b0010_0110_0110_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjnq_bits_into_instruction() {
        let bits: Inst = 0b0010_0110_0110_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjnQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
            }
        )
    }

    #[test]
    fn test_convert_valid_fsgnjxq_bits_into_instruction() {
        let bits: Inst = 0b0010_0110_0110_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FsgnjxQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
            }
        )
    }

    #[test]
    fn test_convert_valid_fminq_bits_into_instruction() {
        let bits: Inst = 0b0010_1110_0110_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FminQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
            }
        )
    }

    #[test]
    fn test_convert_valid_fmaxq_bits_into_instruction() {
        let bits: Inst = 0b0010_1110_0110_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FmaxQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X6,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtsq_bits_into_instruction() {
        let bits: Inst = 0b0100_0000_0011_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtSQ {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtqs_bits_into_instruction() {
        let bits: Inst = 0b0100_0110_0000_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtQS {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtdq_bits_into_instruction() {
        let bits: Inst = 0b0100_0010_0011_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtDQ {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtqd_bits_into_instruction() {
        let bits: Inst = 0b0100_0110_0001_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtQD {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_feqq_bits_into_instruction() {
        let bits: Inst = 0b1010_0110_0011_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FeqQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X3,
            }
        )
    }

    #[test]
    fn test_convert_valid_fltq_bits_into_instruction() {
        let bits: Inst = 0b1010_0110_0011_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FltQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X3,
            }
        )
    }

    #[test]
    fn test_convert_valid_fleq_bits_into_instruction() {
        let bits: Inst = 0b1010_0110_0011_1100_1000_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FleQ {
                rd: Register::X29,
                rs1: Register::X25,
                rs2: Register::X3,
            }
        )
    }

    #[test]
    fn test_convert_valid_fclassq_bits_into_instruction() {
        let bits: Inst = 0b1110_0110_0000_1100_1001_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FclassQ {
                rd: Register::X29,
                rs1: Register::X25,
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtwq_bits_into_instruction() {
        let bits: Inst = 0b1100_0110_0000_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtWQ {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtwuq_bits_into_instruction() {
        let bits: Inst = 0b1100_0110_0001_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtWUQ {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtqw_bits_into_instruction() {
        let bits: Inst = 0b1101_0110_0000_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtQW {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtqwu_bits_into_instruction() {
        let bits: Inst = 0b1101_0110_0001_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtQWU {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtlq_bits_into_instruction() {
        let bits: Inst = 0b1100_0110_0010_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtLQ {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtluq_bits_into_instruction() {
        let bits: Inst = 0b1100_0110_0011_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtLUQ {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtql_bits_into_instruction() {
        let bits: Inst = 0b1101_0110_0010_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtQL {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_convert_valid_fcvtqlu_bits_into_instruction() {
        let bits: Inst = 0b1101_0110_0011_1100_1010_1110_1101_0011;
        let instruction: Instruction = bits.into();
        assert_eq!(
            instruction,
            Instruction::FcvtQLU {
                rd: Register::X29,
                rs1: Register::X25,
                rm: 2
            }
        )
    }

    #[test]
    fn test_valid_bits_for_invalid_base() {
        let bits: Inst = 0b0000_0000_0000_1000_0110_0100_0000_0011 as u32;
        let enc_table = EncodingTable::new(Extension::I, Base::I32);
        let instruction: Instruction = Instruction::decode(bits, &enc_table);
        assert_eq!(
            instruction,
            Instruction::Undefined
        )

    }

    #[test]
    fn test_valid_bits_for_invalid_ext() {
        let bits: Inst = 0b0001_0100_0000_0101_0010_0110_0010_1111 as u32;
        let enc_table = EncodingTable::new(Extension::M, Base::I32);
        let instruction: Instruction = Instruction::decode(bits, &enc_table);
        assert_eq!(
            instruction,
            Instruction::Undefined
        )
    }

    #[test]
    fn test_invalid_bits_for_valid_base_and_ext() {
        let bits: Inst = 0b0001_0100_0001_0101_0010_0110_0010_1111 as u32;
        let enc_table = EncodingTable::new(Extension::A, Base::I32);
        let instruction: Instruction = Instruction::decode(bits, &enc_table);
        assert_eq!(
            instruction,
            Instruction::Undefined
        )
    }

    #[test]
    fn test_fetch_and_decode_add_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Add {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 0
            }
        )
    }

    #[test]
    fn test_add_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_0011 as u8];

        // Preload some values int the relevant registers
        soft.registers[Register::X12 as usize] = 24u64;
        soft.registers[Register::X21 as usize] = 32u64;

        // Load the program code
        soft.load_program(program);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            56u64
        )
    }

    #[test]
    fn test_fetch_and_decode_addi_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Addi {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 0
            }
        );
    }

    #[test]
    fn test_addi_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1001_0011 as u8];

        // Preload value into rs1
        soft.registers[Register::X21 as usize] = 1000u64;

        // Load the program code
        soft.load_program(program);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            4276u64
        )
    }

    #[test]
    fn fetch_and_decode_lui_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0011_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Lui {
                rd: Register::X10,
                imm: -859144192,
            }
        );
    }

    #[test]
    fn test_lui_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0011_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            (-859144192 as i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_auipc_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0001_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        println!("{:?}", instruction);
        assert_eq!(
            instruction,
            Instruction::Auipc {
                rd: Register::X10,
                imm: -859144192,
            }
        );
    }

    #[test]
    fn test_auipc_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0001_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        soft.execute();
        assert_eq!(
            soft.registers[Register::X10 as usize],
            (-859144192 as i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_jal_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0110_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Jal {
                rd: Register::X10,
                imm: -360112,
            }
        );
    }
    
    #[test]
    fn test_jal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0110_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        soft.execute();
        println!("{}", (-360112i64) as u64);
        assert_eq!(
            soft.registers[Register::X10 as usize],
            4
        );

        assert_eq!(
            soft.pc,
            18446744073709191504
        )
    }

    #[test]
    fn fetch_and_decode_jalr_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0110_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Jalr {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
            }
        );    
    }

    #[test]
    fn test_jalr_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0110_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        soft.pc = 0;
        soft.registers[Register::X21 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            4
        );

        assert_eq!(
            soft.pc,
            ((1000 + 3276) & !1) 
        )
    }

    #[test]
    fn fetch_and_decode_beq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Beq {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -2870,
                func3: 0
            }
        );
    }
    
    #[test]
    fn test_beq_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 100;
        soft.execute();
        
        assert_eq!(
            soft.pc,
            (-2870 as i64) as u64
        )
        
    }

    #[test]
    fn test_beq_not_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 1000;
        soft.execute();
        
        assert_eq!(
            soft.pc,
            0
        )
    }

    #[test]
    fn fetch_and_decode_bne_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Bne {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -2870,
                func3: 1
            }
        );
    }

    #[test]
    fn test_bne_not_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.pc,
            (-2870 as i64) as u64
        )
    }

    #[test]
    fn test_bne_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 100;
        soft.execute();

        assert_eq!(
            soft.pc,
            0
        )
    }

    #[test]
    fn fetch_and_decode_blt_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Blt {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -2870,
                func3: 4
            }
        );
    }

    #[test]
    fn test_blt_less_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.pc,
            (-2870 as i64) as u64
        )
    }

    #[test]
    fn test_blt_greater_or_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.registers[Register::X12 as usize] = 100;
        soft.execute();

        assert_eq!(
            soft.pc,
            0
        )
    }

    #[test]
    fn fetch_and_decode_bge_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Bge {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -2870,
                func3: 5
            }
        );
    }

    #[test]
    fn test_bge_greater_or_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.registers[Register::X20 as usize] = 100;
        soft.execute();

        assert_eq!(
            soft.pc,
            (-2870 as i64) as u64
        )
    }

    #[test]
    fn test_bge_less_than_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 10;
        soft.registers[Register::X12 as usize] = 100;
        soft.execute();

        assert_eq!(
            soft.pc,
            0
        )
    }

    #[test]
    fn fetch_and_decode_bltu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Bltu {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -2870,
                func3: 6
            }
        );
    }

    #[test]
    fn test_bltu_less_than_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 10;
        soft.registers[Register::X12 as usize] = 100;
        soft.execute();

        assert_eq!(
            soft.pc,
            (-2870 as i64) as u64
        )
    }

    #[test]
    fn test_bltu_greater_than_or_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 10;
        soft.execute();

        assert_eq!(
            soft.pc,
            0
        )
    }

    #[test]
    fn fetch_and_decode_bgeu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Bgeu {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -2870,
                func3: 7
            }
        );
    }

    #[test]
    fn test_bgeu_greater_than_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 10;
        soft.execute();

        assert_eq!(
            soft.pc,
            (-2870 as i64) as u64
        )
    }

    #[test]
    fn test_bgeu_less_than_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0110_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 10;
        soft.registers[Register::X12 as usize] = 100;
        soft.execute();

        assert_eq!(
            soft.pc,
            0
        )
    }

    #[test]
    fn fetch_and_decode_lb_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Lb {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 0,
            }
        );
    }
    
    #[test]
    fn test_lb_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.bus.write(3376, 235, 8);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            235
        )
    }

    #[test]
    fn fetch_and_decode_lh_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Lh {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 1,
            }
        );
    }
    
    #[test]
    fn test_lh_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.bus.write(3376, 3000, 16);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            3000
        )
    }

    #[test]
    fn fetch_and_decode_lw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Lw {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 2,
            }
        );
    }
    
    #[test]
    fn test_lw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.bus.write(3376, 100000, 32);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            100000
        )
    }

    #[test]
    fn fetch_and_decode_lbu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Lbu {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 4,
            }
        );
    }
    
    #[test]
    fn test_lbu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.bus.write(3376, 235, 8);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            235
        )
    }

    #[test]
    fn fetch_and_decode_lhu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Lhu {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 5,
            }
        );
    }

    #[test]
    fn test_lhu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.bus.write(3376, 3000, 16);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            3000
        )
    }

    #[test]
    fn fetch_and_decode_sb_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Sb {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: 110,
                func3: 0,
            }
        );
    }
    
    #[test]
    fn test_sb_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 235;
        soft.execute();

        assert_eq!(
            soft.bus.read(&((210 as i64) as u64), 8).unwrap(),
            235
        )
        
    }

    #[test]
    fn fetch_and_decode_sh_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Sh {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: 110,
                func3: 1,
            }
        );
    }    

    #[test]
    fn test_sh_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 3000;
        soft.execute();

        assert_eq!(
            soft.bus.read(&210, 16).unwrap(),
            3000
        )
    }

    #[test]
    fn fetch_and_decode_sw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Sw {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: 110,
                func3: 2,
            }
        );
    }

    #[test]
    fn test_sw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 100000;
        soft.execute();

        assert_eq!(
            soft.bus.read(&210, 32).unwrap(),
            100000
        )
    }

    #[test]
    fn fetch_and_decode_slti_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Slti {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 2,
            }
        );
    }
    
    #[test]
    fn test_slti_less_than_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 2;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            1
        )
    }

    #[test]
    fn test_slti_greater_or_equal_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            0
        )
    }

    #[test]
    fn fetch_and_decode_xori_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Xori {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 4,
            }
        );
    }    

    #[test]
    fn test_xori_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 0b1100_1100_1100;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            0b0000_0000_0000
        )
    }

    #[test]
    fn fetch_and_decode_ori_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Ori {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 6,
            }
        );
    }

    #[test]
    fn test_ori_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 0b0011_0011_0011;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            0b1111_1111_1111
        )
    }

    #[test]
    fn fetch_and_decode_andi_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Andi {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 7,
            }
        );
    }    

    #[test]
    fn test_andi_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 0b0011_0011_0011;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            0b0000_0000_0000
        )
    }

    #[test]
    fn fetch_and_decode_slli_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Slli {
                rd: Register::X10,
                rs1: Register::X21,
                shamt: 12,
                func3: 1,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_slli_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            1000u64.wrapping_shl(12)
        )
    }

    #[test]
    fn fetch_and_decode_srli_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Srli {
                rd: Register::X10,
                rs1: Register::X21,
                shamt: 12,
                func3: 5,
                func7: 0,
            }
        );
    }
    
    #[test]
    fn test_srli_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            1000u64.wrapping_shr(12)
        );
    }

    #[test]
    fn fetch_and_decode_srai_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Srai {
                rd: Register::X10,
                rs1: Register::X21,
                shamt: 12,
                func3: 5,
                func7: 32,
            }
        );
    }

    #[test]
    fn test_srai_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0001_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            1000i64.wrapping_shr(12) as u64
        );
    }

    #[test]
    fn fetch_and_decode_sub_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Sub {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 32,
            }
        );
    }

    #[test]
    fn test_sub_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.registers[Register::X12 as usize] = 300;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            700
        )
    }

    #[test]
    fn fetch_and_decode_sll_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Sll {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 1,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_sll_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 12;
        soft.registers[Register::X21 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            1000 << (((0b1100 & 0x3f) as u64) as u32)
        )
    }

    #[test]
    fn fetch_and_decode_slt_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Slt {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 2,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_slt_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 12;
        soft.registers[Register::X21 as usize] = 10;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            1
        )
    }

    #[test]
    fn fetch_and_decode_sltu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Sltu {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 3,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_sltu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 12;
        soft.registers[Register::X21 as usize] = 10;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            1            
        )
    }

    #[test]
    fn fetch_and_decode_xor_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Xor {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 4,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_xor_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 0b1100_1100_1100;
        soft.registers[Register::X12 as usize] = 0b0011_0011_0011;
        soft.execute();
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            0b1111_1111_1111
        )
        
    }

    #[test]
    fn fetch_and_decode_srl_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Srl {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_srl_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 12;
        soft.registers[Register::X21 as usize] = 10;
        soft.execute();
        let shamt = ((12 & 0x3f) as u64) as u32;

        assert_eq!(
            soft.registers[Register::X10 as usize],
            10u64.wrapping_shr(shamt)
        )
    }

    #[test]
    fn fetch_and_decode_sra_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Sra {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 32,
            }
        );
    }

    #[test]
    fn test_sra_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 12;
        soft.registers[Register::X21 as usize] = 10;
        soft.execute();
        let shamt = ((12 & 0x3f) as u64) as u32;

        assert_eq!(
            soft.registers[Register::X10 as usize],
            (10i64.wrapping_shr(shamt) as u64)
        )
    }

    #[test]
    fn fetch_and_decode_or_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Or {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 6,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_or_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 12;
        soft.registers[Register::X21 as usize] = 10;
        soft.execute();
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            (0b0000_1010 | 0b0000_1100)
        )
    }

    #[test]
    fn fetch_and_decode_and_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::And {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 7,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_and_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0011_0011 as u8];
        soft.load_program(program);
        
        soft.registers[Register::X12 as usize] = 12;
        soft.registers[Register::X21 as usize] = 10;
        soft.execute();
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            (0b0000_1010 & 0b0000_1100)
        )
    }

    #[test]
    fn fetch_and_decode_lwu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Lwu {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 6,
            }
        );
    }

    #[test]
    fn test_lwu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.bus.write(3376, 100000, 32);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            100000
        )
    }

    #[test]
    fn fetch_and_decode_ld_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Ld {
                rd: Register::X10,
                rs1: Register::X21,
                imm: 3276,
                func3: 3,
            }
        );
    }

    #[test]
    fn test_ld_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0000_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.bus.write(3376, 100000, 64);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            100000
        )
    }

    #[test]
    fn fetch_and_decode_sd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Sd {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: 110,
                func3: 3,
            }
        );
    }

    #[test]
    fn test_sd_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0010_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 100;
        soft.registers[Register::X12 as usize] = 100000;
        soft.execute();

        assert_eq!(
            soft.bus.read(&210, 64).unwrap(),
            100000
        )
    }

    #[test]
    fn fetch_and_decode_addiw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1001_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Addiw {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3276,
                func3: 0
            }
        );
    }

    #[test]
    fn test_addiw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_1100 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1001_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            4276
        )
    }

    #[test]
    fn fetch_and_decode_slliw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0001_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Slliw {
                rd: Register::X10,
                rs1: Register::X21,
                shamt: 12,
                func3: 1,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_slliw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0001_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.execute();
        let shamt: u32 = ((12i32) as u32);

        assert_eq!(
            soft.registers[Register::X10 as usize],
            (((1000u64.wrapping_shl(shamt.try_into().unwrap()) as i32) as i64) as u64)
        )
    }

    #[test]
    fn fetch_and_decode_sraiw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0001_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Sraiw {
                rd: Register::X10,
                rs1: Register::X21,
                shamt: 12,
                func3: 5,
                func7: 32,
            }
        );
    }

    #[test]
    fn test_sraiw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0001_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        let shamt = 12;
        soft.execute();
        

        assert_eq!(
            soft.registers[Register::X10 as usize],
            (((1000u64.wrapping_shr(shamt) as i32) as i64) as u64)
        );
    }

    #[test]
    fn fetch_and_decode_addw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Addw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 0
            }
        )
    }

    #[test]
    fn test_addw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 500;
        soft.registers[Register::X12 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            1500
        )
    }

    #[test]
    fn fetch_and_decode_subw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();

        assert_eq!(
            instruction,
            Instruction::Subw {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 32,
            }
        );
    }

    #[test]
    fn test_subw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.registers[Register::X12 as usize] = 300;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X10 as usize],
            700
        )
    }

    #[test]
    fn fetch_and_decode_sllw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Sllw {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 1,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_sllw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.registers[Register::X12 as usize] = 8;
        soft.execute();
        let shamt = ((0b1000 & 0x3f) as u64) as u32;

        assert_eq!(
            soft.registers[Register::X10 as usize],
            ((1000u32.wrapping_shl(shamt) as i32) as i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_srlw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Srlw {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 0,
            }
        );
    }

    #[test]
    fn test_srlw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.registers[Register::X12 as usize] = 8;
        soft.execute();
        let shamt = ((0b1000 & 0x3f) as u64) as u32;

        assert_eq!(
            soft.registers[Register::X10 as usize],
            ((1000u32.wrapping_shr(shamt) as i32) as i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_sraw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Sraw {
                rd: Register::X10,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 32,
            }
        );
    }

    #[test]
    fn test_sraw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 1000;
        soft.registers[Register::X12 as usize] = 8;
        soft.execute();
        let shamt = ((0b1000 & 0x3f) as u64) as u32;

        assert_eq!(
            soft.registers[Register::X10 as usize],
            (((1000u32 >> shamt) as i32) as i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_csrrw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Csrrw {
                rd: Register::X10,
                rs1: Register::X21,
                csr: 1036,
                func3: 1
            }
        );
    }

    #[test]
    fn test_csrrw_execution_dest_non_zero() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        soft.csr[1036usize] = 1000;
        soft.registers[Register::X21 as usize] = 500;
        soft.execute();
        let csr_val = 0b0011_1110_1000;
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            csr_val.zero_extend(&12)
        );

        assert_eq!(
            soft.csr[1036usize],
            soft.registers[Register::X21 as usize]
        )
    }

    #[test]
    fn fetch_and_decode_csrrs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Csrrs {
                rd: Register::X10,
                rs1: Register::X21,
                csr: 1036,
                func3: 2
            }
        );
    }

    #[test]
    fn test_csrrs_execution_dest_non_zero() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        soft.csr[1036usize] = 1000;
        soft.registers[Register::X21 as usize] = 500;
        soft.execute();
        let csr_val = 0b0011_1110_1000;
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            csr_val.zero_extend(&12)
        );

        assert_eq!(
            soft.csr[1036usize],
            (csr_val | soft.registers[Register::X21 as usize])
        )
    }

    #[test]
    fn fetch_and_decode_csrrc_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Csrrc {
                rd: Register::X10,
                rs1: Register::X21,
                csr: 1036,
                func3: 3
            }
        );
    }

    #[test]
    fn test_csrrc_execution_dest_non_zero() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        soft.csr[1036usize] = 1000;
        soft.registers[Register::X21 as usize] = 500;
        soft.execute();
        let csr_val = 0b0011_1110_1000;
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            csr_val.zero_extend(&12)
        );

        assert_eq!(
            soft.csr[1036usize],
            (csr_val & soft.registers[Register::X21 as usize])
        )
    }

    #[test]
    fn fetch_and_decode_csrrwi_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Csrrwi {
                rd: Register::X10,
                uimm: 21,
                csr: 1036,
                func3: 5
            }
        );
    }

    #[test]
    fn test_csrrwi_execution_dest_non_zero() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        soft.csr[1036usize] = 1000;
        soft.registers[Register::X21 as usize] = 500;
        soft.execute();
        let csr_val = 0b0011_1110_1000;
        let imm = 21u64.zero_extend(&7);
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            csr_val
        );

        assert_eq!(
            soft.csr[1036usize],
            imm
        )
    }

    #[test]
    fn fetch_and_decode_csrrsi_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Csrrsi {
                rd: Register::X10,
                uimm: 21,
                csr: 1036,
                func3: 6
            }
        );
    }

    #[test]
    fn test_csrrsi_execution_dest_non_zero() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        soft.csr[1036usize] = 1000;
        soft.registers[Register::X21 as usize] = 500;
        soft.execute();
        let csr_val = 0b0011_1110_1000;
        let imm = 21u64.zero_extend(&7);
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            csr_val
        );

        assert_eq!(
            soft.csr[1036usize],
            imm | csr_val
        )
    }

    #[test]
    fn fetch_and_decode_csrrci_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        
        assert_eq!(
            instruction,
            Instruction::Csrrci {
                rd: Register::X10,
                uimm: 21,
                csr: 1036,
                func3: 7
            }
        );
    }

    #[test]
    fn test_csrrci_execution_dest_non_zero() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b0111_0011 as u8];
        soft.load_program(program);
        soft.csr[1036usize] = 1000;
        soft.registers[Register::X21 as usize] = 500;
        soft.execute();
        let csr_val = 0b0011_1110_1000;
        let imm = 21u64.zero_extend(&7);
        
        assert_eq!(
            soft.registers[Register::X10 as usize],
            csr_val
        );

        assert_eq!(
            soft.csr[1036usize],
            imm & csr_val
        )
    }

    #[test]
    fn fetch_and_decode_mul_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Mul {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 1
            }
        )
    }

    #[test]
    fn test_mul_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 10u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            200u64
        )
    }

    #[test]
    fn fetch_and_decode_mulh_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Mulh {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 1,
                func7: 1
            }
        )
    }

    #[test]
    fn test_mulh_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1001_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 25u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            500u64
        )
    }

    #[test]
    fn fetch_and_decode_mulhsu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Mulhsu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 2,
                func7: 1
            }
        )
    }

    #[test]
    fn test_mulhsu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1010_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 25u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        let expected = 20i128.overflowing_mul(25i128).0;

        assert_eq!(
            soft.registers[Register::X11 as usize],
            expected as u64
        )
    }

    #[test]
    fn fetch_and_decode_mulhu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Mulhu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 3,
                func7: 1
            }
        )
    }

    #[test]
    fn test_mulhu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1011_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 25u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        let expected = 20u128.overflowing_mul(25u128).0;
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            expected as u64
        )
    }

    #[test]
    fn fetch_and_decode_div_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Div {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 4,
                func7: 1
            }
        )
    }

    #[test]
    fn test_div_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 5u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            (4i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_divu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Divu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 1
            }
        )
    }

    #[test]
    fn test_divu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 5u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            4u64
        )
    }

    #[test]
    fn fetch_and_decode_rem_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Rem {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 6,
                func7: 1
            }
        )
    }

    #[test]
    fn test_rem_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 17u64;
        soft.registers[Register::X21 as usize] = 25u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            (8i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_remu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Remu {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 7,
                func7: 1
            }
        )
    }

    #[test]
    fn test_remu_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 17u64;
        soft.registers[Register::X21 as usize] = 25u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            8u64
        )
    }

    #[test]
    fn fetch_and_decode_mulw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Mulw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 0,
                func7: 1
            }
        )
    }

    #[test]
    fn test_mulw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1000_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 25u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            500u64
        )
    }

    #[test]
    fn fetch_and_decode_divw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Divw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 4,
                func7: 1
            }
        )
    }

    #[test]
    fn test_divw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1100_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 5u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            (4i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_divuw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Divuw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 5,
                func7: 1
            }
        )
    }

    #[test]
    fn test_divuw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1101_0101 as u8, 0b1011_0011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 5u64;
        soft.registers[Register::X21 as usize] = 20u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            4u64
        )
    }

    #[test]
    fn fetch_and_decode_remw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Remw {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 6,
                func7: 1
            }
        )
    }

    #[test]
    fn test_remw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1110_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 17u64;
        soft.registers[Register::X21 as usize] = 25u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            (8i64) as u64
        )
    }

    #[test]
    fn fetch_and_decode_remuw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::RemuW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X12,
                func3: 7,
                func7: 1
            }
        )
    }

    #[test]
    fn test_remuw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0010 as u8, 0b1100_1010 as u8, 0b1111_0101 as u8, 0b1011_1011 as u8];
        soft.load_program(program);
        soft.registers[Register::X12 as usize] = 17u64;
        soft.registers[Register::X21 as usize] = 25u64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            8u64
        )
    }

    #[test]
    fn fetch_and_decode_lrw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0010 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::LrW {
                rd: Register::X11,
                rs1: Register::X21,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_lrw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0010 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.bus.write(200, 1000, 32);
        soft.execute();
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1000
        );

        assert!(
            soft.res.contains(&200)
        );
    }

    #[test]
    fn fetch_and_decode_scw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::ScW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_scw_execution_addr_reserved() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.res.push(200);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            0
        );

        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            soft.registers[Register::X27 as usize]
        );

        assert!(
            !soft.res.contains(&200)
        );
    }

    #[test]
    fn test_scw_execution_addr_not_reserved() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        );

        assert_ne!(
            soft.bus.read(&200, 32).unwrap(),
            soft.registers[Register::X27 as usize]
        );

        assert!(
            !soft.res.contains(&200)
        );
    }

    #[test]
    fn fetch_and_decode_amoswapw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoswapW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoswapw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();

        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            soft.registers[Register::X27 as usize]
        );
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );
    }

    #[test]
    fn fetch_and_decode_amoaddw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoaddW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoaddw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            6000
        )

    }

    #[test]
    fn fetch_and_decode_amoxorw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoxorW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoxorw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            0b1000001100000
        )
    }

    #[test]
    fn fetch_and_decode_amoandw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoandW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoandw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            0b1110001000
        )
    }

    #[test]
    fn fetch_and_decode_amoorw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoorW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoorw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            0b1001111101000
        )
    }

    #[test]
    fn fetch_and_decode_amominw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1000_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmominW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amominw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1000_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            1000
        )
    }

    #[test]
    fn fetch_and_decode_amomaxw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmomaxW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amomaxw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            5000
        )
    }

    #[test]
    fn fetch_and_decode_amominuw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmominuW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amominuw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            1000
        )
    }

    #[test]
    fn fetch_and_decode_amomaxuw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmomaxuW {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
        
    }

    #[test]
    fn test_amomaxuw_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 32);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 32).unwrap(),
            5000
        )
    }

    #[test]
    fn fetch_and_decode_lrd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0010 as u8, 0b0000_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::LrD {
                rd: Register::X11,
                rs1: Register::X21,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_lrd_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0010 as u8, 0b0000_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.bus.write(200, 1000, 64);
        soft.execute();
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1000
        );

        assert!(
            soft.res.contains(&200)
        );
    }

    #[test]
    fn fetch_and_decode_scd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::ScD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_scd_execution_address_reserved() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.res.push(200);
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            0
        );

        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            soft.registers[Register::X27 as usize]
        );

        assert!(
            !soft.res.contains(&200)
        );
        
    }

    #[test]
    fn test_scd_execution_address_not_reserved() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        );

        assert_ne!(
            soft.bus.read(&200, 64).unwrap(),
            soft.registers[Register::X27 as usize]
        );

        assert!(
            !soft.res.contains(&200)
        );
        
    }

    #[test]
    fn fetch_and_decode_amoswapd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoswapD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoswapd_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();

        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            soft.registers[Register::X27 as usize]
        );
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );
    }

    #[test]
    fn fetch_and_decode_amoxord_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoaddD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoxord_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            0b1000001100000
        )
    }

    #[test]
    fn fetch_and_decode_amoandd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoandD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoandd_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            0b1110001000
        )
        
    }

    #[test]
    fn fetch_and_decode_amoord_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmoorD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amoord_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            0b1001111101000
        )
    }

    #[test]
    fn fetch_and_decode_amomind_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1000_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmominD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amomind_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1000_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            1000
        )
    }

    #[test]
    fn fetch_and_decode_amomaxd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmomaxD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amomaxd_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            5000
        )
    }

    #[test]
    fn fetch_and_decode_amominud_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmominuD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amominud_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            1000
        )
    }

    #[test]
    fn fetch_and_decode_amomaxud_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::AmomaxuD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                aq: 0,
                rl: 1,
            }
        )
    }

    #[test]
    fn test_amomaxud_execution() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_1111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 200;
        soft.registers[Register::X27 as usize] = 1000;
        soft.bus.write(200, 5000, 64);
        soft.execute();
        
        // assert the value in rd == value in address at rs1
        assert_eq!(
            soft.registers[Register::X11 as usize],
            5000
        );

        // assert the value in memory at address in rs1 is equal to prev value + val in rs2
        assert_eq!(
            soft.bus.read(&200, 64).unwrap(),
            5000
        )
    }

    #[test]
    fn fetch_and_decode_flw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1000_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Flw {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3643,
            }
        )        
    }

    #[test]
    fn test_flw_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1000_0111 as u8];
        soft.load_program(program);
        soft.bus.write(3643, 5000, 32);
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            (f32::from_bits(5000 as u32)) as f64
        )
    }

    #[test]
    fn fetch_and_decode_fsw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Fsw {
                rs1: Register::X21,
                rs2: Register::X27,
                imm: 123,
            }
        )
    }

    #[test]
    fn test_fsw_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1010_0111 as u8];
        soft.load_program(program);
        soft.f_registers[Register::X27 as usize] = f32::from_bits(5000u32) as f64;
        soft.execute();

        assert_eq!(
            soft.bus.read(&123, 32).unwrap(),
            5000
        )
    }

    
    #[test]
    fn fetch_and_decode_fmadds_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmaddS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmadds_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0011 as u8];
        
        soft.load_program(program);
        
        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        soft.f_registers[Register::X28 as usize] = f32::from_bits(2u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let rs3_val = f32::from_bits(2u32) as f64;
        let res = rs1_val.mul_add(rs2_val, rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fmsubs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmsubS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmsubs_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0111 as u8];
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        soft.f_registers[Register::X28 as usize] = f32::from_bits(2u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let rs3_val = f32::from_bits(2u32) as f64;
        let res = rs1_val.mul_add(rs2_val, -rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fnmsubs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FnmsubS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fnmsubs_execute() {  
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        soft.f_registers[Register::X28 as usize] = f32::from_bits(2u32) as f64;
        
        soft.execute();
        
        let rs1_val =  -f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let rs3_val = -f32::from_bits(2u32) as f64;
        let res = rs1_val.mul_add(rs2_val, rs3_val);

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }
    
    #[test]
    fn fetch_and_decode_fnmadds_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FnmaddS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fnmadds_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1111 as u8];
        soft.load_program(program);
    
        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        soft.f_registers[Register::X28 as usize] = f32::from_bits(2u32) as f64;
        
        soft.execute();
        
        let rs1_val =  -f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let rs3_val = f32::from_bits(2u32) as f64;
        let res = rs1_val.mul_add(rs2_val, rs3_val);

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fadds_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FaddS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fadds_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let res = rs1_val + rs2_val;

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsubs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsubS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fsubs_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let res = rs1_val - rs2_val;

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fmuls_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmulS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmuls_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let res = rs1_val * rs2_val;

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fdivs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FdivS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )

    }

    #[test]
    fn test_fdivs_execute() {

        let mut soft = SoftThread::default();
        let program = vec![0b0001_1001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let res = rs1_val / rs2_val;

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsqrts_instruction() {

        let mut soft = SoftThread::default();
        let program = vec![0b0101_1000 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsqrtS {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fsqrts_execute() {

        let mut soft = SoftThread::default();
        let program = vec![0b0101_1000 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let res = rs1_val.sqrt();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsgnjs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fsgnjs_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
    
        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let res = rs1_val.copysign(rs2_val);

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsgnjns_instruction() {

        let mut soft = SoftThread::default();
        let program = vec![0b0010_0001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjnS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fsgnjns_execute() {

        let mut soft = SoftThread::default();
        let program = vec![0b0010_0001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = -f32::from_bits(100u32) as f64;
        let res = rs1_val.copysign(rs2_val);

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }
    
    #[test]
    fn fetch_and_decode_fsgnjxs_instruction() {

        let mut soft = SoftThread::default();
        let program = vec![0b0010_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjxS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fsgnjxs_execute() {

        let mut soft = SoftThread::default();
        let program = vec![0b0010_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);


        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let sign_1 = (f32::from_bits(200u32)).to_bits() & 0x8000_0000;
        let sign_2 = (f32::from_bits(100u32)).to_bits() & 0x8000_0000;
        let other = (f32::from_bits(200u32)).to_bits() & 0x7fff_ffff;
        let res = (f32::from_bits((sign_1 ^ sign_2) | other)) as f64;


        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )

    }

    #[test]
    fn fetch_and_decode_fmins_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FminS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fmins_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
    
        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let res = rs1_val.min(rs2_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )

    }

    #[test]
    fn fetch_and_decode_fmaxs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmaxS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fmaxs_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;
        
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let rs2_val = f32::from_bits(100u32) as f64;
        let res = rs1_val.max(rs2_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fcvtws_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtWS {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtws_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
    
        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
       
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let res = (rs1_val.round() as u32) as u64;
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fcvtwus_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtWUS {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtwus_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
       
        soft.execute();
        
        let rs1_val =  f32::from_bits(200u32) as f64;
        let res = ((rs1_val.round() as u32) as i32) as u64;
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fmvxw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0000 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmvXW {
                rd: Register::X11,
                rs1: Register::X21,
            }
        )
    }

    #[test]
    fn test_fmvxw_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0000 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
       
        soft.execute();
         
        let rs1_val =  f32::from_bits(200u32) as f64;
        let res = (((rs1_val.to_bits() & 0xffffffff) as i32) as i64) as u64;
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_feqs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FeqS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_feqs_eq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(200u32) as f64;

        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }

    #[test]
    fn test_feqs_ne_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f32::from_bits(200u32) as f64;
        soft.f_registers[Register::X27 as usize] = f32::from_bits(100u32) as f64;

        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            0
        )
    }


    #[test]
    fn fetch_and_decode_flts_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FltS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_flts_lt_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        
        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.f_registers[Register::X27 as usize] = 200f64;

        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }

    #[test]
    fn test_flts_eq_execute() { 
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        
        soft.f_registers[Register::X21 as usize] = 200.0f64;
        soft.f_registers[Register::X27 as usize] = 200.0f64;

        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            0
        )

    }


    #[test]
    fn test_flts_gt_execute() { 
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        
        soft.f_registers[Register::X21 as usize] = 300.0 as f64;
        soft.f_registers[Register::X27 as usize] = 200.0 as f64;
         
        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            0
        )

    }

    
    #[test]
    fn fetch_and_decode_fles_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FleS {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fles_lt_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100.0f64;
        soft.f_registers[Register::X27 as usize] = 200.0f64;
        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }


    #[test]
    fn test_fles_eq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200.0f64;
        soft.f_registers[Register::X27 as usize] = 200.0f64;

        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }
    

    #[test]
    fn test_fles_gt_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0001 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 300.0f64;
        soft.f_registers[Register::X27 as usize] = 200.0f64;

        soft.execute();
         
        assert_eq!(
            soft.registers[Register::X11 as usize],
            0
        )
    }

    #[test]
    fn fetch_and_decode_fclasss_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0000 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FclassS {
                rd: Register::X11,
                rs1: Register::X21,
            }
        )
    }

    #[test]
    fn test_fclasss_execute() {
        //FclassS doesn't currently
        //do anything, need to implement
        //class enum and logic for it
    }


    #[test]
    fn fetch_and_decode_fcvtsw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtSW {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_fcvtsw_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 300u64;
        soft.execute();
         
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            (((300u64) as i32) as f32) as f64 
        )
    }

    #[test]
    fn fetch_and_decode_fcvtswu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtSWU {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1,
            }
        )
    }

    #[test]
    fn test_fcvtswu_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 300u64;
        soft.execute();
         
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            (((300u64) as u32) as f32) as f64 
        )
    }

    #[test]
    fn fetch_and_decode_fmvwx_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1111_0000 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmvWX {
                rd: Register::X11,
                rs1: Register::X21,
            }
        )
    }

    #[test]
    fn test_fmvwx_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1111_0000 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 300u64;
        soft.execute(); 
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            f64::from_bits((300u64) & 0xffff_ffff) 
        )
    }

    #[test]
    fn fetch_and_decode_fcvtls_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtLS {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtls_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 300.0f64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            ((300.0f64) as f32).round() as u64 
        )
    }

    #[test]
    fn fetch_and_decode_fcvtlus_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtLUS {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtlus_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0000 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 300.0f64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            ((300.0f64) as f32).round() as u64 
        )
    }

    #[test]
    fn fetch_and_decode_fcvtsl_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtSL {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtsl_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 300u64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            ((300u64) as f32) as f64 
        )
        
    }

    #[test]
    fn fetch_and_decode_fcvtslu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtSLU {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtslu_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0000 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 300u64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            ((300u64) as f32) as f64 
        )
    }


    #[test]
    fn fetch_and_decode_fld_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1000_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Fld {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3643,
            }
        )        
    }

    #[test]
    fn test_fld_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1000_0111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 3643;
        let val = (5000.0f64).to_bits();
        soft.bus.write(3643, val, 64);
        soft.execute();
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            f64::from_bits(val)
        )
    }
    
   
    #[test]
    fn fetch_and_decode_fsd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Fsd {
                rs1: Register::X21,
                rs2: Register::X27,
                imm: 123,
            }
        )
    }

    #[test]
    fn test_fsd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1011_0101 as u8, 0b1010_0111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 123;
        soft.f_registers[Register::X27 as usize] = f64::from_bits(5000u64);
        soft.execute();

        assert_eq!(
            soft.bus.read(&123, 64).unwrap(),
            5000
        )
    }
    
    #[test]
    fn fetch_and_decode_fmaddd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmaddD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmaddd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val =  f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }


    #[test]
    fn fetch_and_decode_fmsubd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmsubD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmsubd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0111 as u8];
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val =  f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, -rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fnmsubd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FnmsubD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fnmsubd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val =  -f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, -rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fnmaddd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FnmaddD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fnmaddd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1111 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val = -f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_faddd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FaddD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_faddd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64); 
        let res = rs1_val + rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsubd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsubD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fsubd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64); 
        let res = rs1_val - rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }
    
    #[test]
    fn fetch_and_decode_fmuld_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmulD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmuld_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64); 
        let res = rs1_val * rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }    

    #[test]
    fn fetch_and_decode_fdivd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FdivD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fdivd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64); 
        let res = rs1_val / rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }    

    #[test]
    fn fetch_and_decode_fsqrtd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0101_1010 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsqrtD {
                rd: Register::X11,
                rs1: Register::X21, 
                rm: 2
            }
        )
    }

    #[test]
    fn test_fsqrtd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0101_1010 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
    
        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64); 
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let res = rs1_val.sqrt();
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )

    }

    #[test]
    fn fetch_and_decode_fsgnjd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27, 
            }
        )
    }

    #[test]
    fn test_fsgnjd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64); 
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let res = rs1_val.copysign(rs2_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }
    
    #[test]
    fn fetch_and_decode_fsgnjnd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjnD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27, 
            }
        )
    }

    #[test]
    fn test_fsgnjnd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64); 
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let res = rs1_val.copysign(-rs2_val);
 
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsgnjxd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjxD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27, 
            }
        )
    }

    #[test]
    fn test_fsgnjxd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64); 
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        soft.execute();
        
        let sign_1 = f64::from_bits(200u64).to_bits() & 0x8000_0000_0000_0000;
        let sign_2 = f64::from_bits(100u64).to_bits() & 0x8000_0000_0000_0000;
        let other =  f64::from_bits(200u64).to_bits() & 0x7fff_ffff_ffff_ffff;

        let res = f64::from_bits((sign_1 ^ sign_2) | other);
 
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )

    }

    #[test]
    fn fetch_and_decode_fmind_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0010_1011 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FminD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
            }
        )
    }

    #[test]
    fn test_fmind_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0010_1011 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.f_registers[Register::X27 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            100f64
        )
    }

    #[test]
    fn fetch_and_decode_fmaxd_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0010_1011 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmaxD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
            }
        )
    }

    #[test]
    fn test_fmaxd_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0010_1011 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.f_registers[Register::X27 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            200f64
        )
    }

    #[test]
    fn fetch_and_decode_fcvtsd_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0100_0000 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtSD {
                rd: Register::X11,
                rs1: Register::X21, 
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtsd_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0100_0000 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            100f64
        )

    }

    #[test]
    fn fetch_and_decode_fcvtds_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0100_0010 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtDS {
                rd: Register::X11,
                rs1: Register::X21, 
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtds_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b0100_0010 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            (100f32) as f64
        )
    }

    #[test]
    fn fetch_and_decode_feqd_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FeqD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,  
            }
        )
    }

    #[test]
    fn test_feqd_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.f_registers[Register::X27 as usize] = 100f64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }

    #[test]
    fn fetch_and_decode_fltd_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FltD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,  
            }
        )

    }

    #[test]
    fn test_fltd_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 50f64;
        soft.f_registers[Register::X27 as usize] = 100f64;
        soft.execute();
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }

    #[test]
    fn fetch_and_decode_fled_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FleD {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,  
            }
        )
    }

    #[test]
    fn test_fled_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1010_0011 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 50f64;
        soft.f_registers[Register::X27 as usize] = 100f64;
        soft.execute();
        
        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }    

    #[test]
    fn fetch_and_decode_fclassd_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1110_0010 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FclassD {
                rd: Register::X11,
                rs1: Register::X21,
            }
        )
    }

    
    #[test]
    fn fetch_and_decode_fcvtwd_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtWD {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtwd_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();
        assert_eq!(
            soft.registers[Register::X11 as usize],
            ((100f64).round() as i32) as u64
        )
    } 


    #[test]
    fn fetch_and_decode_fcvtwud_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtWUD {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtwud_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();
        assert_eq!(
            soft.registers[Register::X11 as usize],
            (((100f64).round() as u32) as i32) as u64
        )
    }

    #[test]
    fn fetch_and_decode_fcvtdw_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtDW {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtdw_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 100u64;
        soft.execute();
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            ((100u64) as i32) as f64
        )
    } 

    #[test]
    fn fetch_and_decode_fcvtdwu_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtDWU {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtdwu_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 100u64;
        soft.execute();
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            ((100u64) as u32) as f64
        )
    } 

    #[test]
    fn fetch_and_decode_fcvtld_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0010_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtLD {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtld_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0010_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();
        assert_eq!(
            soft.registers[Register::X11 as usize],
            100f64.round() as u64
        )
    } 

    #[test]
    fn fetch_and_decode_fcvtlud_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtLUD {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtlud_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1100_0010 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();
        assert_eq!(
            soft.registers[Register::X11 as usize],
            100f64.round() as u64
        )
    }
    
    #[test]
    fn fetch_and_decode_fmvxd_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1110_0010 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmvXD {
                rd: Register::X11,
                rs1: Register::X21,
            }
        )
    }

    #[test]
    fn test_fmvxd_execute() { 
        let mut soft = SoftThread::default(); 
        let program = vec![0b1110_0010 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();
        assert_eq!(
            soft.registers[Register::X11 as usize],
            100f64.to_bits()
        )
    }    

    #[test]
    fn fetch_and_decode_fcvtdl_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0010_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtDL {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtdl_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0010_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
    
        soft.registers[Register::X21 as usize] = 100u64;
        soft.execute();
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            100u64 as f64
        )
    }    

    #[test]
    fn fetch_and_decode_fcvtdlu_instruction() { 
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtDLU {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }
    
    #[test]
    fn test_fcvtdlu_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1101_0010 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 100u64;
        soft.execute();
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            100u64 as f64
        )
    }

    #[test]
    fn fetch_and_decode_fmvdx_instruction() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1111_0010 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmvDX {
                rd: Register::X11,
                rs1: Register::X21, 
            }
        )
    }

    #[test]
    fn test_fmvdx_execute() {
        let mut soft = SoftThread::default(); 
        let program = vec![0b1111_0010 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8]; 
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.execute();
        assert_eq!(
            soft.registers[Register::X11 as usize],
            100f64.to_bits()
        )
    }    

    #[test]
    fn fetch_and_decode_flq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1100_0101 as u8, 0b1000_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Flq {
                rd: Register::X11,
                rs1: Register::X21,
                imm: 3643,
            }
        )        
    }

    #[test]
    fn test_flq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1100_0101 as u8, 0b1000_0111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 3643;
        let val = (5000.0f64).to_bits();
        soft.bus.write(3643, val, 64);
        soft.execute();
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            f64::from_bits(val)
        )
    } 
    
    #[test]
    fn fetch_and_decode_fsq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1100_0101 as u8, 0b1010_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::Fsq {
                rs1: Register::X21,
                rs2: Register::X27,
                imm: 123,
            }
        )
    }

    #[test]
    fn test_fsq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0011 as u8, 0b1011_1010 as u8, 0b1100_0101 as u8, 0b1010_0111 as u8];
        soft.load_program(program);
        soft.registers[Register::X21 as usize] = 123;
        soft.f_registers[Register::X27 as usize] = f64::from_bits(5000u64);
        soft.execute();

        assert_eq!(
            soft.bus.read(&123, 64).unwrap(),
            5000
        )
    }

    #[test]
    fn fetch_and_decode_fmaddq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmaddQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmaddq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val =  f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fmsubq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmsubQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmsubq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_0111 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val =  f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, -rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fnmsubq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FnmsubQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fnmsubq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val =  -f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, -rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fnmaddq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1111 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FnmaddQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rs3: Register::X28,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fnmaddq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1100_1111 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        soft.f_registers[Register::X28 as usize] = f64::from_bits(2u64);
        
        soft.execute();
        
        let rs1_val =  -f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let rs3_val = f64::from_bits(2u64);
        let res = rs1_val.mul_add(rs2_val, rs3_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_faddq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FaddQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_faddq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let res = rs1_val + rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }    

    #[test]
    fn fetch_and_decode_fsubq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsubQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fsubq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0000_1111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let res = rs1_val - rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fmulq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmulQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fmulq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let res = rs1_val * rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fdivq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FdivQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fdivq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0001_1111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64);
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let rs2_val = f64::from_bits(100u64);
        let res = rs1_val / rs2_val;
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsqrtq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0101_1110 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsqrtQ {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 2
            }
        )
    }

    #[test]
    fn test_fsqrtq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0101_1110 as u8, 0b0000_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64);
        
        soft.execute();
        
        let rs1_val = f64::from_bits(200u64);
        let res = rs1_val.sqrt();
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fsgnjq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0111 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fsgnjq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0111 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.f_registers[Register::X27 as usize] = 100f64;
        soft.execute();

        let rs1_val = 200f64;
        let rs2_val = 100f64;
        let res = rs1_val.copysign(rs2_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )

    }

    #[test]
    fn fetch_and_decode_fsgnjnq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0111 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjnQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fsgnjnq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0111 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.f_registers[Register::X27 as usize] = 100f64;
        soft.execute();

        let rs1_val = 200f64;
        let rs2_val = -100f64;
        let res = rs1_val.copysign(rs2_val);
        
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )

    }

    #[test]
    fn fetch_and_decode_fsgnjxq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FsgnjxQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fsgnjxq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = f64::from_bits(200u64); 
        soft.f_registers[Register::X27 as usize] = f64::from_bits(100u64); 
        soft.execute();

        let sign_1 = f64::from_bits(200u64).to_bits() & 0x8000_0000_0000_0000;
        let sign_2 = f64::from_bits(100u64).to_bits() & 0x8000_0000_0000_0000;
        let other =  f64::from_bits(200u64).to_bits() & 0x7fff_ffff_ffff_ffff;

        let res = f64::from_bits((sign_1 ^ sign_2) | other);
 
        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            res
        )
    }

    #[test]
    fn fetch_and_decode_fminq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1111 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FminQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fminq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1111 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        
        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.f_registers[Register::X27 as usize] = 100f64;

        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            100f64
        )
    }

    #[test]
    fn fetch_and_decode_fmaxq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1111 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FmaxQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27
            }
        )
    }

    #[test]
    fn test_fmaxq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0010_1111 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
    
        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.f_registers[Register::X27 as usize] = 100f64;

        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            200f64
        )
    }

    #[test]
    fn fetch_and_decode_fcvtsq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtSQ {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtsq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0000 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            200f64
        )
    }

    #[test]
    fn fetch_and_decode_fcvtqs_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0110 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtQS {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtqs_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0110 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            200f64
        )
    }
    
    #[test]
    fn fetch_and_decode_fcvtdq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0010 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtDQ {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtdq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0010 as u8, 0b0011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            (200 as f32) as f64 
        )
    }

    #[test]
    fn fetch_and_decode_fcvtqd_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0110 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtQD {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 1
            }
        )
    }

    #[test]
    fn test_fcvtqd_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b0100_0110 as u8, 0b0001_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            (200 as f32) as f64 
        )
    }

    #[test]
    fn fetch_and_decode_feqq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FeqQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
            }
        )
    }

    #[test]
    fn test_feqq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0111 as u8, 0b1011_1010 as u8, 0b1010_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 100f64;
        soft.f_registers[Register::X27 as usize] = 100f64;

        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }

    #[test]
    fn fetch_and_decode_fltq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0111 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FltQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
            }
        )
    }

    #[test]
    fn test_fltq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0111 as u8, 0b1011_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        
        soft.f_registers[Register::X21 as usize] = 50f64;
        soft.f_registers[Register::X27 as usize] = 100f64;

        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }

    #[test]
    fn fetch_and_decode_fleq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0111 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FleQ {
                rd: Register::X11,
                rs1: Register::X21,
                rs2: Register::X27,
            }
        )
    }

    #[test]
    fn test_fleq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1010_0111 as u8, 0b1011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 50f64;
        soft.f_registers[Register::X27 as usize] = 100f64;

        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            1
        )
    }

    #[test]
    fn fetch_and_decode_fclassq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1110_0110 as u8, 0b0000_1010 as u8, 0b1001_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FclassQ {
                rd: Register::X11,
                rs1: Register::X21,
            }
        )
    }

    #[test]
    fn fetch_and_decode_fcvtwq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtWQ {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }
    

    #[test]
    fn test_fcvtwq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        
        soft.load_program(program);
        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            ((200f64.round() as i32) as u64)
        )
    }

    #[test]
    fn fetch_and_decode_fcvtwuq_instruction() { 
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0001_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtWUQ {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }
    
    #[test]
    fn test_fcvtwuq_execute() { 
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0001_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            (((200f64.round() as u32) as i32) as u64)
        )
    }

    #[test]
    fn fetch_and_decode_fcvtqw_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtQW {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }
    
    #[test]
    fn test_fcvtqw_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0000_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 200u64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            ((200f64.round() as i32) as f64)
        )
    }
    
    #[test]
    fn fetch_and_decode_fcvtqwu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0001_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtQWU {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtqwu_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0001_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 200u64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            ((200f64.round() as u32) as f64)
        )
    }

    #[test]
    fn fetch_and_decode_fcvtlq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtLQ {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtlq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            ((200f64.round() as u32) as u64)
        )
    }

    #[test]
    fn fetch_and_decode_fcvtluq_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtLUQ {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtluq_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1100_0110 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.f_registers[Register::X21 as usize] = 200f64;
        soft.execute();

        assert_eq!(
            soft.registers[Register::X11 as usize],
            200f64.round() as u64
        )
    }

    #[test]
    fn fetch_and_decode_fcvtql_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtQL {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtql_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0010_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);

        soft.registers[Register::X21 as usize] = 200u64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            200u64 as f64
        )
    }

    #[test]
    fn fetch_and_decode_fcvtqlu_instruction() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
        let instruction: Instruction = soft.fetch().into();
        assert_eq!(
            instruction,
            Instruction::FcvtQLU {
                rd: Register::X11,
                rs1: Register::X21,
                rm: 0
            }
        )
    }

    #[test]
    fn test_fcvtqlu_execute() {
        let mut soft = SoftThread::default();
        let program = vec![0b1101_0110 as u8, 0b0011_1010 as u8, 0b1000_0101 as u8, 0b1101_0011 as u8];
        soft.load_program(program);
    
        soft.registers[Register::X21 as usize] = 200u64;
        soft.execute();

        assert_eq!(
            soft.f_registers[Register::X11 as usize],
            200u64 as f64
        )
    }
}
