use std::error::Error;

use crate::instructions::handle_instruction;

#[derive(Debug)]
pub struct VirtualMachine {
    pc: u16,
    memory: [i16; u16::MAX as usize],
    registers: [i16; 8],
    condition_register: u16,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            pc: 0,
            memory: [0; u16::MAX as usize],
            registers: [0; 8],
            condition_register: 0,
        }
    }
}

impl VirtualMachine {
    pub fn set_pc(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn move_pc(&mut self, dist: u16) {
        self.pc = self.pc.wrapping_add(dist);
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let instr = self.get_memory(self.pc);

            // Check if instruction is a halt instruction
            if !handle_instruction(instr as u16, self)? || self.pc >= u16::MAX - 1 {
                break;
            }

            self.pc += 1;
        }
        Ok(())
    }

    pub fn set_memory(&mut self, addr: u16, value: i16) {
        self.memory[addr as usize] = value;
    }

    pub fn get_memory(&self, addr: u16) -> i16 {
        self.memory[addr as usize]
    }

    pub fn set_register(&mut self, loc: u16, val: i16) {
        self.condition_register = loc;
        self.registers[loc as usize] = val;
    }

    pub fn get_register(&self, loc: u16) -> i16 {
        self.registers[loc as usize]
    }

    pub fn get_condition_register(&self) -> u16 {
        self.condition_register
    }
}
