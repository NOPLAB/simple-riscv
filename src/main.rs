mod bus;
mod computer;
mod dram;
mod processor;

use computer::Computer;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let path = Path::new("./test/sample02");

    let mut emulator = Computer::new();

    if let Result::Err(error) = emulator.run(path) {
        println!("{}", error);
    }
}
