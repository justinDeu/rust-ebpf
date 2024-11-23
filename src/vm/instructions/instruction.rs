use super::arithmetic::{Arithmetic32Instruction, Arithmetic64Instruction};
use super::jump::{Jump32, Jump64};
use super::load::{LoadNonStandardInstruction, LoadRegisterInstruction};
use super::store::{StoreImmediateInstruction, StoreRegisterInstruction};
use crate::vm::vm::VirtualMachine;

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionClass {
    LoadNonStandard,
    LoadRegister,
    StoreImmediate,
    StoreRegister,
    Arithmetic32,
    Jump64,
    Jump32,
    Arithmetic64,
}
impl TryFrom<u8> for InstructionClass {
    type Error = String;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(InstructionClass::LoadNonStandard),
            1 => Ok(InstructionClass::LoadRegister),
            2 => Ok(InstructionClass::StoreImmediate),
            3 => Ok(InstructionClass::StoreRegister),
            4 => Ok(InstructionClass::Arithmetic32),
            5 => Ok(InstructionClass::Jump64),
            6 => Ok(InstructionClass::Jump32),
            7 => Ok(InstructionClass::Arithmetic64),
            _ => Err(format!("Invalid instruction class: {:#02x}", val)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    R0, // return value from function calls, and exit value for eBPF programs
    R1, // R1-R5: arguments for function calls
    R2,
    R3,
    R4,
    R5,
    R6, // R6-R9: callee saved registers that function calls will preserve
    R7,
    R8,
    R9,
    R10, // read-only frame pointer to access stack
}

impl TryFrom<u8> for Register {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            8 => Ok(Register::R8),
            9 => Ok(Register::R9),
            10 => Ok(Register::R10),
            _ => Err("Invalid register value"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub immediate: i32,
    pub offset: i16,
    pub src: Register,
    pub dst: Register,
    pub opcode: u8,
}

impl From<u64> for Instruction {
    fn from(bits: u64) -> Self {
        Self {
            immediate: (bits >> 32) as i32,
            offset: (bits >> 16) as i16,
            src: Register::try_from(((bits >> 12) & 0xf) as u8).expect("Invalid register value"),
            dst: Register::try_from(((bits >> 8) & 0xf) as u8).expect("Invalid register value"),
            opcode: bits as u8,
        }
    }
}

impl Instruction {
    pub fn class(&self) -> InstructionClass {
        InstructionClass::try_from(self.opcode & 0b11).expect("invalid instruction class")
    }
}

impl From<Instruction> for Box<dyn ExecutableInstruction> {
    fn from(value: Instruction) -> Self {
        match value.class() {
            InstructionClass::LoadNonStandard => Box::new(LoadNonStandardInstruction::from(value)),
            InstructionClass::LoadRegister => Box::new(LoadRegisterInstruction::from(value)),
            InstructionClass::StoreImmediate => Box::new(StoreImmediateInstruction::from(value)),
            InstructionClass::StoreRegister => Box::new(StoreRegisterInstruction::from(value)),
            InstructionClass::Arithmetic32 => Box::new(Arithmetic32Instruction::from(value)),
            InstructionClass::Jump64 => Box::new(Jump64::from(value)),
            InstructionClass::Jump32 => Box::new(Jump32::from(value)),
            InstructionClass::Arithmetic64 => Box::new(Arithmetic64Instruction::from(value)),
        }
    }
}

pub trait ExecutableInstruction {
    fn execute(&self, vm: &mut VirtualMachine);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u64() {
        let instruction = Instruction::from(0x10101010_0101_13_00);
        assert_eq!(instruction.immediate, 0x10101010);
        assert_eq!(instruction.offset, 0x0101);
        assert_eq!(instruction.src, Register::R1);
        assert_eq!(instruction.dst, Register::R3);
        assert_eq!(instruction.class(), InstructionClass::LoadNonStandard);
    }

    #[test]
    #[should_panic]
    fn panic_instruction_class() {
        let inst = Instruction::from(0x0000000000000008);
        inst.class();
    }
}
