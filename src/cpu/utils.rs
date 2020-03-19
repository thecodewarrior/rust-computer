///! Various utilities or small types that only serve to clutter the more focused cpu files

pub struct ProgramCounter {
    pub address: u32,
}

impl ProgramCounter {
    pub fn new(address: u32) -> ProgramCounter {
        ProgramCounter { address }
    }

    /// Increments the address and returns the original value
    pub fn advance(&mut self) -> u32 {
        let original = self.address;
        self.address += 1;
        return original;
    }

    /// Increments the address by n and returns the original value
    pub fn advance_n(&mut self, amount: u32) -> u32 {
        let original = self.address;
        self.address += amount;
        return original;
    }
}

pub struct CpuPanic {}

impl CpuPanic {
    pub fn new() -> CpuPanic {
        CpuPanic {}
    }
}
