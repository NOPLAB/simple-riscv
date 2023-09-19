use std::path::Path;

use crate::processor::{Processor, ProcessorError};

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

        self.processor = Processor::new();
        self.processor.load(path)?;

        for i in 0..10 {
            self.processor.increment()?;
        }

        Ok(())
    }
}
