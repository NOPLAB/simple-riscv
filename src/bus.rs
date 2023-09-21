use std::fmt::Display;

use crate::{
    dram::Dram,
    processor::{ProcessorError, ProcessorErrorTrait},
};

pub enum BusErrorType {
    AddressOutOfBounds,
}

pub struct BusError {
    pub error_type: BusErrorType,
}

impl BusError {
    fn new(error_type: BusErrorType) -> Box<Self> {
        Box::new(BusError { error_type })
    }
}

impl ProcessorErrorTrait for BusError {}

impl Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            BusErrorType::AddressOutOfBounds => write!(f, "AddressOutOfBounds"),
        }
    }
}

pub const DRAM_BASE: u32 = 0x800_0000;

#[derive(Debug, Clone)]
pub struct Bus {
    pub dram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Self { dram: Dram::new() }
    }

    pub fn load8(&mut self, start_address: u32, data: Vec<u8>) -> Result<(), ProcessorError> {
        if start_address >= DRAM_BASE as u32 {
            self.dram.load8(start_address - DRAM_BASE as u32, data)
        } else {
            Err(BusError::new(BusErrorType::AddressOutOfBounds))
        }
    }

    pub fn read8(&self, address: u32) -> Result<u8, ProcessorError> {
        if address >= DRAM_BASE as u32 {
            self.dram.read8(address - DRAM_BASE as u32)
        } else {
            Err(BusError::new(BusErrorType::AddressOutOfBounds))
        }
    }

    pub fn read16(&self, address: u32) -> Result<u16, ProcessorError> {
        if address >= DRAM_BASE as u32 {
            self.dram.read16(address - DRAM_BASE as u32)
        } else {
            Err(BusError::new(BusErrorType::AddressOutOfBounds))
        }
    }

    pub fn read32(&self, address: u32) -> Result<u32, ProcessorError> {
        if address >= DRAM_BASE as u32 {
            self.dram.read32(address - DRAM_BASE as u32)
        } else {
            Err(BusError::new(BusErrorType::AddressOutOfBounds))
        }
    }

    pub fn write8(&mut self, address: u32, value: u8) -> Result<(), ProcessorError> {
        if address >= DRAM_BASE as u32 {
            self.dram.write8(address - DRAM_BASE as u32, value)
        } else {
            Err(BusError::new(BusErrorType::AddressOutOfBounds))
        }
    }

    pub fn write16(&mut self, address: u32, value: u16) -> Result<(), ProcessorError> {
        if address >= DRAM_BASE as u32 {
            self.dram.write16(address - DRAM_BASE as u32, value)
        } else {
            Err(BusError::new(BusErrorType::AddressOutOfBounds))
        }
    }

    pub fn write32(&mut self, address: u32, value: u32) -> Result<(), ProcessorError> {
        if address >= DRAM_BASE as u32 {
            self.dram.write32(address - DRAM_BASE as u32, value)
        } else {
            Err(BusError::new(BusErrorType::AddressOutOfBounds))
        }
    }
}
