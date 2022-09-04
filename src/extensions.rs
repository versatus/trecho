#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Base {
    I32,
    I64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Extension {
    I32,
    I64,
    M32,
    M64,
    A32,
    A64,
    F32,
    F64,
    D32,
    D64,
    G32,
    G64
}

impl From<Base> for &'static str {
    fn from(base: Base) -> &'static str {
        match base {
            Base::I32 => return "I32",
            Base::I64 => return "I64"
        }
    }
}

impl From<Extension> for &'static str {
    fn from(ext: Extension) -> &'static str {
        match ext {
            Extension::I32 => return "I32",
            Extension::I64 => return "I64",
            Extension::M32 => return "M32",
            Extension::M64 => return "M64",
            Extension::A32 => return "A32",
            Extension::A64 => return "A64",
            Extension::F32 => return "F32",
            Extension::F64 => return "F64",
            Extension::D32 => return "D32",
            Extension::D64 => return "D64",
            Extension::G32 => return "G32",
            Extension::G64 => return "G64"
        }
    }
}

impl From<&Extension> for &'static str {
    fn from(ext: &Extension) -> &'static str {
        match ext {
            Extension::I32 => return "I32",
            Extension::I64 => return "I64",
            Extension::M32 => return "M32",
            Extension::M64 => return "M64",
            Extension::A32 => return "A32",
            Extension::A64 => return "A64",
            Extension::F32 => return "F32",
            Extension::F64 => return "F64",
            Extension::D32 => return "D32",
            Extension::D64 => return "D64",
            Extension::G32 => return "G32",
            Extension::G64 => return "G64"
        }
    }
}

impl Extension {
    pub fn into_str(&self) -> &'static str {
        self.into()
    }
}