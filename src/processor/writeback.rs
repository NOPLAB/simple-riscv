use std::fmt::Display;

use crate::bus::Bus;

use super::{
    decode::{DecodeResult, Opcode},
    execute::ExecuteResult,
    register::XRegisters,
    ProcessorError, ProcessorErrorTrait,
};

pub enum WritebackErrorType {}

pub struct WritebackError {
    error_type: WritebackErrorType,
}

impl WritebackError {
    fn new(error_type: WritebackErrorType) -> Box<Self> {
        Box::new(WritebackError { error_type })
    }
}

impl ProcessorErrorTrait for WritebackError {}

impl Display for WritebackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            _ => todo!(),
        }
    }
}

pub struct Writeback();

impl Writeback {
    pub fn writeback(
        &self,
        decode: DecodeResult,
        execute: ExecuteResult,
        xregs: &mut XRegisters,
        bus: &mut Bus,
    ) -> Result<(), ProcessorError> {
        let rd_data = bus.dram.read32(execute.alu_out)?;

        println!("Writeback: rd(wb_data) 0x{:0>8x}({})", rd_data, rd_data);

        match decode.opcode {
            Opcode::LW => xregs.write(decode.rd, rd_data),
            Opcode::SW => bus.dram.write32(execute.alu_out, decode.rs2_data)?,
        }
        Ok(())
    }
}
