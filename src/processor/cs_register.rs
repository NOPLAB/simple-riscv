use std::{fmt::Display, ops::Add};

const REGISTERS_COUNT: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct ControlAndStatusRegister {
    csregs: [u32; REGISTERS_COUNT],
}

impl ControlAndStatusRegister {
    pub fn new() -> Self {
        let mut xregs = [0u32; REGISTERS_COUNT];

        Self { csregs: xregs }
    }

    pub fn read(&self, index: u32) -> u32 {
        self.csregs[index as usize]
    }

    pub fn write(&mut self, index: u32, value: u32) {
        self.csregs[index as usize] = value;
    }
}

impl Display for ControlAndStatusRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for i in 0..REGISTERS_COUNT {
            let s = format!(
                "\x1b[38;5;4m{:0>4}:\x1b[m 0x{:x} \n",
                i,
                self.read(i as u32)
            );
            res = res.add(&s);
        }
        write!(f, "{}", res)
    }
}
