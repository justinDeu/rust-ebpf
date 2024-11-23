use crate::vm::vm::VirtualMachine;

use super::instruction::{ExecutableInstruction, Instruction};

pub struct Arithmetic32Instruction {
    instruction: Instruction,
}

impl From<Instruction> for Arithmetic32Instruction {
    fn from(value: Instruction) -> Self {
        Self { instruction: value }
    }
}

impl ExecutableInstruction for Arithmetic32Instruction {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing Arithmetic32Instruction!")
    }
}

pub struct Arithmetic64Instruction {
    instruction: Instruction,
}

impl From<Instruction> for Arithmetic64Instruction {
    fn from(value: Instruction) -> Self {
        Self { instruction: value }
    }
}

impl ExecutableInstruction for Arithmetic64Instruction {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing Arithmetic64Instruction!")
    }
}
