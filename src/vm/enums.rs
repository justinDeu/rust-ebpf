#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum BPFLoadStoreMode {
    BPF_IMM,
    BPF_ABS,
    BPF_IND,
    BPF_MEM,
    BPF_ATOMIC,
}

impl From<u8> for BPFLoadStoreMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::BPF_IMM,
            0x20 => Self::BPF_ABS,
            0x40 => Self::BPF_IND,
            0x60 => Self::BPF_MEM,
            0xc0 => Self::BPF_ATOMIC,
            _ => panic!("Invalid BPFLoadStoreMode: 0x{:02x}", value),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum BPFSizeModifier {
    BPF_W,
    BPF_H,
    BPF_B,
    BPF_DW,
}

impl From<u8> for BPFSizeModifier {
    fn from(value: u8) -> Self {
        match value & 0x18 {
            0x00 => Self::BPF_W,
            0x08 => Self::BPF_H,
            0x10 => Self::BPF_B,
            0x18 => Self::BPF_DW,
            _ => panic!("Invalid BPFSizeModifer: 0x{:02x}", value),
        }
    }
}

impl BPFSizeModifier {
    pub fn size(&self) -> usize {
        match self {
            BPFSizeModifier::BPF_W => 4,
            BPFSizeModifier::BPF_H => 2,
            BPFSizeModifier::BPF_B => 1,
            BPFSizeModifier::BPF_DW => 8,
        }
    }
}

#[allow(non_camel_case_types)]
enum BPFAtomicOp {
    BPF_ADD,
    BPF_OR,
    BPF_AND,
    BPF_XOR,
}

impl From<u8> for BPFAtomicOp {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::BPF_ADD,
            0x40 => Self::BPF_OR,
            0x50 => Self::BPF_AND,
            0xa0 => Self::BPF_XOR,
            _ => panic!("Invalid BPFAtomicOp: 0x{:02x}", value),
        }
    }
}

#[allow(non_camel_case_types)]
enum BPFJumpCode {
    BPF_JA,
    BPF_JEQ,
    BPF_JGT,
    BPF_JGE,
    BPF_JSET,
    BPF_JNE,
    BPF_JSGT,
    BPF_JSGE,
    BPF_CALL,
    BPF_EXIT,
    BPF_JLT,
    BPF_JLE,
    BPF_JSLT,
    BPF_JSLE,
}

impl From<u8> for BPFJumpCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::BPF_JA,
            0x10 => Self::BPF_JEQ,
            0x20 => Self::BPF_JGT,
            0x30 => Self::BPF_JGE,
            0x40 => Self::BPF_JSET,
            0x50 => Self::BPF_JNE,
            0x60 => Self::BPF_JSGT,
            0x70 => Self::BPF_JSGE,
            0x80 => Self::BPF_CALL,
            0x90 => Self::BPF_EXIT,
            0xa0 => Self::BPF_JLT,
            0xb0 => Self::BPF_JLE,
            0xc0 => Self::BPF_JSLT,
            0xd0 => Self::BPF_JSLE,
            _ => panic!("Invalid BPFCode: 0x{:02x}", value),
        }
    }
}

#[allow(non_camel_case_types)]
enum BPFByteSwap {
    BPF_TO_LE,
    BPF_TO_BE,
}

impl From<u8> for BPFByteSwap {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::BPF_TO_LE,
            0x08 => Self::BPF_TO_BE,
            _ => panic!("Invalid BPFByteSwap: 0x{:02x}", value),
        }
    }
}
