use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::Bus,
    dram::{self, Dram},
};

use super::ProcessorError;

pub struct Fetch {
    bus: Rc<RefCell<Bus>>,
}

impl Fetch {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self { bus }
    }

    pub fn fetch(&mut self, address: u32) -> Result<u32, ProcessorError> {
        println!(
            "0x{:x}",
            Rc::clone(&(self.bus)).borrow_mut().dram.read32(address)?
        );

        Ok(0)
    }
}
