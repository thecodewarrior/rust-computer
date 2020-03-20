use super::*;
use crate::cpu::CpuResult;

pub struct Cpu {
    pub frames: Vec<StackFrame>,
    pub program_counter: ProgramCounter,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
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
                        self.frame().pop();
                    }
                    Dup => {
                        let value = self.frame().pop();
                        self.frame().push2(value, value);
                    }
                    Swap => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push2(b, a);
                    }
                    PushValue(value) => {
                        self.frame().push(value);
                    }
                    PushFrame(size) => {
                        self.frames.push(StackFrame::new(size));
                    }
                    PopFrame => {
                        self.frames.pop().expect("no frames");
                    }
                    Store(var) => {
                        let value = self.frame().pop();
                        self.frame().store(var, value);
                    }
                    Load(var) => {
                        let value = self.frame().load(var);
                        self.frame().push(value);
                    }
                }
            }

            UMath(op) => {
                use super::UMathOp::*;
                match op {
                    Add => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a + b);
                    }
                    Sub => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a - b);
                    }
                    Mul => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a * b);
                    }
                    Div => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a / b);
                    }
                    Rem => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a % b);
                    }
                    LeftShift => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a << b);
                    }
                    RightShift => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a >> b);
                    }
                    BitNot => {
                        let value = self.frame().pop();
                        self.frame().push(!value);
                    }
                    BitAnd => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a & b);
                    }
                    BitOr => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a | b);
                    }
                    BitXor => {
                        let (a, b) = self.frame().pop2();
                        self.frame().push(a ^ b);
                    }
                }
            }
            IMath => {}
            FMath => {}

            Jump(dest) => self.program_counter.address = dest,
            UJump(op, dest) => {
                use super::UJumpOp::*;
                let test = match op {
                    Zero => self.frame().pop() == 0,
                    NotZero => self.frame().pop() != 0,
                    Equal => {
                        let (a, b) = self.frame().pop2();
                        a == b
                    }
                    NotEqual => {
                        let (a, b) = self.frame().pop2();
                        a != b
                    }
                    LessThan => {
                        let (a, b) = self.frame().pop2();
                        a < b
                    }
                    GreaterThan => {
                        let (a, b) = self.frame().pop2();
                        a > b
                    }
                    LessThanOrEqual => {
                        let (a, b) = self.frame().pop2();
                        a <= b
                    }
                    GreaterThanOrEqual => {
                        let (a, b) = self.frame().pop2();
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

    pub fn frame(&mut self) -> &mut StackFrame {
        self.frames.last_mut().expect("no frames")
    }

}


pub struct StackFrame {
    pub stack: Vec<u32>,
    pub vars: Vec<u32>
}

impl StackFrame {
    fn new(size: u32) -> StackFrame {
        StackFrame {
            stack: vec![],
            vars: vec![0; size as usize]
        }
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

    fn store(&mut self, var: u32, value: u32) {
        *self.vars.get_mut(var as usize).expect("var is out of bounds") = value;
    }

    fn load(&self, var: u32) -> u32 {
        *self.vars.get(var as usize).expect("var is out of bounds")
    }
}
