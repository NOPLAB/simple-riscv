use std::fmt::Display;

use crate::bus::Bus;

use super::{ProcessorError, ProcessorErrorTrait};

pub enum FetchErrorType {
    DramError(Box<dyn ProcessorErrorTrait>),
}

pub struct FetchError {
    error_type: FetchErrorType,
}

impl FetchError {
    fn new(error_type: FetchErrorType) -> Box<Self> {
        Box::new(FetchError { error_type })
    }
}

impl ProcessorErrorTrait for FetchError {}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            FetchErrorType::DramError(err) => write!(f, "Dram Error: {}", err),
        }
    }
}

pub struct Fetch();

impl Fetch {
    pub fn fetch(&mut self, pc: u32, bus: &Bus) -> Result<u32, ProcessorError> {
        println!("Fetch: 0x{:0>8x}", bus.dram.read32(pc)?);

        let physical_pc = pc;

        match bus.dram.read32(physical_pc) {
            Ok(value) => Ok(value),
            Err(err) => Err(FetchError::new(FetchErrorType::DramError(err))),
        }
    }
}
