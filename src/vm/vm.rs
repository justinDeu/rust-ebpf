use core::panic;

use super::enums::{BPFLoadStoreMode, BPFSizeModifier};
use super::instructions::instruction::Instruction;
use super::instructions::instruction::InstructionClass;
use super::stack::Stack;

#[derive(Default)]
pub struct VirtualMachine {
    pub registers: [u64; 11],
    pub instruction_pointer: u64,
    pub stack: Stack,
}

impl VirtualMachine {
    pub fn run(&mut self, instructions: &[u64]) {
        self.reset();

        instructions
            .iter()
            .for_each(|x| self.execute_instruction(Instruction::from(*x)));
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.class() {
            InstructionClass::Jump32 => self.handle_jump32(instruction),
            InstructionClass::LoadNonStandard => self.handle_loadnonstandard(instruction),
            InstructionClass::LoadRegister => self.handle_loadregister(instruction),
            InstructionClass::StoreImmediate => self.handle_storeimmediate(instruction),
            InstructionClass::StoreRegister => self.handle_storeregister(instruction),
            InstructionClass::Arithmetic32 => self.handle_arithmetic32(instruction),
            InstructionClass::Jump64 => self.handle_jump64(instruction),
            InstructionClass::Arithmetic64 => self.handle_arithmetic64(instruction),
        }
    }

    fn handle_jump32(&mut self, instruction: Instruction) {
        println!("Handle Jump32")
    }
    fn handle_jump64(&mut self, instruction: Instruction) {
        println!("Handle Jump64")
    }
    fn handle_loadregister(&mut self, instruction: Instruction) {
        let load_mode = BPFLoadStoreMode::from(instruction.opcode >> 5);
        let size = BPFSizeModifier::from((instruction.opcode >> 3) & 0b11);

        match load_mode {
            BPFLoadStoreMode::BPF_ABS | BPFLoadStoreMode::BPF_IND => {
                panic!("Deprecated BPF packet loading disabled")
            }
            BPFLoadStoreMode::BPF_IMM => {
                println!("Instruction.dst: {:#02x}", instruction.dst as usize);
                println!("Instruction.immediate: {:#04x}", instruction.immediate);
                println!(
                    "Instruction.immediate cast: {:#08x}",
                    instruction.immediate as u64 & ((1 << size.size()) - 1)
                );
                println!("Instruction.reg[2] pre: {:#08x}", self.registers[2]);
                self.registers[instruction.dst as usize] =
                    instruction.immediate as u64 & ((1 << size.size()) - 1);
                println!("Instruction.reg[2] post: {:#08x}", self.registers[2]);
            }
            BPFLoadStoreMode::BPF_MEM => todo!(),
            BPFLoadStoreMode::BPF_ATOMIC => todo!(),
        }
    }
    fn handle_loadnonstandard(&mut self, instruction: Instruction) {
        let load_mode = BPFLoadStoreMode::from(instruction.opcode >> 5);
        let size = BPFSizeModifier::from(instruction.opcode & 0x18);

        if load_mode != BPFLoadStoreMode::BPF_IMM {
            panic!("Invalid Load Mode, expected BPF_IMM")
        }

        if size != BPFSizeModifier::BPF_DW {
            panic!("Invalid Size Modifier for BPF_LD, expected BPF_IMM")
        }

        //self.registers[instruction.dst as usize] = instruction.immediate
        todo!("Handle LoadNonStandard")
    }
    fn handle_storeimmediate(&mut self, instruction: Instruction) {
        println!("Handle StoreImmediate")
    }
    fn handle_storeregister(&mut self, instruction: Instruction) {
        println!("Handle StoreRegister")
    }
    fn handle_arithmetic32(&mut self, instruction: Instruction) {
        println!("Handle Arithmetic32");
    }
    fn handle_arithmetic64(&mut self, instruction: Instruction) {
        println!("Handle Arithmetic64")
    }

    fn reset(&mut self) {
        self.registers = [0u64; 11];
        self.stack = Stack::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_register_imm() {
        let mut vm = VirtualMachine::default();

        vm.run(&[0xFF_00_FF_00_00_00_02_19]);

        assert_eq!(vm.registers[2], 0xFF00FF00_u64)
    }
}
