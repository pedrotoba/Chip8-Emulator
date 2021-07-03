pub struct Memory
{
    mem: [u8; 4096],
}

impl Memory
{
    pub fn new() -> Memory {
        Memory { mem:[0;4096] }
    }

    pub fn loadrom(&mut self, rombuffer : Vec<u8>)
    {
        let offset = 0x200;
        for i in 0..rombuffer.len()
        {
           self.mem[offset+i as usize] = rombuffer[i as usize];
        }
    }

    pub fn read_addr(&self, addr : u16) -> u8
    {
        self.mem[addr as usize]
    }

    pub fn write_addr(&mut self, addr : u16, value : u8)
    {
        self.mem[0x200 + addr as usize] = value;
    }

    pub fn print(&self)
    {
        for i in 0..self.mem.len() {
            print!("{:#X} ", self.mem[i]);
        }
        println!();
    }
}