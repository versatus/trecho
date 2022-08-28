pub mod register;


mod tests {
    use super::*;
    use crate::register::Register;
    #[test]
    fn test_match_register() {
        let reg = Register::X0;

        match reg {
            Register::X0 => { println!("Matched Register 0"); assert!(true);}
            _ => { assert!(false); }
        }
    }
}