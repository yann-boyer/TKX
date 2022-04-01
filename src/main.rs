mod opcodes;
mod interpreter;

use crate::interpreter::Interpreter;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage : ./tkx <bf-program>");
        std::process::exit(1);
    }

    let program_path = &args[1];

    let mut interpreter = Interpreter::new();
    interpreter.load_program(program_path);
    interpreter.run();
}