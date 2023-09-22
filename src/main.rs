mod bus;
mod computer;
mod dram;
mod processor;

use computer::Computer;
use std::{env, path::Path};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let mut emulator = Computer::new();

    if let Result::Err(error) = emulator.run(path) {
        println!("{}", error);
    }
}
