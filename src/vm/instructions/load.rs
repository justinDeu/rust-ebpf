use crate::vm::vm::VirtualMachine;

use super::instruction::{ExecutableInstruction, Instruction};

pub struct LoadRegisterInstruction {
    instruction: Instruction,
}

impl From<Instruction> for LoadRegisterInstruction {
    fn from(value: Instruction) -> Self {
        Self { instruction: value }
    }
}

impl ExecutableInstruction for LoadRegisterInstruction {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing LoadRegisterInstruction!")
    }
}

pub struct LoadNonStandardInstruction {
    instruction: Instruction,
}

impl From<Instruction> for LoadNonStandardInstruction {
    fn from(value: Instruction) -> Self {
        Self { instruction: value }
    }
}

impl ExecutableInstruction for LoadNonStandardInstruction {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing LoadNonStandardInstruction!")
    }
}
