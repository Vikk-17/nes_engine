#![allow(unused)]

// Bit Indexing
const CARRY_FLAG: u8 = 0b0000_0001;  // Bit 0
const ZERO_FLAG: u8 = 0b0000_0010;  // Bit 1
const INTERRUPT_FLAG: u8 = 0b0000_0100;  // Bit 2
const DECIMAL_FLAG: u8 = 0b0000_1000;  // Bit 3
const BREAK_FLAG: u8 = 0b0001_0000;  // Bit 4
const OVERFLOW_FLAG: u8 = 0b0100_0000;  // Bit 6
const NEGATIVE_FLAG: u8 = 0b1000_0000;  // Bit 7

pub struct CPU {

    /*
     * 8 bit register
     * 8 bit for status 
     * 16 bits program counter
     */

    pub register_a: u8, 
    pub register_x: u8,
    pub status: u8, 
    pub program_counter: u16, 
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0
        }
    }
    
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        // copies the current content of accumulator into the x register
        self.register_x = self.register_a;
        // sets the zero and negative flags as appropriate
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // ZERO_FLAG (Z): Set if result == 0
        if result == 0 {
            // set Bit 1
            self.status |= ZERO_FLAG;
        } else {
            // clear Bit 1
            self.status &= !ZERO_FLAG;
        }

        // NEGATIVE_FLAG (N): Set if bit 7 of result is set
        if result & NEGATIVE_FLAG != 0 {
            // set Bit 7
            self.status |= NEGATIVE_FLAG;
        } else {
            // clear Bit 7
            self.status &= !NEGATIVE_FLAG;
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

                    self.lda(param);
                }

                // TAX (0xAA) opscode
                0xAA => self.tax(), 

                0xE8 => self.inx(),

                // BRK (0x00) opcode implementation
                0x00 => return,

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
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}

