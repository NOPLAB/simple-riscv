use std::{path::Path, thread, time::Duration};

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

        loop {
            self.processor.increment()?;
            // thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}
