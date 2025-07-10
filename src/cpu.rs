#![allow(unused)]
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

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode{
                // LDA (Load accumulator) 0xA9 with one parmaeter
                // opcode implementations
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.registers_a = param;

                    if self.registers_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    if self.registers_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }

                // BRK (0x00) opcode implementation
                0x00 => {
                    return;
                }
                _ => todo!()
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.registers_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}

