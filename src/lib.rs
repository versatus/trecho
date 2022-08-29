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
    use crate::encoding::OpCodeType;

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
}