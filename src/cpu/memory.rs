use super::*;
use std::convert::TryInto;

pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(memory_size: usize) -> Memory {
        Memory {
            data: vec![0; memory_size],
        }
    }

    pub fn read_byte(&self, address: u32) -> CpuResult<u8> {
        let address = address as usize;
        if address >= self.data.len() {
            return Err(CpuPanic::new());
        }
        Ok(self.data[address])
    }

    pub fn read_short(&self, address: u32) -> CpuResult<u16> {
        let address = address as usize;
        if address >= self.data.len() - 2 {
            return Err(CpuPanic::new());
        }
        Ok(u16::from_be_bytes(
            self.data[address..address+2].try_into().unwrap()
        ))
    }

    pub fn read_word(&self, address: u32) -> CpuResult<u32> {
        let address = address as usize;
        if address >= self.data.len() - 4 {
            return Err(CpuPanic::new());
        }

        Ok(u32::from_be_bytes(
            self.data[address..address + 4].try_into().unwrap(),
        ))
    }

    pub fn write_byte(&mut self, address: u32, value: u8) -> CpuResult<()> {
        let address = address as usize;
        if address >= self.data.len() {
            return Err(CpuPanic::new());
        }

        self.data[address as usize] = value;
        Ok(())
    }

    pub fn write_short(&mut self, address: u32, value: u16) -> CpuResult<()> {
        let address = address as usize;
        if address >= self.data.len() - 2 {
            return Err(CpuPanic::new());
        }

        self.data[address..address + 2].copy_from_slice(&value.to_be_bytes());
        Ok(())
    }

    pub fn write_word(&mut self, address: u32, value: u32) -> CpuResult<()> {
        let address = address as usize;
        if address >= self.data.len() - 4 {
            return Err(CpuPanic::new());
        }

        self.data[address..address + 4].copy_from_slice(&value.to_be_bytes());
        Ok(())
    }

    pub fn write_width(&mut self, width: DataWidth, address: u32, value: u32) -> CpuResult<()> {
        match width {
            DataWidth::Byte => self.write_byte(address, value as u8),
            DataWidth::Short => self.write_short(address, value as u16),
            DataWidth::Word => self.write_word(address, value),
        }
    }

    pub fn read_width(&self, width: DataWidth, address: u32) -> CpuResult<u32> {
        match width {
            DataWidth::Byte => self.read_byte(address).map(|x| x as u32),
            DataWidth::Short => self.read_short(address).map(|x| x as u32),
            DataWidth::Word => self.read_word(address),
        }
    }
}

#[derive(Clone, Copy)]
pub enum DataWidth {
    Byte, Short, Word
}

impl DataWidth {
    pub fn bitmask(&self) -> u32 {
        match self {
            DataWidth::Byte => 0xff,
            DataWidth::Short => 0xffff,
            DataWidth::Word => 0xffff_ffff,
        } 
    }

    pub fn size(&self) -> usize {
        match self {
            DataWidth::Byte => 1,
            DataWidth::Short => 2,
            DataWidth::Word => 4,
        } 
    }
}