use core::panic;

use super::enums::BPFSizeModifier;

const STACK_SIZE: usize = 512;

pub struct Stack {
    data: [u8; STACK_SIZE],
}

impl Default for Stack {
    fn default() -> Self {
        Stack { data: [0u8; 512] }
    }
}

impl Stack {
    fn set(&mut self, addr: usize, size: BPFSizeModifier, value: u64) {
        self.size_check(addr, &size);

        let uaddr = addr as usize;
        match size {
            BPFSizeModifier::BPF_W => {
                self.data[uaddr..uaddr + size.size()].copy_from_slice(&(value as u32).to_le_bytes())
            }
            BPFSizeModifier::BPF_H => {
                self.data[uaddr..uaddr + size.size()].copy_from_slice(&(value as u16).to_le_bytes())
            }
            BPFSizeModifier::BPF_B => {
                self.data[uaddr..uaddr + size.size()].copy_from_slice(&(value as u8).to_le_bytes())
            }
            BPFSizeModifier::BPF_DW => {
                self.data[uaddr..uaddr + size.size()].copy_from_slice(&value.to_le_bytes())
            }
        }
    }
    fn get(&self, addr: usize, size: BPFSizeModifier) -> Vec<u8> {
        self.size_check(addr, &size);

        self.data[addr..addr + size.size()].to_vec()
    }

    fn size_check(&self, addr: usize, size: &BPFSizeModifier) {
        if addr + size.size() >= STACK_SIZE {
            panic!("Access Violation: 0x{:04x}", addr + size.size())
        }
    }

    pub fn get_byte(&self, addr: usize) -> u8 {
        self.get(addr, BPFSizeModifier::BPF_B)[0]
    }

    pub fn get_word(&self, addr: usize) -> u32 {
        let data = self.get(addr, BPFSizeModifier::BPF_B);

        assert!(data.len() == 4, "Vec must have exactly 4 bytes for u32");
        u32::from_le_bytes([data[0], data[1], data[2], data[3]])
    }

    pub fn get_hword(&self, addr: usize) -> u16 {
        let data = self.get(addr, BPFSizeModifier::BPF_H);

        assert!(data.len() == 2, "Vec must have exactly 2 bytes for u16");
        u16::from_le_bytes([data[0], data[1]])
    }

    pub fn get_dword(&self, addr: usize) -> u64 {
        let data = self.get(addr, BPFSizeModifier::BPF_B);

        assert!(data.len() == 8, "Vec must have exactly 8 bytes for u64");
        u64::from_le_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ])
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.set(addr, BPFSizeModifier::BPF_B, value.into());
    }

    pub fn set_word(&mut self, addr: usize, value: u32) {
        self.set(addr, BPFSizeModifier::BPF_W, value.into());
    }

    pub fn set_hword(&mut self, addr: usize, value: u16) {
        self.set(addr, BPFSizeModifier::BPF_H, value.into());
    }

    pub fn set_dword(&mut self, addr: usize, value: u64) {
        self.set(addr, BPFSizeModifier::BPF_DW, value);
    }
}
