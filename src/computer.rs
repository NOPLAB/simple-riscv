use std::fs::File;
use std::io::Read;
use std::path::Path;

use thiserror::Error;

use crate::bus::Bus;
use crate::processor::Processor;
use crate::processor::{ProcessorError, ProcessorResult};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("File open error")]
    FileOpenError(std::io::Error),

    #[error("File read error")]
    FileReadError(std::io::Error),
}

pub struct Computer<P>
where
    P: Processor,
{
    processor: P,
    bus: Bus,
}

impl<P> Computer<P>
where
    P: Processor,
{
    pub fn new(processor: P, bus: Bus) -> Self {
        Self { processor, bus }
    }

    pub fn load_from_file(&mut self, start_address: u32, path: &Path) -> Result<(), LoadError> {
        let mut program_file = File::open(path).map_err(|e| LoadError::FileOpenError(e))?;
        let mut program_data = Vec::new();
        program_file.read_to_end(&mut program_data).map_err(|e| LoadError::FileReadError(e))?;

        let _ = self.bus.load8(start_address, program_data); // TODO

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ProcessorError> {
        loop {
            match self.processor.increment(&mut self.bus)? {
                ProcessorResult::OK => (),
                ProcessorResult::ECALL => break,
            };
            // thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}
