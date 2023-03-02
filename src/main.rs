use std::env;

mod firmware;
use firmware::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = Rom::new(args[1].clone())
        .expect("argument not provided");
    let vec = FirmwarePart::create_from_offsets(&rom.get_offsets());

    for part in &vec {
        part.carve_from_rom(&rom).expect("bruh moment");
    }
    
    println!("{:?}", vec);
}
