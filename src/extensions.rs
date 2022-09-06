#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Base {
    I32,
    I64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Extension {
    I,
    M,
    A,
    F,
    D,
    G,
}

impl From<Base> for &'static str {
    fn from(base: Base) -> &'static str {
        match base {
            Base::I32 => return "I32",
            Base::I64 => return "I64"
        }
    }
}

impl From<&Base> for &'static str {
    fn from(base: &Base) -> &'static str {
        match base {
            Base::I32 => return "I32",
            Base::I64 => return "I64"
        }
    }
}

impl From<Extension> for &'static str {
    fn from(ext: Extension) -> &'static str {
        match ext {
            Extension::I => return "I",
            Extension::M => return "M",
            Extension::A => return "A",
            Extension::F => return "F",
            Extension::D => return "D",
            Extension::G => return "G",
        }
    }
}

impl From<&Extension> for &'static str {
    fn from(ext: &Extension) -> &'static str {
        match ext {
            Extension::I => return "I",
            Extension::M => return "M",
            Extension::A => return "A",
            Extension::F => return "F",
            Extension::D => return "D",
            Extension::G => return "G"
        }
    }
}

impl From<&'static str> for Base {
    fn from(input: &'static str) -> Base {
        match input {
            "32" => Base::I32,
            "64" => Base::I64,
            _ => Base::I32
        }
    }
}

impl From<&'static str> for Extension {
    fn from(input: &'static str) -> Extension {
        match input {
            "I" => return Extension::I,
            "M" => return Extension::M,
            "A" => return Extension::A,
            "F" => return Extension::F,
            "D" => return Extension::D,
            "G" => return Extension::G,
            _ => return Extension::I
        }
    }
}

impl Extension {
    pub fn into_str(&self) -> &'static str {
        self.into()
    }
}

impl Base {
    pub fn into_str(&self) -> &'static str {
        self.into()
    }
}