use crate::processor::{ProcessorError, ProcessorErrorTrait};
use std::fmt::Display;

pub enum DramErrorType {
    AddressOutOfBounds,
}

pub struct DramError {
    error_type: DramErrorType,
}

impl DramError {
    fn new(error_type: DramErrorType) -> Box<Self> {
        Box::new(DramError { error_type })
    }
}

impl ProcessorErrorTrait for DramError {}

impl Display for DramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            DramErrorType::AddressOutOfBounds => write!(f, "Address is Out Of Range."),
        }
    }
}

pub const DRAM_SIZE: u32 = 1024 * 1024 * 1; // 1MB

#[derive(Debug, Clone)]
pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new() -> Self {
        Self {
            dram: vec![0; DRAM_SIZE as usize],
        }
    }

    pub fn load8(&mut self, start_address: u32, data: Vec<u8>) -> Result<(), ProcessorError> {
        if start_address + (data.len() as u32) < DRAM_SIZE as u32 {
            for (count, mem) in self
                .dram
                .iter_mut()
                .skip(start_address as usize)
                .take(data.len())
                .enumerate()
            {
                *mem = data[count];
            }
            Ok(())
        } else {
            Err(DramError::new(DramErrorType::AddressOutOfBounds))
        }
    }

    pub fn read8(&self, address: u32) -> Result<u8, ProcessorError> {
        let mem_address = address; // None

        if mem_address < DRAM_SIZE as u32 {
            Ok(self.dram[mem_address as usize])
        } else {
            Err(DramError::new(DramErrorType::AddressOutOfBounds))
        }
    }

    pub fn read16(&self, address: u32) -> Result<u16, ProcessorError> {
        let mem_address = address << 1; // multi 2

        if mem_address < DRAM_SIZE as u32 {
            Ok((self.dram[mem_address as usize] as u16) << 8
                | (self.dram[mem_address as usize + 1] as u16))
        } else {
            Err(DramError::new(DramErrorType::AddressOutOfBounds))
        }
    }

    pub fn read32(&self, address: u32) -> Result<u32, ProcessorError> {
        let mem_address = address << 2; // multi 4

        if mem_address < DRAM_SIZE as u32 {
            Ok((self.dram[mem_address as usize] as u32) << 24
                | (self.dram[mem_address as usize + 1] as u32) << 16
                | (self.dram[mem_address as usize + 2] as u32) << 8
                | (self.dram[mem_address as usize + 3] as u32))
        } else {
            Err(DramError::new(DramErrorType::AddressOutOfBounds))
        }
    }

    pub fn write8(&mut self, address: u32, value: u8) -> Result<(), ProcessorError> {
        let mem_address = address; // None

        if mem_address < DRAM_SIZE as u32 {
            self.dram[mem_address as usize] = value;
            Ok(())
        } else {
            Err(DramError::new(DramErrorType::AddressOutOfBounds))
        }
    }

    pub fn write16(&mut self, address: u32, value: u16) -> Result<(), ProcessorError> {
        let mem_address = address << 1; // div 2

        if mem_address < DRAM_SIZE as u32 {
            self.dram[mem_address as usize] = (value >> 8) as u8;
            self.dram[mem_address as usize + 1] = value as u8;
            Ok(())
        } else {
            Err(DramError::new(DramErrorType::AddressOutOfBounds))
        }
    }

    pub fn write32(&mut self, address: u32, value: u32) -> Result<(), ProcessorError> {
        let mem_address = address << 2; // div 4

        if mem_address < DRAM_SIZE as u32 {
            self.dram[mem_address as usize] = (value >> 24) as u8;
            self.dram[mem_address as usize + 1] = ((value & 0x00FF_0000) >> 16) as u8;
            self.dram[mem_address as usize + 2] = ((value & 0x0000_FF00) >> 8) as u8;
            self.dram[mem_address as usize + 3] = value as u8;

            Ok(())
        } else {
            Err(DramError::new(DramErrorType::AddressOutOfBounds))
        }
    }
}

impl Display for Dram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for i in 0..DRAM_SIZE {
            str += &format!("{:0>2x} ", self.dram[i as usize]).to_string();
            if i % 8 == 7 {
                str += " ";
            }
            if i % 16 == 15 {
                str += "\n";
            }
        }

        write!(f, "{}", str)
    }
}
