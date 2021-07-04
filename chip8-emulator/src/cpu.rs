// This file contains the definition of the chip8 cpu, the cpu is going to be made from the registers, the memory, and it will execute instructions
use std::fs::File;
use std::io::Read;

use crate::Memory;

pub struct Cpu
{
    // Registers
    vx : [u8;16],
    pc : u16,
    sp : u8,
    i : u16,
    ram : Memory
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        // Cpu initialization, the program counter must start at 0x200, and also all the registers are initialized to 0
        Cpu {
            vx:[0;16],
            pc : 0x200,
            sp : 0x00,
            i : 0x00,
            ram : Memory::new()
        }
    }

    pub fn load_rom_file(&mut self, romfile : &str)
    {
        let mut f = File::open(romfile).unwrap();
        let mut romdata = Vec::<u8>::new();
        f.read_to_end(&mut romdata);

        self.ram.loadrom(romdata);

        // self.ram.print();
    }

    pub fn execute_rom(&mut self)
    {
        for i in 0..10
        {
        // With that, start to read the memory and decode the instructions
        // All the instructions are 2 bytes long and the most significant byte is first, so the instruction is the current plus the next, ex: [18][23] = 1823
        let most_significant = self.ram.read_addr(self.pc) as u16;
        let least_significant = self.ram.read_addr(self.pc+1) as u16;

        let instruction : u16 = ((most_significant << 8) | (least_significant)) as u16;

        println!("[] Instruction decoded {:#X}", instruction);
        // Now the instruction can be decoded and executed, and in loop do the next instructions

        // Store the variables from the instructions

        let nnn = instruction & 0x0FFF;
        let n = instruction & 0x000F;
        let x = (instruction & 0x0F00) >> 8;
        let y = instruction & 0x00F0;
        let kk = instruction & 0x00FF;

        println!("nnn: {:#X}, n: {:#X}, x: {:#X}, y: {:#X}, kk: {:#X}", nnn, n, x, y, kk);
        match instruction & 0xF000
        {
            0x1000 => {
                println!("-> [+] JP to {:#X}",nnn);
                self.pc = nnn;
            }
            0x6000 => {
                println!("-> [+] LD V{:?}, {:?}",x,kk);
                self.pc+=2;
            }
            _ => panic!("[X] Instruction not found: {:#X}", instruction)
            
            
        }
        

        }
    }
}