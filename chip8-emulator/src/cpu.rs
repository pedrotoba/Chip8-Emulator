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
    ram : Memory,
    stack: Vec<u16>
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
            ram : Memory::new(),
            stack : Vec::new()
        }
    }

    pub fn load_rom_file(&mut self, romfile : &str)
    {
        let mut f = File::open(romfile).unwrap();
        let mut romdata = Vec::<u8>::new();
        f.read_to_end(&mut romdata);

        self.ram.loadrom(romdata);

        //self.ram.print();
    }

    pub fn execute_rom(&mut self)
    {
        println!("*** Registers : {:?}",self.vx);
        println!("*** PC : {:#X}",self.pc);
       // println!("0x247 {:#X} 0x248 {:#X}", self.ram.read_addr(0x247),self.ram.read_addr(0x248));
        //panic!();
        loop
        {
            // With that, start to read the memory and decode the instructions
            // All the instructions are 2 bytes long and the most significant byte is first, so the instruction is the current plus the next, ex: [18][23] = 1823
            let most_significant = self.ram.read_addr(self.pc) as u16;
            let least_significant = self.ram.read_addr(self.pc+1) as u16;

            let instruction : u16 = ((most_significant << 8) | (least_significant)) as u16;

            // Now the instruction can be decoded and executed, and in loop do the next instructions

            // Store the variables from the instructions

            let nnn = instruction & 0x0FFF;
            let n = instruction & 0x000F;
            let x = (instruction & 0x0F00) >> 8;
            let y = (instruction & 0x00F0) >> 4;
            let kk = instruction & 0x00FF;

            println!("[+] Instruction decoded {:#X}, Values -> nnn: {:#X}, n: {:#X}, x: {:#X}, y: {:#X}, kk: {:#X}", instruction, nnn, n, x, y, kk);

            match instruction & 0xF000
            {
                0x0 => {
                    match kk
                    {
                        0xEE => {
                            println!("-> RET"); // Return from subroutine
                            // Get the last element in stack
                            let pop  = match self.stack.pop() {
                                Some(val) => self.pc = val,
                                None => panic!("[X] Stack empty.")
                            };

                            
                        }
                        0xE0 => {
                            println!("-> CLS"); // Clear the screen
                        }
                        _ => {panic!("[X] Error in instruction {:#X}", instruction);}
                    }
                }
                
                0x1000 => {
                    println!("-> JP to {:#X}",nnn); // Jump to nnn
                    self.pc = nnn;
                }
                0x2000 => {
                    println!("-> CALL {:#X}",nnn); // Call subroutine at nnn.
                    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                    self.stack.push(self.pc + 2);
                    self.pc = nnn;
                }
                0x3000 => {
                    println!("-> SE V{:?}, {:?}",x,kk); // Skip next instruction if Vx = kk.
                    if self.vx[x as usize] == kk as u8
                    {
                        self.pc += 4;
                    }
                    else
                    {
                        self.pc += 2;
                    }
                }
                0x4000 => {
                    println!("-> SNE V{:?}, {:?}",x,kk); // Skip next instruction if Vx != kk.
                    if self.vx[x as usize] != kk as u8
                    {
                        self.pc += 4;
                    }
                    else
                    {
                        self.pc += 2;
                    }
                }
                0x5000 => {
                    println!("-> SE V{:?}, V{:?}",x,y); // Skip next instruction if Vx = Vy.
                    if self.vx[x as usize] == self.vx[y as usize]
                    {
                        self.pc += 4;
                    }
                    else
                    {
                        self.pc += 2;
                    }
                }
                0x6000 => {
                    println!("-> LD V{:?}, {:?}",x,kk); // Set Vx = kk.
                    self.vx[x as usize] = kk as u8;
                    self.pc+=2;
                }
                0x7000 => {
                    println!("-> ADD V{:?}, {:?}",x,kk); // Set Vx = Vx + kk.

                    self.vx[x as usize] = self.vx[x as usize].wrapping_add(kk as u8);
                    self.pc+=2;
                }
                0x8000 => {
                    match n
                    {
                        0x0 => {
                            println!("-> LD V{:?}, V{:?}",self.vx[x as usize], self.vx[y as usize]); // Set Vx = Vy.
                            self.vx[x as usize] = self.vx[y as usize];
                        }
                        0x1 => {
                            println!("-> OR V{:?}, V{:?}",self.vx[x as usize], self.vx[y as usize]); // Set Vx = Vx OR Vy.
                            self.vx[x as usize] |= self.vx[y as usize];
                        }
                        0x2 => {
                            println!("-> AND V{:?}, V{:?}",self.vx[x as usize], self.vx[y as usize]); // Set Vx = Vx AND Vy.
                            self.vx[x as usize] &= self.vx[y as usize];
                        }
                        0x3 => {
                            println!("-> XOR V{:?}, V{:?}",self.vx[x as usize], self.vx[y as usize]); // Set Vx = Vx XOR Vy.
                            self.vx[x as usize] ^= self.vx[y as usize];
                        }
                        0x4 => {
                            println!("-> ADD V{:?}, V{:?}",self.vx[x as usize], self.vx[y as usize]); // Set Vx = Vx + Vy, set VF = carry.
                            let sum : u16 = self.vx[x as usize] as u16 + self.vx[y as usize] as u16;
                            self.vx[x as usize] = sum as u8;
                            if sum > 0xFF
                            {
                                self.vx[0xF] = 1;
                            }
                        }
                        0x5 => {
                            println!("-> SUB V{:?}, V{:?}",self.vx[x as usize], self.vx[y as usize]); // Set Vx = Vx - Vy, set VF = NOT borrow.

                            self.vx[x as usize] = self.vx[x as usize].wrapping_sub(self.vx[y as usize] as u8);
                            if self.vx[x as usize] > self.vx[y as usize]
                            {
                                self.vx[0xF] = 1;
                            }
                            else
                            {
                                self.vx[0xF] = 0;
                            }
                        }
                        0x6 => {
                            println!("-> SHR V{:?}, 1",self.vx[x as usize]); // Set Vx = Vx SHR 1.
                            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                            self.vx[0xF] = (self.vx[x as usize] & 0b00000001) >> 7; // Compare the last bit and insert into vf
                            self.vx[x as usize] >>= 1; // Dividing by two shifting one bit to the right
                        }
                        0xE => {
                            println!("-> SHL V{:?}, 1",self.vx[x as usize]); // Set Vx = Vx SHL 1
                            // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                            self.vx[0xF] = (self.vx[x as usize] & 0b10000000) >> 7; // Compare the first bit and insert into vf
                            self.vx[x as usize] <<= 1; // Multiply by two shifting one bit to the left
                        }
                        _ => {
                            panic!("[X] Instruction 8XY[] not found: {:#X}", instruction);
                        }
                    }
                    self.pc+=2;
                }
                0x9000 => {
                    println!("-> SNE V{:?}, V{:?}",x,y); // Skip next instruction if Vx != Vy.
                    if self.vx[x as usize] != self.vx[y as usize]
                    {
                        self.pc += 4;
                    }
                    else
                    {
                        self.pc += 2;
                    }
                }
                0xA000 => {
                    println!("-> LD I, {:?}",nnn);
                    self.i = nnn;
                    self.pc+=2;
                }
                0xE000 => {
                    println!("-> LD I, {:?}",nnn);
                    self.i = nnn;
                    self.pc+=2;
                }
                0xD000 => {
                    println!("-> DRW Vx({:?}), Vy({:?}), {:?}",x,y,n);
                // self.i = nnn;
                /*
                    Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                    The interpreter reads n bytes from memory, starting at the address stored in I. 
                    These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. 
                    If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. 
                    If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                */
                    self.pc+=2;
                }
                0xF000 => {
                    match kk
                    {
                        0x1E => {
                            println!("-> ADD I-{:?}, V{:?}",self.i,self.vx[x as usize]);
                            self.i += self.vx[x as usize] as u16;
                            self.pc+=2;
                        }
                        0x55 => {
                            println!("-> LD I-{:?}, V{:?}",self.i,self.vx[x as usize]); // Store registers V0 through Vx in memory starting at location I.
                            // Loop throught all the registers from 0 to x
                            for i in 0..x 
                            {
                                self.ram.write_addr(self.i+i, self.vx[i as usize]);
                            }
                            self.pc+=2;
                        }
                        _ => {
                            panic!("[X] Instruction F not found: {:#X}", instruction);
                        }
                    }
                }
                _ => panic!("[X] Instruction not found: {:#X}", instruction)
                
                
            }

            println!("\t*** Registers Vx: {:?} , I: {:?}",self.vx, self.i);
            println!("\t*** Stack: {:?}",self.stack);
            println!("\t*** PC : {:#X}",self.pc);

           //let mut line = String::new();
          // let b1 = std::io::stdin().read_line(&mut line).unwrap();
            

        }
    }
}