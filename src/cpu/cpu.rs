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
        let insn = Insn::decode(memory, &mut self.program_counter)?;

        use super::Insn::*;
        match insn {
            Nop => {}
            Stack(op) => {
                use super::StackOp::*;
                match op {
                    Pop => {
                        self.pop();
                    }
                    Dup => {
                        let value = self.pop();
                        self.push2(value, value);
                    }
                }
            }
            PushValue(value) => self.push(value),
            UMath(op) => {
                use super::UMathOp::*;
                match op {
                    Add => {
                        let (a, b) = self.pop2();
                        self.push(a + b);
                    }
                    Sub => {
                        let (a, b) = self.pop2();
                        self.push(a - b);
                    }
                    Mul => {
                        let (a, b) = self.pop2();
                        self.push(a * b);
                    }
                    Div => {
                        let (a, b) = self.pop2();
                        self.push(a / b);
                    }
                    Rem => {
                        let (a, b) = self.pop2();
                        self.push(a % b);
                    }

                    LeftShift => {
                        let (a, b) = self.pop2();
                        self.push(a << b);
                    }
                    RightShift => {
                        let (a, b) = self.pop2();
                        self.push(a >> b);
                    }
                    BitNot => {
                        let value = self.pop();
                        self.push(!value);
                    }
                    BitAnd => {
                        let (a, b) = self.pop2();
                        self.push(a & b);
                    }
                    BitOr => {
                        let (a, b) = self.pop2();
                        self.push(a | b);
                    }
                    BitXor => {
                        let (a, b) = self.pop2();
                        self.push(a ^ b);
                    }
                }
            }
            IMath => {}
            FMath => {}

            Jump(dest) => self.program_counter.address = dest,
            UJump(op, dest) => {
                use super::UJumpOp::*;
                let test = match op {
                    Zero => self.pop() == 0,
                    NotZero => self.pop() != 0,
                    Equal => {
                        let (a, b) = self.pop2();
                        a == b
                    }
                    NotEqual => {
                        let (a, b) = self.pop2();
                        a != b
                    }
                    LessThan => {
                        let (a, b) = self.pop2();
                        a < b
                    }
                    GreaterThan => {
                        let (a, b) = self.pop2();
                        a > b
                    }
                    LessThanOrEqual => {
                        let (a, b) = self.pop2();
                        a <= b
                    }
                    GreaterThanOrEqual => {
                        let (a, b) = self.pop2();
                        a >= b
                    }
                };
                if test {
                    self.program_counter.address = dest;
                }
            }
            IJump(_) => {}
            FJump(_) => {}
        }

        Ok(())
    }

    fn pop(&mut self) -> u32 {
        self.stack.pop().expect("stack empty")
    }

    fn pop2(&mut self) -> (u32, u32) {
        let b = self.stack.pop().expect("stack empty");
        let a = self.stack.pop().expect("stack only had one element");
        (a, b)
    }

    #[allow(unused)]
    fn peek(&self) -> u32 {
        *self.stack.last().expect("stack empty")
    }

    #[allow(unused)]
    fn peek2(&self) -> (u32, u32) {
        (
            *self
                .stack
                .get(self.stack.len() - 2)
                .expect("stack only had one element"),
            *self.stack.get(self.stack.len() - 1).expect("stack empty"),
        )
    }

    fn push(&mut self, value: u32) {
        self.stack.push(value);
    }

    fn push2(&mut self, a: u32, b: u32) {
        self.stack.push(a);
        self.stack.push(b);
    }
}
