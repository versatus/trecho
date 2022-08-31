pub mod register;
pub mod instructions;
pub mod soft;
pub mod vm;
pub mod extensions;
pub mod encoding;
pub mod encoding_types;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::{Register, RegisterAbi, HardWiredZero};
    use crate::encoding::{OpCodeType, Unpacked, Decoder};
    use crate::encoding_types::*;

    #[test]
    fn test_match_register() {
        let reg = Register::X0;

        match reg {
            Register::X0 => { println!("Matched Register 0"); assert!(true);}
            _ => { assert!(false); }
        }
    }

    #[test]
    fn test_convert_register_to_abi() {
        let reg = Register::X0;
        let abi: RegisterAbi = reg.into();
        assert_eq!(abi, RegisterAbi::Zero(HardWiredZero))
    }

    #[test]
    fn test_convert_valid_opcode_to_opcode_type() {
        let opcode = 0b0010111 as u8;
        let opcode_type: OpCodeType = opcode.into();
        assert_eq!(opcode_type, OpCodeType::U);
    }

    #[test]
    fn test_convert_invalid_opcode_to_opcode_type() {
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
        println!("{:?}", unpacked);
    }

    // TODO: Add test cases for each `Instruction` struct variant type with a fixed u32 inst to test conversion.
    #[test]
    fn test_convert_lui_bits_into_instruction() {
        unimplemented!()
    }
    
    #[test]
    fn test_convert_auipc_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_jal_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_jalr_bits_into_instruction() {
        unimplemented!()    
    }

    #[test]
    fn test_convert_beq_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_bne_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_blt_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_bge_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_bltu_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_bgeu_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_lb_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_lh_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_lw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_lbu_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_lhu_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sb_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sh_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_addi_bits_into_instruction() {
        unimplemented!()   
    }

    #[test]
    fn test_convert_slti_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sltiu_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_xori_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_ori_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_andi_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_slli_bits_into_instruction() {
        unimplemented!()
    }
    
    #[test]
    fn test_convert_srli_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_srai_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_add_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sub_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sll_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_slt_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sltu_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_xor_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_srl_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sra_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_or_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_and_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_fence_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_ecall_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_ebreak_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_lwu_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_ld_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sd_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_addiw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_slliw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_srliw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sraiw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_addw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_subw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sllw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_srlw_bits_into_instruction() {
        unimplemented!()
    }

    #[test]
    fn test_convert_sraw_bits_into_instruction() {
        unimplemented!()
    }
}
