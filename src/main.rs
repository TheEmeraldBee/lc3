use byteorder::{BigEndian, ReadBytesExt};
use common::vm::VirtualMachine;
use repl::repl;
use std::{env::args, error::Error, fs::File, io::BufReader};

mod repl;

fn main() -> Result<(), Box<dyn Error>> {
    let mut vm = VirtualMachine::default();

    if let Some(filepath) = args().nth(1) {
        let mut file_reader = BufReader::new(File::open(filepath)?);

        let pc = file_reader.read_u16::<BigEndian>()?;
        vm.set_pc(pc);

        let mut idx = pc;

        while let Ok(val) = file_reader.read_i16::<BigEndian>() {
            vm.set_memory(idx, val);
            idx += 1;
        }

        vm.run()?;
    } else {
        repl(vm)?;
    }

    Ok(())
}
