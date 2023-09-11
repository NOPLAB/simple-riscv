use std::{fmt::Display, fs::File, io::Read, path::Path};

use crate::bus::Bus;

use self::{fetch::Fetch, register::XRegisters};

pub mod fetch;
pub mod register;

pub struct Processor {
    pub xregs: XRegisters,
    pub pc: u32,

    pub bus: Bus,
}

impl Processor {
    pub fn new() -> Self {
        let mut bus = Bus::new();
        Self {
            xregs: XRegisters::new(),
            pc: 0,
            bus,
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<(), ProcessorError> {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        self.bus.dram.load8(0, data)?;
        println!("{}", self.bus.dram);

        Ok(())
    }
}

pub trait ProcessorErrorTrait: Display {}
pub type ProcessorError = Box<dyn ProcessorErrorTrait>;

pub fn sign_extension(value: u32, n_top: u32) -> u32 {
    // 符号を取得
    let sign = (value >> n_top) % 2;

    // 符号ビットを除外する
    let mut value = value % 2u32.pow(n_top);

    // 符号がマイナス
    if sign == 1 {
        // 符号拡張
        value = u32::MAX - 2u32.pow(n_top) + value + 1
    }

    value
}
