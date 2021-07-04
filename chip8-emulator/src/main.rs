
//mod memory;
//use memory::Memory;
mod memory;
use memory::Memory;

mod cpu;
use cpu::Cpu;


fn main(){


    let mut chip8_cpu = Cpu::new();

    chip8_cpu.load_rom_file("tests/test_opcode.ch8");
    chip8_cpu.execute_rom();

}
