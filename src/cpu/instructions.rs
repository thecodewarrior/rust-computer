use super::*;
use rust_computer_macros::bits;

pub enum Insn {
    Nop,
    Stack(StackOp),      // manipulate the stack (pop, dup, swap, etc.)
    UMath(UMathOp),      // perform unsigned integer math
    IMath,               // perform signed integer math - placeholder
    FMath,               // perform floating point math - placeholder
    Jump(u32),           // unconditional jump
    UJump(UJumpOp, u32), // jump based on an unsigned integer comparison
    IJump(u32),          // jump based on an signed integer comparison - placeholder
    FJump(u32),          // jump based on an floating point comparison - placeholder
}

pub enum StackOp {
    Pop,
    Dup,
    Swap,
    PushValue(u32),

    PushFrame(u32),
    PopFrame,
    Store(u32),
    Load(u32),
}

pub enum UMathOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    LeftShift,
    RightShift,
    BitNot,
    BitAnd,
    BitOr,
    BitXor,
}

pub enum UJumpOp {
    Zero,
    NotZero,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Insn {
    pub fn decode(memory: &Memory, pc: &mut ProgramCounter) -> CpuResult<Insn> {
        // opcodes starting with a 1 use the last nibble as a parameter
        Ok(match memory.read_byte(pc.advance())? {
            0u8 => Insn::Nop,
            1u8 => Insn::Stack(match memory.read_byte(pc.advance())? {
                0b0000 => StackOp::Pop,
                0b0001 => StackOp::Dup,
                0b0010 => StackOp::Swap,
                0b0011 => StackOp::PushValue(memory.read_word(pc.advance_n(4))?),
                0b0100 => StackOp::PushFrame(memory.read_word(pc.advance_n(4))?),
                0b0101 => StackOp::PopFrame,
                0b0110 => StackOp::Store(memory.read_word(pc.advance_n(4))?),
                0b0111 => StackOp::Load(memory.read_word(pc.advance_n(4))?),
                _ => return Err(CpuPanic::new()),
            }),
            4u8 => Insn::UMath(match memory.read_byte(pc.advance())? {
                0b0000 => UMathOp::Add,
                0b0001 => UMathOp::Sub,
                0b0010 => UMathOp::Mul,
                0b0011 => UMathOp::Div,
                0b0100 => UMathOp::Rem,
                0b0101 => UMathOp::LeftShift,
                0b0110 => UMathOp::RightShift,
                0b0111 => UMathOp::BitNot,
                0b1000 => UMathOp::BitAnd,
                0b1001 => UMathOp::BitOr,
                0b1010 => UMathOp::BitXor,
                _ => return Err(CpuPanic::new()),
            }),
            5u8 => Insn::IMath,
            6u8 => Insn::FMath,
            7u8 => Insn::Jump(memory.read_word(pc.advance_n(4))?),
            8u8 => Insn::UJump(
                match memory.read_byte(pc.advance())? {
                    0b0000 => UJumpOp::Zero,
                    0b0001 => UJumpOp::NotZero,
                    0b0010 => UJumpOp::Equal,
                    0b0011 => UJumpOp::NotEqual,
                    0b0100 => UJumpOp::LessThan,
                    0b0101 => UJumpOp::GreaterThan,
                    0b0110 => UJumpOp::LessThanOrEqual,
                    0b0111 => UJumpOp::GreaterThanOrEqual,
                    _ => return Err(CpuPanic::new()),
                },
                memory.read_word(pc.advance_n(4))?,
            ),
            9u8 => Insn::IJump(memory.read_word(pc.advance_n(4))?),
            10u8 => Insn::FJump(memory.read_word(pc.advance_n(4))?),
            _ => return Err(CpuPanic::new()),
        })
    }
}
