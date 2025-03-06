use std::fmt::Display;

use crate::{bus::Bus, processor::decode::Opcode};

use self::{
    cs_register::ControlAndStatusRegister, decode::Decode, execute::Execute, fetch::Fetch,
    writeback::Writeback, x_register::XRegisters,
};

pub mod cs_register;
pub mod decode;
pub mod execute;
pub mod fetch;
pub mod writeback;
pub mod x_register;

pub trait Processor {
    fn increment(&mut self, computer: &mut Bus) -> Result<ProcessorResult, ProcessorError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorResult {
    OK,
    ECALL,
}

pub struct RiscVProcessor {
    pub xregs: XRegisters,
    pub csr: ControlAndStatusRegister,
    pub pc: u32,

    pub fetch: Fetch,
    pub decode: Decode,
    pub execute: Execute,
    pub writeback: Writeback,
}

impl RiscVProcessor {
    pub fn new() -> Self {
        Self {
            xregs: XRegisters::new(),
            csr: ControlAndStatusRegister::new(),
            pc: 0x80000000 + 0x1000,
            fetch: Fetch(),
            decode: Decode(),
            execute: Execute(),
            writeback: Writeback(),
        }
    }
}

impl Processor for RiscVProcessor {
    // todo
    fn increment(&mut self, bus: &mut Bus) -> Result<ProcessorResult, ProcessorError> {
        println!("pc: 0x{:0>8x}", self.pc - 0x1000); // !DO

        println!("Xregisters: {}", self.xregs);
        let inst = self.fetch.fetch(self.pc, &bus)?;
        let decode_res = self.decode.decode(inst, &self.xregs)?;
        let execute_res = self.execute.execute(decode_res, self.pc)?;
        self.writeback.writeback(
            decode_res,
            execute_res,
            &mut self.xregs,
            &mut self.csr,
            bus,
        )?;

        // この処理はFetchでやるべき
        if let Some(br_target) = execute_res.br_target {
            self.pc = br_target;
            println!("Processor: BR TARGET: {:x}", br_target);
        } else if let Some(jmp_target) = execute_res.jmp_target {
            self.pc = jmp_target;
            println!("Processor: JMP TARGET: {:x}", jmp_target);
        } else if decode_res.opcode == Opcode::ECALL {
            self.pc = self.csr.read(0x305);
            println!("Processor: ECALL!!!!");
            return Ok(ProcessorResult::ECALL);
        } else {
            self.pc += 4;
        }

        println!();

        Ok(ProcessorResult::OK)
    }
}

pub trait ProcessorErrorTrait: Display {}
pub type ProcessorError = Box<dyn ProcessorErrorTrait>;
