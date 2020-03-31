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
            op if bits!(op; "0000_0001") => { // move
                let source = Location::decode(memory, &mut self.program_counter)?;
                let dest = Location::decode(memory, &mut self.program_counter)?;
                let value = self.get_value(memory, source)?;
                self.set_value(memory, dest, value)?;
            }
            op if bits!(op; "0000_1xxx") => { // unsigned math
                let a = Location::decode(memory, &mut self.program_counter)?;
                let b = Location::decode(memory, &mut self.program_counter)?;
                let dest = Location::decode(memory, &mut self.program_counter)?;
                let value_a = Wrapping(self.get_value(memory, a)?);
                let value_b = Wrapping(self.get_value(memory, b)?);

                let result = match op & 0b111 {
                    0b000 => value_a + value_b,
                    0b001 => value_a - value_b,
                    0b010 => value_a * value_b,
                    0b011 => value_a / value_b,
                    _ => unreachable!()
                };
                self.set_value(memory, dest, result.0)?;
            }
            _ => return Err(CpuPanic::new())
        }

        Ok(())
    }

    fn get_value(&self, memory: &Memory, location: Location) -> CpuResult<u32> {
        match location {
            Location::Immediate(value) => Ok(value),
            Location::Direct(direct) => self.get_direct(direct),
            Location::Indirect(direct, width) => {
                let address = self.get_direct(direct)?;
                memory.read_width(width, address)
            },
        }
    }

    fn get_direct(&self, location: DirectAddress) -> CpuResult<u32> {
        match location {
            DirectAddress::Register(index) => if index < 16 { 
                Ok(self.frame()?.registers[index])
            } else {
                Err(CpuPanic::new())
            },
            DirectAddress::Frame(index) => if index < self.frame()?.vars.len() { 
                Ok(self.frame()?.vars[index])
            } else {
                Err(CpuPanic::new())
            },
        }
    }

    fn set_value(&mut self, memory: &mut Memory, location: Location, value: u32) -> CpuResult<()> {
        match location {
            Location::Immediate(_) => Err(CpuPanic::new()),
            Location::Direct(direct) => self.set_direct(direct, value),
            Location::Indirect(direct, width) => {
                let address = self.get_direct(direct)?;
                memory.write_width(width, address, value)
            },
        }
    }

    fn set_direct(&mut self, location: DirectAddress, value: u32) -> CpuResult<()> {
        match location {
            DirectAddress::Register(index) => if index < 16 { 
                self.frame_mut()?.registers[index] = value;
                Ok(())
            } else {
                Err(CpuPanic::new())
            },
            DirectAddress::Frame(index) => if index < self.frame()?.vars.len() { 
                self.frame_mut()?.vars[index] = value;
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
    Direct(DirectAddress),
    Indirect(DirectAddress, DataWidth),
}

enum DirectAddress {
    Register(usize),
    Frame(usize),
}

impl Location {
    pub fn decode(memory: &Memory, pc: &mut ProgramCounter) -> CpuResult<Location> {
        // opcodes starting with a 1 use the last nibble as a parameter
        return Ok(match memory.read_byte(pc.advance())? {
            it if bits!(it; "0xxx_xxxx") => Location::Immediate(it as u32 & 0b0111_1111),
            it if bits!(it; "1100_xxxx") => Location::Direct(DirectAddress::Register(it as usize & 0b0000_1111)),

            it if bits!(it; "1000_00xx") => {
                let width = DataWidth::decode(it);
                Location::Immediate(memory.read_width(width, pc.advance_n(width.size()))?)
            },
            it if bits!(it; "1000_01xx") => {
                let width = DataWidth::decode(it);
                Location::Direct(DirectAddress::Frame(
                    memory.read_width(width, pc.advance_n(width.size()))? as usize
                ))
            },

            it if bits!(it; "1000_1xxx") => {
                let width = DataWidth::decode(it);
                Location::Indirect(read_direct(memory, pc,it)?, width)
            },
            _ => return Err(CpuPanic::new())
        });

        fn read_direct(memory: &Memory, pc: &mut ProgramCounter, location: u8) -> CpuResult<DirectAddress> {
            if location & 0b0000_0100 == 0 {
                Ok(DirectAddress::Register(
                    memory.read_byte(pc.advance())? as usize
                ))
            } else {
                Ok(DirectAddress::Frame(
                    memory.read_word(pc.advance_n(4))? as usize
                ))
            }
        }
    }
}

impl DataWidth {
    /// Gets the data width based on the last two bits in the passed byte
    pub fn decode(opcode: u8) -> DataWidth {
        match opcode & 0b0000_0011 {
            0b00 => DataWidth::Byte,
            0b01 => DataWidth::Short,
            0b10 => DataWidth::Word,
            0b11 => DataWidth::Word,
            _ => unreachable!()
        }
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
