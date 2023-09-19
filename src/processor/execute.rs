use std::fmt::Display;

use crate::bus::Bus;

use super::{
    decode::{DecodeResult, Opcode},
    register::XRegisters,
    ProcessorError, ProcessorErrorTrait,
};

pub enum ExecuteErrorType {
    NotMatchOpcode,
}

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
            ExecuteErrorType::NotMatchOpcode => write!(f, "Not match opcode."),
        }
    }
}

pub struct Execute();

impl Execute {
    pub fn execute(
        &self,
        decode: DecodeResult,
        bus: &mut Bus,
        xregs: &mut XRegisters,
    ) -> Result<u32, ProcessorError> {
        match decode.opcode {
            Opcode::LW => Ok(decode.rs1 + decode.imm_i_sext),
            Opcode::None => Err(ExecuteError::new(ExecuteErrorType::NotMatchOpcode)),
            _ => todo!(),
        }
    }
}
