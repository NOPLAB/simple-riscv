use std::path::Path;

use crate::processor::{Processor, ProcessorError};

pub struct Emulator<'a> {
    processor: Processor,
    path: &'a Path,
}

impl<'a> Emulator<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self {
            processor: Processor::new(),
            path,
        }
    }

    pub fn run(&mut self) -> Result<(), ProcessorError> {
        println!("RUN - {}", self.path.to_str().unwrap());

        self.processor.load(self.path)?;

        Ok(())
    }
}
