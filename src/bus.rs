use crate::dram::Dram;

pub const DRAM_BASE: usize = 0x8000_0000;

pub struct Bus {
    pub dram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Self { dram: Dram::new() }
    }
}
