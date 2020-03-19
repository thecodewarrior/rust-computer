use super::*;

pub enum Insn {
    Nop,
    Pop,
    Dup,
    PushValue(u32),
    UAdd,
    USub,
    UMul,
    UDiv,
    URem,
    Jump(u32),
    JumpEqualZero(u32),
    JumpNotZero(u32),
}

impl Insn {
    pub fn decode(memory: &Memory, pc: &mut ProgramCounter) -> CpuResult<Insn> {
        Ok(match memory.read_byte(pc.advance())? {
            0 => Insn::Nop,
            1 => Insn::Pop,
            2 => Insn::Dup,
            3 => Insn::PushValue(memory.read_word(pc.advance_n(4))?),
            4 => Insn::UAdd,
            5 => Insn::USub,
            6 => Insn::UMul,
            7 => Insn::UDiv,
            8 => Insn::URem,
            9 => Insn::Jump(memory.read_word(pc.advance_n(4))?),
            10 => Insn::JumpEqualZero(memory.read_word(pc.advance_n(4))?),
            11 => Insn::JumpNotZero(memory.read_word(pc.advance_n(4))?),
            _ => return Err(CpuPanic::new()),
        })
    }
}
