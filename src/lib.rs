mod bus;
mod computer;
mod dram;
mod processor;

use processor::{Processor, ProcessorResult};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct WasmComputer {
    processor: Processor,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum WasmComputerResult {
    OK,
    ECALL,
}

#[wasm_bindgen]
impl WasmComputer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            processor: Processor::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) -> Result<(), JsValue> {
        Ok(self.processor.load(program).unwrap())
    }

    pub fn increment(&mut self) -> Result<WasmComputerResult, JsValue> {
        match self.processor.increment() {
            Ok(ProcessorResult::OK) => Ok(WasmComputerResult::OK),
            Ok(ProcessorResult::ECALL) => Ok(WasmComputerResult::ECALL),
            Err(err) => Err(err.to_string().into()),
        }
    }
}
