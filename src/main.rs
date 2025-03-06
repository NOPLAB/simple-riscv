mod bus;
mod computer;
mod dram;
mod processor;

use bus::Bus;
use computer::Computer;
use processor::RiscVProcessor;
use std::{env, path::Path};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let bus = Bus::new();
    let processor = RiscVProcessor::new();

    let mut emulator = Computer::new(processor, bus);

    emulator.load_from_file(0x80000000, path).unwrap();

    if let Result::Err(error) = emulator.run() {
        println!("{}", error);
    }
}
