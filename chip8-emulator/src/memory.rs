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

    pub fn print(&self)
    {
        for i in 0..self.mem.len() {
            print!("{} ", self.mem[i]);
        }
    }
}