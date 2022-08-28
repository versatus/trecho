
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