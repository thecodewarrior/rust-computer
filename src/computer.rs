use crate::cpu::{Cpu, Memory};

pub struct Computer {
    memory: Memory,
    cpu: Cpu
}

impl Computer {
    pub fn new(memory_size: usize) -> Computer {
        Computer {
            memory: Memory::new(memory_size),
            cpu: Cpu::new()
        }
    }
}