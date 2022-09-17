#![allow(unused, unused_mut, dead_code)]
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

pub trait RegisterValue:
    Sized +
    Clone +
    Not<Output = Self> +
    BitAnd<Output = Self> +
    BitOr<Output = Self> +
    BitXor<Output = Self> +
    Shl<Output = Self> +
    Shr<Output = Self> +

{
    const BITS: u8;
    const SHIFT_MASK: u8;

    fn zero() -> Self;
    fn one() -> Self;
    fn min_val() -> Self;
    fn max_val() -> Self;
    fn equal(&self, other: &Self) -> Self;
    fn less_than(&self, other: &Self) -> Self;
    fn less_than_signed(&self, other: &Self) -> Self;
    fn not_equal(&self, rhs: &Self) -> Self;
    fn greater_equal(&self, rhs: &Self) -> Self;
    fn greater_equal_signed(&self, rhs: &Self) -> Self;
    fn logical_not(&self) -> Self;
    fn condition(&self, tval: &Self, fval: &Self) -> Self;
    fn overflowing_add(&self, rhs: &Self) -> Self;
    fn overflowing_sub(&self, rhs: &Self) -> Self;
    fn overflowing_mul(&self, rhs: &Self) -> Self;
    fn overflowing_div(&self, rhs: &Self) -> Self;
    fn overflowing_div_euclid(&self, rhs: &Self) -> Self;
    fn overflowing_rem(&self, rhs: &Self) -> Self;
    fn overflowing_rem_euclid(&self, rhs: &Self) -> Self;
    fn overflowing_mul_high_signed(&self, rhs: &Self) -> Self;
    fn overflowing_mul_high_unsigned(&self, rhs: &Self) -> Self;
    fn overflowing_mul_high_signed_unsigned(&self, rhs: &Self) -> Self;
    fn overflowing_div_signed(&self, rhs: &Self) -> Self;
    fn overflowing_rem_signed(&self, rhs: &Self) -> Self;
    fn overflowing_neg(&self) -> Self;
    fn overflowing_pow(&self, exp: u32) -> Self;
    fn overflowing_shl(&self, bits: u32) -> Self;
    fn overflowing_shr(&self, bits: u32) -> Self;
    fn msb_zeros(&self) -> Self;
    fn lsb_zeros(&self) -> Self;
    fn n_ones(&self) -> Self;
    fn mul_no_carry(&self, rhs: &Self) -> Self;
    fn mul_no_carry_high(&self, rhs: &Self) -> Self;
    fn mul_no_carry_rev(&self, rhs: &Self) -> Self;
    fn of_no_carry_byte(&self) -> Self;
    fn revb(&self) -> Self;
    fn shl_signed(&self, bits: &Self) -> Self;
    fn shr_signed(&self, bits: &Self) -> Self;
    fn rotatel(&self, rhs: &Self) -> Self;
    fn rotater(&self, rhs: &Self) -> Self;
    fn zero_extend(&self, start: &Self) -> Self;
    fn sign_extend(&self, start: &Self) -> Self;
}

// The basic register addresses
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HardWiredZero;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReturnAddress;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StackPointer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalPointer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThreadPointer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TemporaryAlternateLink;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Temporary;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SavedRegisterFramePointer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SavedRegister;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionArgumentReturnValues;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionArgument;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegisterAbi {
    Zero(HardWiredZero),
    Ra(ReturnAddress),
    Sp(StackPointer),
    Gp(GlobalPointer),
    Tp(ThreadPointer),
    T0(TemporaryAlternateLink),
    T1(Temporary),
    T2(Temporary),
    S0(SavedRegisterFramePointer),
    S1(SavedRegister),
    A0(FunctionArgumentReturnValues),
    A1(FunctionArgumentReturnValues),
    A2(FunctionArgument),
    A3(FunctionArgument),
    A4(FunctionArgument),
    A5(FunctionArgument),
    A6(FunctionArgument),
    A7(FunctionArgument),
    S2(SavedRegister),
    S3(SavedRegister),
    S4(SavedRegister),
    S5(SavedRegister),
    S6(SavedRegister),
    S7(SavedRegister),
    S8(SavedRegister),
    S9(SavedRegister),
    S10(SavedRegister),
    S11(SavedRegister),
    T3(Temporary),
    T4(Temporary),
    T5(Temporary),
    T6(Temporary)
}

impl From<Register> for RegisterAbi {
    fn from(reg: Register) -> RegisterAbi {
        match reg {
            Register::X0 => return RegisterAbi::Zero(HardWiredZero),
            Register::X1 => return RegisterAbi::Ra(ReturnAddress),
            Register::X2 => return RegisterAbi::Sp(StackPointer),
            Register::X3 => return RegisterAbi::Gp(GlobalPointer),
            Register::X4 => return RegisterAbi::Tp(ThreadPointer),
            Register::X5 => return RegisterAbi::T0(TemporaryAlternateLink),
            Register::X6 => return RegisterAbi::T1(Temporary),
            Register::X7 => return RegisterAbi::T2(Temporary),
            Register::X8 => return RegisterAbi::S0(SavedRegisterFramePointer),
            Register::X9 => return RegisterAbi::S1(SavedRegister),
            Register::X10 => return RegisterAbi::A0(FunctionArgumentReturnValues),
            Register::X11 => return RegisterAbi::A1(FunctionArgumentReturnValues),
            Register::X12 => return RegisterAbi::A2(FunctionArgument),
            Register::X13 => return RegisterAbi::A3(FunctionArgument),
            Register::X14 => return RegisterAbi::A4(FunctionArgument),
            Register::X15 => return RegisterAbi::A5(FunctionArgument),
            Register::X16 => return RegisterAbi::A6(FunctionArgument),
            Register::X17 => return RegisterAbi::A7(FunctionArgument),
            Register::X18 => return RegisterAbi::S2(SavedRegister),
            Register::X19 => return RegisterAbi::S3(SavedRegister),
            Register::X20 => return RegisterAbi::S4(SavedRegister),
            Register::X21 => return RegisterAbi::S5(SavedRegister),
            Register::X22 => return RegisterAbi::S6(SavedRegister),
            Register::X23 => return RegisterAbi::S7(SavedRegister),
            Register::X24 => return RegisterAbi::S8(SavedRegister),
            Register::X25 => return RegisterAbi::S9(SavedRegister),
            Register::X26 => return RegisterAbi::S10(SavedRegister),
            Register::X27 => return RegisterAbi::S11(SavedRegister),
            Register::X28 => return RegisterAbi::T3(Temporary),
            Register::X29 => return RegisterAbi::T4(Temporary),
            Register::X30 => return RegisterAbi::T5(Temporary),
            Register::X31 => return RegisterAbi::T6(Temporary),
        }
    }
}

impl From<usize> for Register {
    fn from(i: usize) -> Register {
        assert!(i < 32);
        match i {
            0 => Register::X0,
            1 => Register::X1,
            2 => Register::X2,
            3 => Register::X3,
            4 => Register::X4,
            5 => Register::X5,
            6 => Register::X6,
            7 => Register::X7,
            8 => Register::X8,
            9 => Register::X9,
            10 => Register::X10,
            11 => Register::X11,
            12 => Register::X12,
            13 => Register::X13,
            14 => Register::X14,
            15 => Register::X15,
            16 => Register::X16,
            17 => Register::X17,
            18 => Register::X18,
            19 => Register::X19,
            20 => Register::X20,
            21 => Register::X21,
            22 => Register::X22,
            23 => Register::X23,
            24 => Register::X24,
            25 => Register::X25,
            26 => Register::X26,
            27 => Register::X27,
            28 => Register::X28,
            29 => Register::X29,
            30 => Register::X30,
            31 => Register::X31,
            _ => panic!("Accessing an Invalid Register")
        }
    }
}

impl From<Register> for usize {
    fn from(reg: Register) -> usize {
        match reg {
            Register::X0 => 0,
            Register::X1 => 1,
            Register::X2 => 2,
            Register::X3 => 3,
            Register::X4 => 4,
            Register::X5 => 5,
            Register::X6 => 6,
            Register::X7 => 7,
            Register::X8 => 8,
            Register::X9 => 9,
            Register::X10 => 10,
            Register::X11 => 11,
            Register::X12 => 12,
            Register::X13 => 13,
            Register::X14 => 14,
            Register::X15 => 15,
            Register::X16 => 16,
            Register::X17 => 17,
            Register::X18 => 18,
            Register::X19 => 19,
            Register::X20 => 20,
            Register::X21 => 21,
            Register::X22 => 22,
            Register::X23 => 23,
            Register::X24 => 24,
            Register::X25 => 25,
            Register::X26 => 26,
            Register::X27 => 27,
            Register::X28 => 28,
            Register::X29 => 29,
            Register::X30 => 30,
            Register::X31 => 31,
        }
    }
}

impl Default for Register {
    fn default() -> Register {
        return Register::X0;
    }
}

impl RegisterValue for u64 {
    const BITS: u8 = 64;
    const SHIFT_MASK: u8 = 0x3F;

    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
    fn min_val() -> Self { u64::MIN }
    fn max_val() -> Self { u64::MAX }
    
    fn equal(&self, other: &Self) -> Self { 
        (self == other).into() 
    }
    
    fn less_than(&self, other: &Self) -> Self { 
        (self < other).into() 
    }
    
    fn less_than_signed(&self, other: &Self) -> Self { 
        ((*self as i64) < (*other as i64)).into() 
    }
    
    fn not_equal(&self, rhs: &Self) -> Self { 
        self.equal(rhs).logical_not() 
    }
    
    fn greater_equal(&self, rhs: &Self) -> Self { 
        self.less_than(rhs).logical_not() 
    }
    
    fn greater_equal_signed(&self, rhs: &Self) -> Self { 
        self.less_than_signed(rhs).logical_not() 
    }
    
    fn logical_not(&self) -> Self { 
        (*self != Self::one()).into() 
    }
    
    fn condition(&self, tval: &Self, fval: &Self) -> Self { 
        if *self == Self::one() {
            *tval
        } else {
            *fval
        }
    }
    
    fn overflowing_add(&self, rhs: &Self) -> Self { 
        (*self).overflowing_add(*rhs).0    
    }
    
    fn overflowing_sub(&self, rhs: &Self) -> Self { 
        (*self).overflowing_sub(*rhs).0
    }
    
    fn overflowing_mul(&self, rhs: &Self) -> Self {
        (*self).overflowing_mul(*rhs).0
    }
    fn overflowing_div(&self, rhs: &Self) -> Self {
        if *rhs == 0 {
            Self::max_val()
        } else {
            (*self).overflowing_div(*rhs).0
        }
    }

    fn overflowing_div_euclid(&self, rhs: &Self) -> Self {
        if *rhs == 0 {
            Self::max_val()
        } else {
            (*self).overflowing_div_euclid(*rhs).0
        }
    }

    fn overflowing_div_signed(&self, rhs: &Self) -> Self {
        if *rhs == 0 {
            (-1i64) as u64
        } else {
            let (val, overflow) = (*self as i64).overflowing_div(*rhs as i64);
            if overflow {
                ((-1i64) as u64) << (<Self as RegisterValue>::BITS - 1)
            } else {
                val as u64
            }
        }
    }
    
    fn overflowing_rem(&self, rhs: &Self) -> Self {
        if *rhs == 0 {
            *self
        } else {
            (*self).overflowing_rem(*rhs).0
        }
    }
    
    fn overflowing_rem_euclid(&self, rhs: &Self) -> Self {
        if *rhs == 0 {
            *self
        } else {
            (*self).overflowing_rem_euclid(*rhs).0
        }
    }

    fn overflowing_rem_signed(&self, rhs: &Self) -> Self {
        if *rhs == 0 {
            *self
        } else {
            let (val, overflow) = (*self as i64).overflowing_rem(*rhs as i64);
            if overflow {
                0
            } else {
                val as u64
            }
        }
    }

    fn overflowing_mul_high_signed(&self, rhs: &Self) -> Self {
        let a = i128::from(*self as i64);
        let b = i128::from(*rhs as i64);
        let (val, _) = a.overflowing_mul(b);
        (val >> 64) as u64
    }

    fn overflowing_mul_high_unsigned(&self, rhs: &Self) -> Self {
        let a = u128::from(*self);
        let b = u128::from(*rhs);
        let (val, _) = a.overflowing_mul(b);
        (val >> 64) as u64
    }

    fn overflowing_mul_high_signed_unsigned(&self, rhs: &Self) -> Self {
        let a = i128::from(*self as i64);
        let b = i128::from(*self);
        let (val, _) = a.overflowing_mul(b);
        (val >> 64) as u64

    }
    
    fn overflowing_neg(&self) -> Self { 
        (*self).overflowing_neg().0    
    }
    
    fn overflowing_pow(&self, exp: u32) -> Self { 
        (*self).overflowing_pow(exp).0    
    }

    fn overflowing_shl(&self, bits: u32) -> Self {
        (*self).overflowing_shl(bits).0
    }

    fn overflowing_shr(&self, bits: u32) -> Self {
        (*self).overflowing_shr(bits).0
    }

    fn msb_zeros(&self) -> Self { self.leading_zeros() as u64 }
    fn lsb_zeros(&self) -> Self { self.trailing_zeros() as u64 }
    fn n_ones(&self) -> Self{ self.count_ones() as u64 }

    fn mul_no_carry(&self, rhs: &Self) -> Self { 
        let mut x: u64 = 0;
        (0..64).into_iter().for_each(|i| {
            if ((rhs >> i) & 1) != 0 {
                x ^= self << i
            }
        });

        x
    }

    fn mul_no_carry_high(&self, rhs: &Self) -> Self {
        let mut x: u64 = 0;
        (0..64).into_iter().for_each(|i| {
            if ((rhs >> i) & 1) != 0 {
                x ^= self << i
            }
        });

        x
    }

    fn mul_no_carry_rev(&self, rhs: &Self) -> Self {
        let mut x: u64 = 0;
        (0..64).into_iter().for_each(|i| {
            if ((rhs >> i) & 1) != 0 {
                x ^= self >> (63 - i);
            }
        });

        x
    }

    fn of_no_carry_byte(&self) -> Self {
        let mut rev_rem = 0;
        if self & 0x_0000_0000_0000_00ff != 0 {
            rev_rem |= 0x_0000_0000_0000_00ff
        }
    
        if self & 0x_0000_0000_0000_ff00 != 0 {
            rev_rem |= 0x_0000_0000_0000_ff00
        }

        if self & 0x_0000_0000_00ff_0000 != 0 {
            rev_rem |= 0x_0000_0000_00ff_0000
        }

        if self & 0x_0000_0000_ff00_0000 != 0 {
            rev_rem |= 0x_0000_0000_ff00_0000
        }

        if self & 0x_0000_00ff_0000_0000 != 0 {
            rev_rem |= 0x_0000_00ff_0000_000
        }

        if self & 0x_0000_ff00_0000_0000 != 0 {
            rev_rem |= 0x_0000_ff00_0000_0000
        }

        if self & 0x_00ff_0000_0000_0000 != 0 {
            rev_rem |= 0x_00ff_0000_0000_0000
        }

        if self & 0xff00_0000_0000_0000 != 0 {
            rev_rem |= 0x_ff00_0000_0000_0000
        }

        rev_rem
    }


    fn revb(&self) -> Self {
        let mut rev = 0;

        let adj = self & 0x_0000_0000_0000_00ff;
        rev |= adj << 56;

        let adj = self & 0x_0000_0000_0000_ff00;
        rev |= adj << 40;

        let adj = self & 0x_0000_0000_00ff_0000;
        rev |= adj << 24;

        let adj = self & 0x_0000_0000_ff00_0000;
        rev |= adj << 8;

        let adj = self & 0x_0000_00ff_0000_0000;
        rev |= adj >> 8;

        let adj = self & 0x_0000_ff00_0000_0000;
        rev |= adj >> 24;
        
        let adj = self & 0x_00ff_0000_0000_0000;
        rev |= adj >> 40;

        let adj = self & 0x_ff00_0000_0000_0000;
        rev |= adj >> 56;

        rev
    }

    fn shl_signed(&self, bits: &Self) -> Self {
        (*self as i64).shl(*bits) as u64
    }

    fn shr_signed(&self, bits: &Self) -> Self {
        (*self as i64).shr(*bits) as u64    
    }

    fn rotatel(&self, rhs: &Self) -> Self {
        (*self as u64).rotate_left((*rhs) as u32) as u64    
    }

    fn rotater(&self, rhs: &Self) -> Self {
        (*self as u64).rotate_right((*rhs) as u32) as u64
    }

    fn zero_extend(&self, start: &Self) -> Self {
        let start = std::cmp::min(*start, 64);
        (*self << (64 - start)) >> (64 - start)
    }

    fn sign_extend(&self, start: &Self) -> Self {
        let start = std::cmp::min(*start, 64);
        (((*self << (64 - start)) as i64) >> (64 - start)) as u64 
    }
}