pub mod encoding;
pub mod encoding_types;
pub mod extensions;
pub mod instructions;
pub mod register;
pub mod soft;
pub mod vm;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::{InstructionDecoder, OpCodeType, Unpacked};
    use crate::encoding_types::*;
    use crate::instructions::Instruction;
    use crate::register::{HardWiredZero, Register, RegisterAbi};

    #[test]
    fn test_match_register() {
        let reg = Register::X0;

        match reg {
            Register::X0 => {
                println!("Matched Register 0");
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

    // TODO: Add test cases for each `Instruction` struct variant type with a fixed u32 inst to test conversion.
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
        assert_eq!(
            instruction,
            Instruction::Sb {
                rs1: Register::X21,
                rs2: Register::X12,
                imm: -821,
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
                imm: -821,
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
                imm: -821,
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
        // println!("{:?}", instruction);
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
}
