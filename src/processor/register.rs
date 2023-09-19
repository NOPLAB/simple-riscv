use std::{fmt::Display, ops::Add};

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

const XREGS_CALL: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0/fp", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

impl Display for XRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for i in 0..32 {
            let s = format!(
                "\x1b[38;5;4m{:0>2}-{}:\x1b[m 0x{:x}, ",
                i,
                XREGS_CALL[i as usize],
                self.read(i)
            );
            res = res.add(&s);
        }
        write!(f, "{}", res)
    }
}
