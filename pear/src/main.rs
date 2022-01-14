mod vm;

use vm::*;

fn main() {
    let mut vm = VM::new(vec![1, 2, 0, 3, 4]);
    match vm.run() {
        Ok(()) => {},
        Err(err) => eprintln!("Error: {}", err)
    }
}
