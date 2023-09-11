use std::{
    cell::{Cell, RefCell},
    fmt::Display,
    fs::File,
    io::Read,
    path::Path,
    rc::Rc,
};

use crate::bus::Bus;

use self::{fetch::Fetch, register::XRegisters};

pub mod fetch;
pub mod register;

pub struct Processor {
    pub xregs: XRegisters,
    pub pc: u32,

    pub fetch: Fetch,

    pub bus: Rc<RefCell<Bus>>,
}

impl Processor {
    pub fn new() -> Self {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        Self {
            xregs: XRegisters::new(),
            pc: 0,
            fetch: Fetch::new(Rc::clone(&(bus))),
            bus,
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<(), ProcessorError> {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        Rc::clone(&(self.bus)).borrow_mut().dram.load8(0, data)?;
        // println!("{}", Rc::clone(&(self.bus)).borrow_mut().dram);

        self.fetch.fetch(0)?;

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
