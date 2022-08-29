#![allow(unused, unused_mut, dead_code)]
use crate::extensions::{I64, I32, Ext};
use crate::encoding_types::{OpCode, Inst};

// S struct for fast OpCode lookups
#[derive(Clone, Debug)]
pub struct EncodingTable<E: Ext> {
    pub table: [OpCodeType; 128],
    ext: E,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpCodeType {
    R,
    I,
    S,
    B,
    U,
    J,
    Invalid,
}

impl<E: Ext> EncodingTable<E> {
    pub fn get(&self, opcode: OpCode) -> OpCodeType {
        self.table[opcode as usize]
    }
}

impl From<Inst> for OpCodeType {
    fn from(inst: Inst) -> OpCodeType {
        let opcode: u8 = (inst & 0b1111111) as u8;
        opcode.into()
    }
}

impl From<OpCode> for OpCodeType {
    fn from(opcode: OpCode) -> OpCodeType {
        let table = EncodingTable::<I64>::default();
        table.get(opcode)
    }
}

impl From<OpCodeType> for OpCode {
    fn from(oct: OpCodeType) -> u8 {
        match oct {
            OpCodeType::R => 0,
            OpCodeType::I => 1,
            OpCodeType::S => 2,
            OpCodeType::B => 3,
            OpCodeType::U => 4,
            OpCodeType::J => 5,
            OpCodeType::Invalid => 6,
        }
    }
}

impl Default for EncodingTable<I64> {
    fn default() -> EncodingTable<I64> {
        let table: [OpCodeType; 128] = I64_TABLE;
        EncodingTable {
            table,
            ext: I64
        }
    }
}

impl Default for EncodingTable<I32> {
    fn default() -> EncodingTable<I32> {
        let table: [OpCodeType; 128] = I32_TABLE;
        EncodingTable {
            table,
            ext: I32
        }
    }
}

// Full Encoding Table for I64
pub const I64_TABLE: [OpCodeType; 128] = [
    /*0b0000000*/ OpCodeType::Invalid,      // decimal: 0     hex: 0x00
    /*0b0000001*/ OpCodeType::Invalid,      // decimal: 1     hex: 0x01
    /*0b0000010*/ OpCodeType::Invalid,      // decimal: 2     hex: 0x02
    /*0b0000011*/ OpCodeType::I,            // decimal: 3     hex: 0x03
    /*0b0000100*/ OpCodeType::Invalid,      // decimal: 4     hex: 0x04
    /*0b0000101*/ OpCodeType::Invalid,      // decimal: 5     hex: 0x05
    /*0b0000110*/ OpCodeType::Invalid,      // decimal: 6     hex: 0x06
    /*0b0000111*/ OpCodeType::Invalid,      // decimal: 7     hex: 0x07
    /*0b0001000*/ OpCodeType::Invalid,      // decimal: 8     hex: 0x08
    /*0b0001001*/ OpCodeType::Invalid,      // decimal: 9     hex: 0x09
    /*0b0001010*/ OpCodeType::Invalid,      // decimal: 10    hex: 0x0a
    /*0b0001011*/ OpCodeType::Invalid,      // decimal: 11    hex: 0x0b
    /*0b0001100*/ OpCodeType::Invalid,      // decimal: 12    hex: 0x0c
    /*0b0001101*/ OpCodeType::Invalid,      // decimal: 13    hex: 0x0d
    /*0b0001110*/ OpCodeType::Invalid,      // decimal: 14    hex: 0x0e
    /*0b0001111*/ OpCodeType::R,            // decimal: 15    hex: 0x0f
    /*0b0010000*/ OpCodeType::Invalid,      // decimal: 16    hex: 0x10
    /*0b0010001*/ OpCodeType::Invalid,      // decimal: 17    hex: 0x11
    /*0b0010010*/ OpCodeType::Invalid,      // decimal: 18    hex: 0x12
    /*0b0010011*/ OpCodeType::I,            // decimal: 19    hex: 0x13
    /*0b0010100*/ OpCodeType::Invalid,      // decimal: 20    hex: 0x14
    /*0b0010101*/ OpCodeType::Invalid,      // decimal: 21    hex: 0x15
    /*0b0010110*/ OpCodeType::Invalid,      // decimal: 22    hex: 0x16
    /*0b0010111*/ OpCodeType::U,            // decimal: 23    hex: 0x17
    /*0b0011000*/ OpCodeType::Invalid,      // decimal: 24    hex: 0x18
    /*0b0011001*/ OpCodeType::Invalid,      // decimal: 25    hex: 0x19  
    /*0b0011010*/ OpCodeType::Invalid,      // decimal: 26    hex: 0x1a
    /*0b0011011*/ OpCodeType::Invalid,      // decimal: 27    hex: 0x1b
    /*0b0011100*/ OpCodeType::Invalid,      // decimal: 28    hex: 0x1c
    /*0b0011101*/ OpCodeType::Invalid,      // decimal: 29    hex: 0x1d
    /*0b0011110*/ OpCodeType::Invalid,      // decimal: 30    hex: 0x1e
    /*0b0011111*/ OpCodeType::Invalid,      // decimal: 31    hex: 0x1f
    /*0b0100000*/ OpCodeType::Invalid,      // decimal: 32    hex: 0x20
    /*0b0100001*/ OpCodeType::Invalid,      // decimal: 33    hex: 0x21
    /*0b0100010*/ OpCodeType::Invalid,      // decimal: 34    hex: 0x22
    /*0b0100011*/ OpCodeType::S,            // decimal: 35    hex: 0x23
    /*0b0100100*/ OpCodeType::Invalid,      // decimal: 36    hex: 0x24  
    /*0b0100101*/ OpCodeType::Invalid,      // decimal: 37    hex: 0x25
    /*0b0100110*/ OpCodeType::Invalid,      // decimal: 38    hex: 0x26
    /*0b0100111*/ OpCodeType::Invalid,      // decimal: 39    hex: 0x27
    /*0b0101000*/ OpCodeType::Invalid,      // decimal: 40    hex: 0x28
    /*0b0101001*/ OpCodeType::Invalid,      // decimal: 41    hex: 0x29
    /*0b0101010*/ OpCodeType::Invalid,      // decimal: 42    hex: 0x2a
    /*0b0101011*/ OpCodeType::Invalid,      // decimal: 43    hex: 0x2b
    /*0b0101100*/ OpCodeType::Invalid,      // decimal: 44    hex: 0x2c
    /*0b0101101*/ OpCodeType::Invalid,      // decimal: 45    hex: 0x2d
    /*0b0101110*/ OpCodeType::Invalid,      // decimal: 46    hex: 0x2e
    /*0b0101111*/ OpCodeType::Invalid,      // decimal: 47    hex: 0x2f
    /*0b0110000*/ OpCodeType::Invalid,      // decimal: 48    hex: 0x30
    /*0b0110001*/ OpCodeType::Invalid,      // decimal: 49    hex: 0x31
    /*0b0110010*/ OpCodeType::Invalid,      // decimal: 50    hex: 0x32
    /*0b0110011*/ OpCodeType::R,            // decimal: 51    hex: 0x33
    /*0b0110100*/ OpCodeType::Invalid,      // decimal: 52    hex: 0x34  
    /*0b0110101*/ OpCodeType::Invalid,      // decimal: 53    hex: 0x35
    /*0b0110110*/ OpCodeType::Invalid,      // decimal: 54    hex: 0x36
    /*0b0110111*/ OpCodeType::U,            // decimal: 55    hex: 0x37
    /*0b0111000*/ OpCodeType::Invalid,      // decimal: 56    hex: 0x38
    /*0b0111001*/ OpCodeType::Invalid,      // decimal: 57    hex: 0x39
    /*0b0111010*/ OpCodeType::Invalid,      // decimal: 58    hex: 0x3a
    /*0b0111011*/ OpCodeType::Invalid,      // decimal: 59    hex: 0x3b
    /*0b0111100*/ OpCodeType::Invalid,      // decimal: 60    hex: 0x3c
    /*0b0111101*/ OpCodeType::Invalid,      // decimal: 61    hex: 0x3d
    /*0b0111110*/ OpCodeType::Invalid,      // decimal: 62    hex: 0x3e
    /*0b0111111*/ OpCodeType::Invalid,      // decimal: 63    hex: 0x3f
    /*0b1000000*/ OpCodeType::Invalid,      // decimal: 64    hex: 0x40
    /*0b1000001*/ OpCodeType::Invalid,      // decimal: 65    hex: 0x41
    /*0b1000010*/ OpCodeType::Invalid,      // decimal: 66    hex: 0x42
    /*0b1000011*/ OpCodeType::Invalid,      // decimal: 67    hex: 0x43
    /*0b1000100*/ OpCodeType::Invalid,      // decimal: 68    hex: 0x44
    /*0b1000101*/ OpCodeType::Invalid,      // decimal: 69    hex: 0x45
    /*0b1000110*/ OpCodeType::Invalid,      // decimal: 70    hex: 0x46
    /*0b1000111*/ OpCodeType::Invalid,      // decimal: 71    hex: 0x47
    /*0b1001000*/ OpCodeType::Invalid,      // decimal: 72    hex: 0x48
    /*0b1001001*/ OpCodeType::Invalid,      // decimal: 73    hex: 0x49
    /*0b1001010*/ OpCodeType::Invalid,      // decimal: 74    hex: 0x4a
    /*0b1001011*/ OpCodeType::Invalid,      // decimal: 75    hex: 0x4b
    /*0b1001100*/ OpCodeType::Invalid,      // decimal: 76    hex: 0x4c
    /*0b1001101*/ OpCodeType::Invalid,      // decimal: 77    hex: 0x4d
    /*0b1001110*/ OpCodeType::Invalid,      // decimal: 78    hex: 0x4e
    /*0b1001111*/ OpCodeType::Invalid,      // decimal: 79    hex: 0x4f
    /*0b1010000*/ OpCodeType::Invalid,      // decimal: 80    hex: 0x50
    /*0b1010001*/ OpCodeType::Invalid,      // decimal: 81    hex: 0x51
    /*0b1010010*/ OpCodeType::Invalid,      // decimal: 82    hex: 0x52
    /*0b1010011*/ OpCodeType::Invalid,      // decimal: 83    hex: 0x53
    /*0b1010100*/ OpCodeType::Invalid,      // decimal: 84    hex: 0x54
    /*0b1010101*/ OpCodeType::Invalid,      // decimal: 85    hex: 0x55
    /*0b1010110*/ OpCodeType::Invalid,      // decimal: 86    hex: 0x56
    /*0b1010111*/ OpCodeType::Invalid,      // decimal: 87    hex: 0x57
    /*0b1011000*/ OpCodeType::Invalid,      // decimal: 88    hex: 0x58
    /*0b1011001*/ OpCodeType::Invalid,      // decimal: 89    hex: 0x59
    /*0b1011010*/ OpCodeType::Invalid,      // decimal: 90    hex: 0x5a
    /*0b1011011*/ OpCodeType::Invalid,      // decimal: 91    hex: 0x5b
    /*0b1011100*/ OpCodeType::Invalid,      // decimal: 92    hex: 0x5c
    /*0b1011101*/ OpCodeType::Invalid,      // decimal: 93    hex: 0x5d
    /*0b1011110*/ OpCodeType::Invalid,      // decimal: 94    hex: 0x5e
    /*0b1011111*/ OpCodeType::Invalid,      // decimal: 95    hex: 0x5f
    /*0b1100000*/ OpCodeType::Invalid,      // decimal: 96    hex: 0x60
    /*0b1100001*/ OpCodeType::Invalid,      // decimal: 97    hex: 0x61
    /*0b1100010*/ OpCodeType::Invalid,      // decimal: 98    hex: 0x62
    /*0b1100011*/ OpCodeType::B,            // decimal: 99    hex: 0x63
    /*0b1100100*/ OpCodeType::Invalid,      // decimal: 100   hex: 0x64
    /*0b1100101*/ OpCodeType::Invalid,      // decimal: 101   hex: 0x65
    /*0b1100110*/ OpCodeType::Invalid,      // decimal: 102   hex: 0x66
    /*0b1100111*/ OpCodeType::I,            // decimal: 103   hex: 0x67
    /*0b1101000*/ OpCodeType::Invalid,      // decimal: 104   hex: 0x68
    /*0b1101001*/ OpCodeType::Invalid,      // decimal: 105   hex: 0x69
    /*0b1101010*/ OpCodeType::Invalid,      // decimal: 106   hex: 0x6a
    /*0b1101011*/ OpCodeType::Invalid,      // decimal: 107   hex: 0x6b
    /*0b1101100*/ OpCodeType::Invalid,      // decimal: 108   hex: 0x6c
    /*0b1101101*/ OpCodeType::Invalid,      // decimal: 109   hex: 0x6d
    /*0b1101110*/ OpCodeType::Invalid,      // decimal: 110   hex: 0x6e
    /*0b1101111*/ OpCodeType::J,            // decimal: 111   hex: 0x6f
    /*0b1110000*/ OpCodeType::Invalid,      // decimal: 112   hex: 0x70
    /*0b1110001*/ OpCodeType::Invalid,      // decimal: 113   hex: 0x70
    /*0b1110010*/ OpCodeType::Invalid,      // decimal: 114   hex: 0x71
    /*0b1110011*/ OpCodeType::R,            // decimal: 115   hex: 0x72
    /*0b1110100*/ OpCodeType::Invalid,      // decimal: 116   hex: 0x73
    /*0b1110101*/ OpCodeType::Invalid,      // decimal: 117   hex: 0x74
    /*0b1110110*/ OpCodeType::Invalid,      // decimal: 118   hex: 0x75
    /*0b1110111*/ OpCodeType::Invalid,      // decimal: 119   hex: 0x76
    /*0b1111000*/ OpCodeType::Invalid,      // decimal: 120   hex: 0x77
    /*0b1111001*/ OpCodeType::Invalid,      // decimal: 121   hex: 0x78
    /*0b1111010*/ OpCodeType::Invalid,      // decimal: 122   hex: 0x79
    /*0b1111011*/ OpCodeType::Invalid,      // decimal: 123   hex: 0x7a
    /*0b1111100*/ OpCodeType::Invalid,      // decimal: 124   hex: 0x7b
    /*0b1111101*/ OpCodeType::Invalid,      // decimal: 125   hex: 0x7c
    /*0b1111110*/ OpCodeType::Invalid,      // decimal: 126   hex: 0x7d
    /*0b1111111*/ OpCodeType::Invalid       // decimal: 127   hex: 0x7e
];

pub const I32_TABLE: [OpCodeType; 128] = [
    /*0b0000000*/ OpCodeType::Invalid,
    /*0b0000001*/ OpCodeType::Invalid,
    /*0b0000010*/ OpCodeType::Invalid,
    /*0b0000011*/ OpCodeType::I,
    /*0b0000100*/ OpCodeType::Invalid,
    /*0b0000101*/ OpCodeType::Invalid,
    /*0b0000110*/ OpCodeType::Invalid,
    /*0b0000111*/ OpCodeType::Invalid,
    /*0b0001000*/ OpCodeType::Invalid,
    /*0b0001001*/ OpCodeType::Invalid,
    /*0b0001010*/ OpCodeType::Invalid,
    /*0b0001011*/ OpCodeType::Invalid,
    /*0b0001100*/ OpCodeType::Invalid,
    /*0b0001101*/ OpCodeType::Invalid,
    /*0b0001110*/ OpCodeType::Invalid,
    /*0b0001111*/ OpCodeType::R,
    /*0b0010000*/ OpCodeType::Invalid,
    /*0b0010001*/ OpCodeType::Invalid,
    /*0b0010010*/ OpCodeType::Invalid,
    /*0b0010011*/ OpCodeType::I,
    /*0b0010100*/ OpCodeType::Invalid,
    /*0b0010101*/ OpCodeType::Invalid,
    /*0b0010110*/ OpCodeType::Invalid,
    /*0b0010111*/ OpCodeType::U,
    /*0b0011000*/ OpCodeType::Invalid,
    /*0b0011001*/ OpCodeType::Invalid,
    /*0b0011010*/ OpCodeType::Invalid,
    /*0b0011011*/ OpCodeType::I,
    /*0b0011100*/ OpCodeType::Invalid,
    /*0b0011101*/ OpCodeType::Invalid,
    /*0b0011110*/ OpCodeType::Invalid,
    /*0b0011111*/ OpCodeType::Invalid,
    /*0b0100000*/ OpCodeType::Invalid,
    /*0b0100001*/ OpCodeType::Invalid,
    /*0b0100010*/ OpCodeType::Invalid,
    /*0b0100011*/ OpCodeType::S,
    /*0b0100100*/ OpCodeType::Invalid,
    /*0b0100101*/ OpCodeType::Invalid,
    /*0b0100110*/ OpCodeType::Invalid,
    /*0b0100111*/ OpCodeType::Invalid,
    /*0b0101000*/ OpCodeType::Invalid,
    /*0b0101001*/ OpCodeType::Invalid,
    /*0b0101010*/ OpCodeType::Invalid,
    /*0b0101011*/ OpCodeType::Invalid,
    /*0b0101100*/ OpCodeType::Invalid,
    /*0b0101101*/ OpCodeType::Invalid,
    /*0b0101110*/ OpCodeType::Invalid,
    /*0b0101111*/ OpCodeType::Invalid,
    /*0b0110000*/ OpCodeType::Invalid,
    /*0b0110001*/ OpCodeType::Invalid,
    /*0b0110010*/ OpCodeType::Invalid,
    /*0b0110011*/ OpCodeType::R,
    /*0b0110100*/ OpCodeType::Invalid,
    /*0b0110101*/ OpCodeType::Invalid,
    /*0b0110110*/ OpCodeType::Invalid,
    /*0b0110111*/ OpCodeType::U,
    /*0b0111000*/ OpCodeType::Invalid,
    /*0b0111001*/ OpCodeType::Invalid,
    /*0b0111010*/ OpCodeType::Invalid,
    /*0b0111011*/ OpCodeType::R,
    /*0b0111100*/ OpCodeType::Invalid,
    /*0b0111101*/ OpCodeType::Invalid,
    /*0b0111110*/ OpCodeType::Invalid,
    /*0b0111111*/ OpCodeType::Invalid,
    /*0b1000000*/ OpCodeType::Invalid,
    /*0b1000001*/ OpCodeType::Invalid,
    /*0b1000010*/ OpCodeType::Invalid,
    /*0b1000011*/ OpCodeType::Invalid,
    /*0b1000100*/ OpCodeType::Invalid,
    /*0b1000101*/ OpCodeType::Invalid,
    /*0b1000110*/ OpCodeType::Invalid,
    /*0b1000111*/ OpCodeType::Invalid,
    /*0b1001000*/ OpCodeType::Invalid,
    /*0b1001001*/ OpCodeType::Invalid,
    /*0b1001010*/ OpCodeType::Invalid,
    /*0b1001011*/ OpCodeType::Invalid,
    /*0b1001100*/ OpCodeType::Invalid,
    /*0b1001101*/ OpCodeType::Invalid,
    /*0b1001110*/ OpCodeType::Invalid,
    /*0b1001111*/ OpCodeType::Invalid,
    /*0b1010000*/ OpCodeType::Invalid,
    /*0b1010001*/ OpCodeType::Invalid,
    /*0b1010010*/ OpCodeType::Invalid,
    /*0b1010011*/ OpCodeType::Invalid,
    /*0b1010100*/ OpCodeType::Invalid,
    /*0b1010101*/ OpCodeType::Invalid,
    /*0b1010110*/ OpCodeType::Invalid,
    /*0b1010111*/ OpCodeType::Invalid,
    /*0b1011000*/ OpCodeType::Invalid,
    /*0b1011001*/ OpCodeType::Invalid,
    /*0b1011010*/ OpCodeType::Invalid,
    /*0b1011011*/ OpCodeType::Invalid,
    /*0b1011100*/ OpCodeType::Invalid,
    /*0b1011101*/ OpCodeType::Invalid,
    /*0b1011110*/ OpCodeType::Invalid,
    /*0b1011111*/ OpCodeType::Invalid,
    /*0b1100000*/ OpCodeType::Invalid,
    /*0b1100001*/ OpCodeType::Invalid,
    /*0b1100010*/ OpCodeType::Invalid,
    /*0b1100011*/ OpCodeType::B,
    /*0b1100100*/ OpCodeType::Invalid,
    /*0b1100101*/ OpCodeType::Invalid,
    /*0b1100110*/ OpCodeType::Invalid,
    /*0b1100111*/ OpCodeType::I,
    /*0b1101000*/ OpCodeType::Invalid,
    /*0b1101001*/ OpCodeType::Invalid,
    /*0b1101010*/ OpCodeType::Invalid,
    /*0b1101011*/ OpCodeType::Invalid,
    /*0b1101100*/ OpCodeType::Invalid,
    /*0b1101101*/ OpCodeType::Invalid,
    /*0b1101110*/ OpCodeType::Invalid,
    /*0b1101111*/ OpCodeType::J,
    /*0b1110000*/ OpCodeType::Invalid,
    /*0b1110001*/ OpCodeType::Invalid,
    /*0b1110010*/ OpCodeType::Invalid,
    /*0b1110011*/ OpCodeType::R,
    /*0b1110100*/ OpCodeType::Invalid,
    /*0b1110101*/ OpCodeType::Invalid,
    /*0b1110110*/ OpCodeType::Invalid,
    /*0b1110111*/ OpCodeType::Invalid,
    /*0b1111000*/ OpCodeType::Invalid,
    /*0b1111001*/ OpCodeType::Invalid,
    /*0b1111010*/ OpCodeType::Invalid,
    /*0b1111011*/ OpCodeType::Invalid,
    /*0b1111100*/ OpCodeType::Invalid,
    /*0b1111101*/ OpCodeType::Invalid,
    /*0b1111110*/ OpCodeType::Invalid,
    /*0b1111111*/ OpCodeType::Invalid
];
