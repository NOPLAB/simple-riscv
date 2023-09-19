use std::fmt::Display;

use crate::bus::Bus;

use super::{
    decode::{DecodeResult, Opcode},
    register::XRegisters,
    ProcessorError, ProcessorErrorTrait,
};

pub enum ExecuteErrorType {}

pub struct ExecuteError {
    error_type: ExecuteErrorType,
}

impl ExecuteError {
    fn new(error_type: ExecuteErrorType) -> Box<Self> {
        Box::new(ExecuteError { error_type })
    }
}

impl ProcessorErrorTrait for ExecuteError {}

impl Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            _ => todo!(),
        }
    }
}

pub struct ExecuteResult {
    pub alu_out: u32,
}

pub struct Execute();

impl Execute {
    pub fn execute(
        &self,
        decode: DecodeResult,
        bus: &mut Bus,
        xregs: &mut XRegisters,
    ) -> Result<ExecuteResult, ProcessorError> {
        let alu_out: u32 = match decode.opcode {
            Opcode::LW => (decode.rs1_data as i32 + decode.imm_i_sext) as u32,
            _ => todo!(),
        };
        println!("Execute: alu_out: {}", alu_out);
        Ok(ExecuteResult { alu_out })
    }
}
