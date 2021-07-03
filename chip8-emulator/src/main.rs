use std::io;
use std::io::prelude::*;
use std::fs::File;

// Memory definition
/*struct Memory{
    // The memory size is 4096 as seen in http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
    mem: [u8; 4096]
}*/

fn main() -> io::Result<()> {

    // 1- 
    // Open a game file and reads all the contents
    let mut f = File::open("tests/test_opcode.ch8")?;
    let mut romdata = Vec::<u8>::new();

    f.read_to_end(&mut romdata)?;

    // Print all the data
    //print!("Test: {:?}",romdata);

    
   /* let mut memory = Memory{ 
        mem: [0;4096]
    };*/

    // 2- 
    // Initialize memory
    let mut memory : [u8;4096] = [0;4096];

    // Copy the rom data to ram from 0x200
    let offset = 0x200;
    for i in 0..romdata.len()
    {
        memory[offset+i as usize] = romdata[i as usize];
    }

    // Print all the memory
    for x in 0..memory.len() {
        print!("{} ", memory[x]);
    }
    
    Ok(())
}
