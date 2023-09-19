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
        let alu_out: Option<u32> = match decode.opcode {
            Opcode::LW => Some((decode.rs1_data as i32 + decode.imm_i_sext) as u32),
            Opcode::None => None,
            _ => todo!(),
        };

        match alu_out {
            Some(alu_out) => {
                println!("Execute: alu_out: {}", alu_out);
                Ok(ExecuteResult { alu_out })
            }
            // None => Err(ExecuteError::new(ExecuteErrorType::NotMatchOpcode)),
            None => Ok(ExecuteResult { alu_out: 0 }),
        }
    }
}
