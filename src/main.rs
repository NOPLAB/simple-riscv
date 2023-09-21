mod bus;
mod computer;
mod dram;
mod processor;

use bitvec::bitvec;
use bitvec::field::BitField;
use bitvec::prelude::Lsb0;
use bitvec::view::BitView;
use computer::Computer;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let mut vec = bitvec![u32, Lsb0;];
    let inst: u32 = 0b0000_0000_0000_0000_0000_0000_1000_1100;
    let slice = inst.view_bits::<Lsb0>();
    vec.append(&mut slice[4..=7].to_bitvec());
    vec.append(&mut slice[0..=3].to_bitvec());
    vec.push(true);

    // vec.extend_from_bitslice(&mut slice[8..=11].to_bitvec());
    // vec.push(false);

    // vec.reverse();
    println!("{}", vec);
    println!("{}", vec[2]);

    let path = Path::new("./test/rv32ui-p-add");

    let mut emulator = Computer::new();

    if let Result::Err(error) = emulator.run(path) {
        println!("{}", error);
    }
}
