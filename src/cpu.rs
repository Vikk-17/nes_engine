#![allow(unused)]

// Bit Indexing
const CARRY_FLAG: u8 = 0b0000_0001;  // Bit 0
const ZERO_FLAG: u8 = 0b0000_0010;  // Bit 1
const INTERRUPT_FLAG: u8 = 0b0000_0100;  // Bit 2
const DECIMAL_FLAG: u8 = 0b0000_1000;  // Bit 3
const BREAK_FLAG: u8 = 0b0001_0000;  // Bit 4
const OVERFLOW_FLAG: u8 = 0b0100_0000;  // Bit 6
const NEGATIVE_FLAG: u8 = 0b1000_0000;  // Bit 7


#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NonAddressing,
}


pub struct CPU {

    /*
     * 8 bit register
     * 8 bit for status 
     * 16 bits program counter
     */

    pub register_a: u8, 
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8, 
    pub program_counter: u16, 
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF], // memory: [expr, N]
        }
    }
   
    fn memory_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
   
    fn memory_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }


    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }


    pub fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let low = self.memory_read(pos) as u16;
        let high = self.memory_read(pos + 1) as u16;

        (high << 8) | (low as u16)
    }


    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.memory_write(pos, low);
        self.memory_write(pos + 1, high);
    }

    pub fn run(&mut self) {
        loop {
            let code = self.memory_read(self.program_counter);
            self.program_counter += 1;
            
            match code {
                0xA9 => {
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xA5 => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xAD => {
                    self.lda(&AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0x85 => {
                    self.sta(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0x95 => {
                    self.sta(&AddressingMode::ZeroPage_X);
                    self.program_counter += 1;
                }
                _ => todo!(),
            }
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }


    // get the address of all operands
    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode:: Immediate => self.program_counter,
            AddressingMode:: ZeroPage => self.memory_read(self.program_counter) as u16,
            AddressingMode:: Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode:: ZeroPage_X => {
                let pos = self.memory_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode:: ZeroPage_Y => {
                let pos = self.memory_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode:: Indirect_X => {
                let base = self.memory_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.memory_read(ptr as u16);
                let hi = self.memory_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.memory_read(self.program_counter);
                let lo = self.memory_read(base as u16);
                let hi = self.memory_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NonAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.memory_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.memory_write(addr, self.register_a);
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

    // pub fn interpret(&mut self, program: Vec<u8>) {
    //     self.program_counter = 0;
    //
    //     loop {
    //         let opscode = program[self.program_counter as usize];
    //         self.program_counter += 1;
    //
    //         match opscode{
    //             // LDA (Load accumulator) 0xA9 with one parmaeter
    //             // opcode implementations
    //             0xA9 => {
    //                 let param = program[self.program_counter as usize];
    //                 self.program_counter += 1;
    //
    //                 self.lda(param);
    //             }
    //
    //             // TAX (0xAA) opscode
    //             0xAA => self.tax(), 
    //
    //             0xE8 => self.inx(),
    //
    //             // BRK (0x00) opcode implementation
    //             0x00 => return,
    //
    //             _ => todo!()
    //         }
    //     }
    // }
}


#[cfg(test)]
mod test {
    use super::*;
    //
    // #[test]
    // fn test_0xa9_lda_immediate_load_data() {
    //     let mut cpu = CPU::new();
    //     cpu.interpret(vec![0xa9, 0x05, 0x00]);
    //     assert_eq!(cpu.register_a, 0x05);
    //     assert!(cpu.status & 0b0000_0010 == 0b00);
    //     assert!(cpu.status & 0b1000_0000 == 0);
    // }
    //
    // #[test]
    // fn test_0xa9_lda_zero_flag() {
    //     let mut cpu = CPU::new();
    //     cpu.interpret(vec![0xA9, 0x00, 0x00]);
    //     assert!(cpu.status & 0b0000_0010 == 0b10);
    // }
    //
    // #[test]
    // fn test_0xaa_tax_move_a_to_x() {
    //     let mut cpu = CPU::new();
    //     cpu.register_a = 10;
    //     cpu.interpret(vec![0xaa, 0x00]);
    //
    //     assert_eq!(cpu.register_x, 10);
    // }
    //
    // #[test]
    // fn test_5_ops_working_together() {
    //     let mut cpu = CPU::new();
    //     cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    //
    //     assert_eq!(cpu.register_x, 0xc1)
    // }
    //
    // #[test]
    // fn test_inx_overflow() {
    //     let mut cpu = CPU::new();
    //     cpu.register_x = 0xff;
    //     cpu.interpret(vec![0xe8, 0xe8, 0x00]);
    //
    //     assert_eq!(cpu.register_x, 1)
    // }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.memory_write(0x10, 0x55);
        cpu.load_and_run(vec![0x55, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }
}

