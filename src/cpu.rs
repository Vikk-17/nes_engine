#![allow(unused)]
fn main() {
    pub struct CPU {

        /*
         * 8 bit registes
         * 8 bit for status 
         * 16 bits program counter
         */

        pub registers_a: u8, 
        pub status: u8, 
        pub program_counter: u16, 
    }

    impl CPU {
        pub fn new() -> Self {
            CPU {
                registers_a: 0,
                status: 0,
                program_counter: 0
            }
        }
    }
    pub fn interprt(&mut self, program: Vec<u8>){
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;
            
            match opscode{
                _ => todo!()
            }
        }
    }

}
