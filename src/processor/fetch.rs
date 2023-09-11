use crate::{
    bus::Bus,
    dram::{self, Dram},
};

use super::ProcessorError;

pub struct Fetch<'a> {
    bus: &'a Bus,
}

impl<'a> Fetch<'a> {
    pub fn new(bus: &'a Bus) -> Self {
        Self { bus }
    }

    pub fn fetch(&mut self, address: u32) -> Result<u32, ProcessorError> {
        println!("{}", self.bus.dram.read32(0)?);
        todo!()
    }
}
