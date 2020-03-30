use super::*;
use crate::cpu::CpuResult;
use rust_computer_macros::bits;

pub struct Cpu {
    pub frames: Vec<StackFrame>,
    pub program_counter: ProgramCounter,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            frames: vec![StackFrame::new(0)],
            program_counter: ProgramCounter::new(0),
        }
    }

    pub fn tick(&mut self, memory: &mut Memory) -> CpuResult<()> {
        match memory.read_byte(self.program_counter.advance())? {
            0b0000_0000 => {}
            0b0000_0001 => {
                let source = Location::decode(memory, &mut self.program_counter)?;
                let dest = Location::decode(memory, &mut self.program_counter)?;
                let value = self.get_value(source)?;
                self.set_value(dest, value)?;
            }
            _ => return Err(CpuPanic::new())
        }

        Ok(())
    }

    fn get_value(&self, location: Location) -> CpuResult<u32> {
        match location {
            Location::Immediate(value) => Ok(value),
            Location::Register(index) => if index < 16 { 
                Ok(self.frame()?.registers[index])
            } else {
                Err(CpuPanic::new())
            },
        }
    }

    fn set_value(&mut self, location: Location, value: u32) -> CpuResult<()> {
        match location {
            Location::Immediate(_) => Err(CpuPanic::new()),
            Location::Register(index) => if index < 16 {
                self.frame_mut()?.registers[index] = value;
                Ok(()) 
            } else { 
                Err(CpuPanic::new())
            },
        }
    }

    pub fn frame(&self) -> Result<&StackFrame, CpuPanic> {
        self.frames.last().ok_or_else(|| CpuPanic::new())
    }

    pub fn frame_mut(&mut self) -> Result<&mut StackFrame, CpuPanic> {
        self.frames.last_mut().ok_or_else(|| CpuPanic::new())
    }
}

enum Location {
    Immediate(u32),
    Register(usize)
}

impl Location {
    pub fn decode(memory: &Memory, pc: &mut ProgramCounter) -> CpuResult<Location> {
        // opcodes starting with a 1 use the last nibble as a parameter
        Ok(match memory.read_byte(pc.advance())? {
            it if bits!(it; "0xxx_xxxx") => Location::Immediate(it as u32 & 0b0111_1111),
            it if bits!(it; "110x_xxxx") => Location::Register(it as usize & 0b0001_1111),
            _ => return Err(CpuPanic::new())
        })
    }
}

pub struct StackFrame {
    pub registers: [u32; 16],
    pub vars: Vec<u32>,
}

impl StackFrame {
    fn new(size: u32) -> StackFrame {
        StackFrame {
            registers: [0; 16],
            vars: vec![0; size as usize],
        }
    }

    fn store(&mut self, var: u32, value: u32) {
        *self
            .vars
            .get_mut(var as usize)
            .expect("var is out of bounds") = value;
    }

    fn load(&self, var: u32) -> u32 {
        *self.vars.get(var as usize).expect("var is out of bounds")
    }
}
