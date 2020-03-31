use super::*;
use crate::cpu::CpuResult;
use rust_computer_macros::bits;
use std::num::Wrapping;

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
            op if bits!(op; "0000_01xx") => { // move
                let width = DataWidth::decode(op);
                let source = Location::decode(memory, &mut self.program_counter)?;
                let dest = Location::decode(memory, &mut self.program_counter)?;
                let value = self.get_value(source, width)?;
                self.set_value(dest, width, value)?;
            }
            op if bits!(op; "0001_xxxx") => { // unsigned math
                let width = DataWidth::decode(op);
                let a = Location::decode(memory, &mut self.program_counter)?;
                let b = Location::decode(memory, &mut self.program_counter)?;
                let dest = Location::decode(memory, &mut self.program_counter)?;
                let value_a = Wrapping(self.get_value(a, width)?);
                let value_b = Wrapping(self.get_value(b, width)?);

                let result = match op >> 2 & 0b11 {
                    0b00 => value_a + value_b,
                    0b01 => value_a - value_b,
                    0b10 => value_a * value_b,
                    0b11 => value_a / value_b,
                    _ => unreachable!()
                };
                self.set_value(dest, width, result.0)?;
            }
            _ => return Err(CpuPanic::new())
        }

        Ok(())
    }

    fn get_value(&self, location: Location, width: DataWidth) -> CpuResult<u32> {
        match location {
            Location::Immediate(value) => Ok(value),
            Location::Register(index) => if index < 16 { 
                Ok(self.frame()?.registers[index])
            } else {
                Err(CpuPanic::new())
            },
        }.map(|v| v & width.bitmask())
    }

    fn set_value(&mut self, location: Location, width: DataWidth, value: u32) -> CpuResult<()> {
        let value = value & width.bitmask();
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

#[derive(Clone, Copy)]
enum DataWidth {
    Byte, Short, Word
}

impl DataWidth {
    /// Gets the data width based on the last two bits in the passed byte
    fn decode(opcode: u8) -> DataWidth {
        match opcode & 0b0000_0011 {
            0b00 => DataWidth::Byte,
            0b01 => DataWidth::Short,
            0b10 => DataWidth::Word,
            0b11 => DataWidth::Word,
            _ => unreachable!()
        }
    }

    fn bitmask(&self) -> u32 {
        match self {
            DataWidth::Byte => 0xff,
            DataWidth::Short => 0xffff,
            DataWidth::Word => 0xffff_ffff,
        } 
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

            it if bits!(it; "1000_0000") => Location::Immediate(memory.read_word(pc.advance_n(4))?),
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
