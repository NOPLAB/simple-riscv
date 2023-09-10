mod bus;
mod dram;
mod emulator;
mod processor;

use emulator::Emulator;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let path = Path::new("./test/rv32ui-p-add");

    let mut emulator = Emulator::new(path);

    if let Result::Err(error) = emulator.run() {
        println!("{}", error);
    }
}
