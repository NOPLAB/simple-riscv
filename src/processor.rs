use std::{fmt::Display, fs::File, io::Read, path::Path};

use crate::bus::Bus;

use self::{decode::Decode, fetch::Fetch, register::XRegisters};

pub mod decode;
pub mod fetch;
pub mod register;

pub struct Processor {
    pub xregs: XRegisters,
    pub pc: u32,

    pub fetch: Fetch,
    pub decode: Decode,

    pub bus: Bus,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            xregs: XRegisters::new(),
            pc: 0,
            fetch: Fetch(),
            decode: Decode(),
            bus: Bus::new(),
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<(), ProcessorError> {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        self.bus.dram.load8(0, data)?;
        // println!("{}", Rc::clone(&(self.bus)).borrow_mut().dram);

        Ok(())
    }

    // todo
    pub fn increment(&mut self) -> Result<(), ProcessorError> {
        let inst = self.fetch.fetch(self.pc / 4, &self.bus)?;
        let decode_res = self.decode.decode(inst, &self.xregs)?;

        self.pc += 4;

        Ok(())
    }
}

pub trait ProcessorErrorTrait: Display {}
pub type ProcessorError = Box<dyn ProcessorErrorTrait>;
