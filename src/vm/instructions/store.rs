use crate::vm::vm::VirtualMachine;

use super::instruction::{ExecutableInstruction, Instruction};

pub struct StoreImmediateInstruction {
    instruction: Instruction,
}

trait StoreInstruction {}

impl From<Instruction> for StoreImmediateInstruction {
    fn from(value: Instruction) -> Self {
        Self { instruction: value }
    }
}

impl ExecutableInstruction for StoreImmediateInstruction {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing StoreImmediateInstruction!")
    }
}

impl StoreImmediateInstruction {
    fn mode(&self) -> u8 {
        (self.instruction.opcode >> 5) & 0b111
    }

    fn size(&self) -> u8 {
        (self.instruction.opcode >> 3) & 0b11
    }

    fn class(&self) -> u8 {
        (self.instruction.opcode) & 0b11
    }
}

pub struct StoreRegisterInstruction {
    instruction: Instruction,
}

impl From<Instruction> for StoreRegisterInstruction {
    fn from(value: Instruction) -> Self {
        Self { instruction: value }
    }
}

impl ExecutableInstruction for StoreRegisterInstruction {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing StoreRegisterInstruction!")
    }
}
impl StoreRegisterInstruction {
    fn mode(&self) -> u8 {
        (self.instruction.opcode >> 5) & 0b111
    }

    fn size(&self) -> u8 {
        (self.instruction.opcode >> 3) & 0b11
    }

    fn class(&self) -> u8 {
        (self.instruction.opcode) & 0b11
    }
}
