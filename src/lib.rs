pub mod register;
pub mod instructions;
pub mod soft;
pub mod vm;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::{Register, RegisterAbi, HardWiredZero};

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
}