use std::io;
use std::io::prelude::*;
use std::fs::File;

mod memory;
use memory::Memory;

fn main() -> io::Result<()> {

    // 1- 
    // Open a game file and reads all the contents
    let mut f = File::open("tests/test_opcode.ch8")?;
    let mut romdata = Vec::<u8>::new();

    f.read_to_end(&mut romdata)?;

    let mut mem = Memory::new();

    mem.loadrom(romdata);

    mem.print();
    
    Ok(())
}
