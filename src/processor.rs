use std::fmt::Display;

use crate::bus::Bus;

pub mod riscv;

pub trait Processor {
    fn increment(&mut self, computer: &mut Bus) -> Result<ProcessorResult, ProcessorError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorResult {
    OK,
    ECALL,
}

pub trait ProcessorErrorTrait: Display {}
pub type ProcessorError = Box<dyn ProcessorErrorTrait>;
