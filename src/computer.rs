use crate::processor::{Processor, ProcessorError, ProcessorResult};

pub struct Computer {
    processor: Processor,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            processor: Processor::new(),
        }
    }

    pub fn run(&mut self, program: Vec<u8>) -> Result<(), ProcessorError> {
        self.processor = Processor::new();
        self.processor.load(program)?;

        loop {
            match self.processor.increment()? {
                ProcessorResult::OK => (),
                ProcessorResult::ECALL => break,
            };
            // thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}
