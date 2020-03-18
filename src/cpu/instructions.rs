use crate::cpu::MemRead;
use crate::cpu::{CpuResult, CpuPanic};

pub enum Register {
    A, B
}

pub enum Insn {
    Nop,
    LoadConstant { register: Register, value: u32 }
}

impl Insn {
    pub fn decode(memory: &dyn MemRead, address: &mut u32) -> CpuResult<Insn> {
        Ok(match memory.read_byte(*address)? {
            0 if address < &mut 100 => Insn::Nop,
            _ => return Err(CpuPanic::new())
        })
    }
}

