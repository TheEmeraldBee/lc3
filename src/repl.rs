use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use common::{instructions::handle_instruction, vm::VirtualMachine};

pub fn repl(mut vm: VirtualMachine) -> Result<(), Box<dyn Error>> {
    let stdin = stdin();

    println!(
        "Menu:\n\
        registers -- print out all registers and their values\n\
        \n\
        quit -- exit the program\n\
        "
    );

    // Simple Virtual Machine
    loop {
        let mut instr = String::new();
        print!(":> ");
        stdout().flush()?;
        stdin.read_line(&mut instr)?;

        instr = instr.trim().to_string();

        match instr.as_str() {
            "registers" => {
                for i in 0..8 {
                    println!("R{i} :: {}", vm.get_register(i))
                }
                continue;
            }
            "quit" => break,
            _ => {}
        }

        let Ok(instruction) = u16::from_str_radix(&instr, 2) else {
            println!("Input a 16bit instruction or a valid command!!");
            continue;
        };

        if instr.len() != 16 {
            println!("You must input all 16 bits of the instruction, no truncation!");
            continue;
        }

        let res = handle_instruction(instruction, &mut vm);
        if let Err(e) = res {
            eprintln!("{e}");
        }
    }
    Ok(())
}
