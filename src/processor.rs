use std::{fmt::Display, fs::File, io::Read, path::Path};

use crate::bus::Bus;

use self::{
    decode::Decode, execute::Execute, fetch::Fetch, register::XRegisters, writeback::Writeback,
};

pub mod decode;
pub mod execute;
pub mod fetch;
pub mod register;
pub mod writeback;

pub struct Processor {
    pub xregs: XRegisters,
    pub pc: u32,

    pub fetch: Fetch,
    pub decode: Decode,
    pub execute: Execute,
    pub writeback: Writeback,

    pub bus: Bus,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            xregs: XRegisters::new(),
            pc: 0,
            fetch: Fetch(),
            decode: Decode(),
            execute: Execute(),
            writeback: Writeback(),
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
        println!("Xregisters: {}", self.xregs);
        let inst = self.fetch.fetch(self.pc, &self.bus)?;
        let decode_res = self.decode.decode(inst, &self.xregs)?;
        let execute_res = self
            .execute
            .execute(decode_res, &mut self.bus, &mut self.xregs)?;
        self.writeback
            .writeback(decode_res, execute_res, &mut self.xregs, &mut self.bus)?;

        self.pc += 4;

        println!("");

        Ok(())
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ProcessorErrorTrait: Display {}
pub type ProcessorError = Box<dyn ProcessorErrorTrait>;
