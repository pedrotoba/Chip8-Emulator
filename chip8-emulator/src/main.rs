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

   // mem.print();

    // Program counter set to the start of the program
    let mut pc = 0x200;

    //loop
   // {
    for i in 0..10
    {
        // With that, start to read the memory and decode the instructions
        // All the instructions are 2 bytes long and the most significant byte is first, so the instruction is the current plus the next, ex: [18][23] = 1823
        let most_significant = mem.read_addr(pc) as u16;
        let least_significant = mem.read_addr(pc+1) as u16;

        let instruction : u16 = ((most_significant << 8) | (least_significant)) as u16;

        println!("[] Instruction decoded {:#X}", instruction);
        // Now the instruction can be decoded and executed, and in loop do the next instructions

        // Store the variables from the instructions

        let nnn = instruction & 0x0FFF;
        let n = instruction & 0x000F;
        let x = instruction & 0x0F00;
        let y = instruction & 0x00F0;
        let kk = instruction & 0x00FF;

        // println!("nnn: {:#X}, n: {:#X}, x: {:#X}, y: {:#X}, kk: {:#X}", nnn, n, x, y, kk);
        match instruction & 0xF000
        {
            0x1000 => {
                println!("-> [+] JP to {:#X}",nnn);
            }
            _ => panic!("[X] Instruction not found: {:#X}", instruction)
            
            
        }
        

    }
    
    Ok(())
}
