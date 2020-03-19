pub mod cpu;
pub mod instructions;
pub mod memory;
pub mod utils;

pub type CpuResult<T> = Result<T, CpuPanic>;

pub use cpu::*;
pub use instructions::*;
pub use memory::*;
pub use utils::*;
