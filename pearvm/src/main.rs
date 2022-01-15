mod vm;

use vm::*;

use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => {
            if !args[1].ends_with(".prbc") {
                panic!(
                    "Error: Invalid input file extension: {} (expected .prbc)",
                    args[1]
                );
            }

            let file = File::open(&args[1]).expect("Can not read the input file");
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();

            reader.read_to_end(&mut buffer).expect("Can't read file");

            let mut vm = VM::new(buffer);

            match vm.run() {
                Ok(()) => {}
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        _ => todo!(),
    }
}
