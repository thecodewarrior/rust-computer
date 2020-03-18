pub mod instructions;
pub mod memory;
pub mod cpu;

pub type CpuResult<T> = Result<T, CpuPanic>;

pub use instructions::*;
pub use memory::*;
pub use cpu::*;