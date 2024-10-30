use thiserror::Error;

use crate::vm::VirtualMachine;

#[derive(Error, Debug)]
pub enum InstructionError {
    #[error("Op Code {0} is invalid")]
    InvalidOpCode(u16),
}

pub struct BitReader {
    val: u16,
    idx: u16,
}

impl BitReader {
    pub fn new(val: u16) -> Self {
        Self { val, idx: 16 }
    }

    pub fn read(&mut self, bits: u16) -> u16 {
        assert!(bits <= self.idx);
        let old = self.idx;
        self.idx -= bits;

        ((1 << (old - self.idx)) - 1) & (self.val >> self.idx)
    }

    pub fn read_to_end(&mut self) -> u16 {
        let old = self.idx;
        self.idx = 0;

        ((1 << (old - self.idx)) - 1) & (self.val >> self.idx)
    }

    pub fn skip(&mut self, bits: u16) {
        assert!(bits <= self.idx);
        self.idx -= bits;
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::BitReader;

    #[test]
    fn test_bit_reader() {
        let mut reader = BitReader::new(0b1011_0011_1010_1100);
        assert_eq!(0b1011, reader.read(4));
        assert_eq!(0b0011, reader.read(4));
        assert_eq!(0b1010, reader.read(4));
        assert_eq!(0b1100, reader.read_to_end());
    }
}

pub fn handle_instruction(instr: u16, vm: &mut VirtualMachine) -> Result<bool, InstructionError> {
    let mut reader = BitReader::new(instr);
    let op_code = reader.read(4);

    match op_code {
        0b0001 => {
            // Handle ADD Instruction
            let dest_register = reader.read(3);
            let source_register_1 = reader.read(3);

            if reader.read(1) == 1 {
                let val = reader.read_to_end();
                let source = vm.get_register(source_register_1);

                vm.set_register(dest_register, source.wrapping_add_unsigned(val));
            } else {
                reader.skip(2);
                let source_register_2 = reader.read_to_end();

                let source_a = vm.get_register(source_register_1);
                let source_b = vm.get_register(source_register_2);

                vm.set_register(dest_register, source_a.wrapping_add(source_b));
            }
        }
        0b0101 => {
            // Handle AND Instruction
            let dest_register = reader.read(3);
            let source_register_1 = reader.read(3);

            if reader.read(1) == 1 {
                let val = reader.read_to_end();
                let source = vm.get_register(source_register_1);

                vm.set_register(dest_register, val as i16 & source);
            } else {
                reader.skip(2);
                let source_register_2 = reader.read_to_end();

                let source_a = vm.get_register(source_register_1);
                let source_b = vm.get_register(source_register_2);

                vm.set_register(dest_register, source_a & source_b);
            }
        }
        0b0000 => {
            // Handle BR Instrucution
            let n = reader.read(1) != 0;
            let z = reader.read(1) != 0;
            let p = reader.read(1) != 0;

            let dist = reader.read_to_end();

            let condition = vm.get_register(vm.get_condition_register());

            if (!n && !z && !p)
                || (n && condition < 0)
                || (z && condition == 0)
                || (p && condition > 0)
            {
                vm.move_pc(dist);
            }
        }
        0b1100 => {
            // Handle JMP and RET Instructions
        }
        0b0100 => {
            // Handle JSR Instruction
        }
        0b0010 => {
            // Handle LD Instruction
        }
        0b1010 => {
            // Handle LDI Instruction
        }
        0b0110 => {
            // Handle LDR Instruction
        }
        0b1110 => {
            // Handle LEA Instruction
        }
        0b1001 => {
            // Handle NOT Instruction
            let dest_reg = reader.read(3);
            let source_reg = reader.read(3);

            assert_eq!(reader.read_to_end(), 0b111111);

            let source_val = vm.get_register(source_reg);
            vm.set_register(dest_reg, !source_val);
        }
        0b1000 => {
            // Handle RTI Instruction
        }
        0b0011 => {
            // Handle ST Instruction
        }
        0b1011 => {
            // Handle STI Instruction
        }
        0b0111 => {
            // Handle STR Instruction
        }
        0b1111 => {
            // Handle TRAP Instruction
            reader.skip(4);
            let trap_code = reader.read_to_end();

            match trap_code {
                0x25 => {
                    return Ok(false);
                }
                _ => {
                    panic!("Invalid Trap Code Found!")
                }
            }
        }
        0b1101 => {
            // Reserved
        }
        _ => return Err(InstructionError::InvalidOpCode(op_code)),
    }
    Ok(true)
}
