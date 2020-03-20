use crate::cpu::CpuResult;
use crate::cpu::{Cpu, Memory};

pub struct Computer {
    pub memory: Memory,
    pub cpu: Cpu,
}

impl Computer {
    pub fn new(memory_size: usize) -> Computer {
        Computer {
            memory: Memory::new(memory_size),
            cpu: Cpu::new(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.memory);
    }
}
