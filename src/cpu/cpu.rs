use super::*;
use crate::cpu::CpuResult;

pub struct Cpu {
    pub stack: Vec<u32>,
    pub frames: Vec<Vec<u32>>,
    pub program_counter: ProgramCounter,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            stack: Vec::new(),
            frames: Vec::new(),
            program_counter: ProgramCounter::new(0),
        }
    }

    pub fn tick(&mut self, memory: &mut Memory) -> CpuResult<()> {
        // store the address from before decoding increments it.
        let insn_address = self.program_counter.address;
        let insn = Insn::decode(memory, &mut self.program_counter)?;

        use super::Insn::*;
        match insn {
            Nop => {}
            Pop => { self.pop(); },
            Dup => { 
                let value = self.pop(); 
                self.push(value);
                self.push(value);
            },
            PushValue(value) => self.push(value),
            UAdd => {
                let b = self.pop();
                let a = self.pop();
                self.push(a + b);
            },
            USub => {
                let b = self.pop();
                let a = self.pop();
                self.push(a - b);
            },
            UMul => {
                let b = self.pop();
                let a = self.pop();
                self.push(a * b);
            },
            UDiv => {
                let b = self.pop();
                let a = self.pop();
                self.push(a / b);
            },
            URem => {
                let b = self.pop();
                let a = self.pop();
                self.push(a % b);
            },
            Jump(dest) => self.program_counter.address = dest,
            JumpEqualZero(dest) => {
                if self.pop() == 0 {
                    self.program_counter.address = dest;
                }
            },
            JumpNotZero(dest) => {
                if self.pop() != 0 {
                    self.program_counter.address = dest;
                }
            },
        }

        Ok(())
    }

    fn pop(&mut self) -> u32 {
        self.stack.pop().expect("stack empty")
    }

    fn push(&mut self, value: u32) {
        self.stack.push(value);
    }

}
