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

    // #[test]
    // fn test_unpack_random_r4_type_bites() {
    //     let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1011_0111 as u32;
    //     let unpacked: Unpacked = bits.into();
    //     assert!(unpacked.imm.is_some());
    //     assert!(unpacked.rd.is_some());
    //     assert_eq!(unpacked.opcode, 55);
    // }

    // #[test]
    // fn test_unpack_random_r_type_bits_opcode_into_instructions() {
    //     let bits: Inst = 0b1100_1100_1100_1100_1100_1100_1011_0011 as u32;
    //     let unpacked: Unpacked = bits.into();
    //     println!("{:?}", unpacked);
    //     assert!(true);
    // }
}