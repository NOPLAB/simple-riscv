use std::fmt::Display;

use crate::{bus::DRAM_BASE, dram::DRAM_SIZE};

const REGISTERS_COUNT: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct XRegisters {
    xregs: [u32; REGISTERS_COUNT],
}

impl XRegisters {
    // todo
    pub fn new() -> Self {
        let mut xregs = [0u32; REGISTERS_COUNT];

        // スタックポインターはデフォルトでメモリのスタートアドレス + 最大メモリサイズを入れる
        xregs[2] = DRAM_BASE + DRAM_SIZE;

        Self { xregs }
    }

    pub fn read(&self, index: u32) -> u32 {
        self.xregs[index as usize]
    }

    pub fn write(&mut self, index: u32, value: u32) {
        // zeroレジスタを書き換え不可にする
        if index != 0 {
            self.xregs[index as usize] = value;
        }
    }
}

impl Display for XRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
