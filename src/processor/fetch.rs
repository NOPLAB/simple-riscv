use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{bus::Bus, dram::DramError};

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

pub struct Fetch {
    bus: Rc<RefCell<Bus>>,
}

impl Fetch {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self { bus }
    }

    // todo
    pub fn fetch(&mut self, pc: u32) -> Result<u32, ProcessorError> {
        println!(
            "Fetch: 0x{:x}",
            Rc::clone(&(self.bus)).borrow_mut().dram.read32(pc)?
        );

        let physical_pc = pc;

        match self.bus.borrow().dram.read32(physical_pc) {
            Ok(value) => Ok(value),
            Err(err) => Err(FetchError::new(FetchErrorType::DramError(err))),
        }
    }
}
