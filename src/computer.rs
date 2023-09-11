use std::{fs::File, io::Read, path::Path};

use crate::{
    bus::Bus,
    processor::{Processor, ProcessorError},
};

pub struct Computer {
    processor: Processor,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            processor: Processor::new(),
        }
    }

    pub fn run(&mut self, path: &Path) -> Result<(), ProcessorError> {
        println!("RUN - {}", path.to_str().unwrap());

        self.processor.load(path)?;

        Ok(())
    }
}
