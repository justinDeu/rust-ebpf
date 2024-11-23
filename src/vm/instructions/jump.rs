use crate::vm::vm::VirtualMachine;

use super::instruction::{ExecutableInstruction, Instruction};

pub struct Jump32 {
    instruction: Instruction,
}

impl From<Instruction> for Jump32 {
    fn from(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl ExecutableInstruction for Jump32 {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing Jump32!")
    }
}

pub struct Jump64 {
    instruction: Instruction,
}

impl From<Instruction> for Jump64 {
    fn from(instruction: Instruction) -> Self {
        Self { instruction }
    }
}

impl ExecutableInstruction for Jump64 {
    fn execute(&self, vm: &mut VirtualMachine) {
        println!("Executing Jump64!")
    }
}
