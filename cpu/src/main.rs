#![allow(unused)]
fn main() {
    pub struct CPU {
        pub registers_a: u8, // 8 bit memory, see the docs
        pub status: u8, // 8 bit memory
        pub program_counter: u16, // 16 bits only for program program_counter
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

    pub fn interpret(&mut self, program: Vec<u8>){
        todo!("")
    }
}
