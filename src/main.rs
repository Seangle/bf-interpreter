use std::env;
use std::fs;
use std::process::exit;

use bf::interpreter::BfInterpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("usage: bfi <path_to_file>");
        exit(-1)
    }

    let source = fs::read_to_string(&args[1]).unwrap();
    let source = source.as_str();

    let mut bf = BfInterpreter::new();

    bf.interpret(source);
}
