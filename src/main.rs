mod parse;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("no arguments provided");

    let preparsed = parse::prepare_file(path);

    println!("Bytecode:");
    for b in preparsed {
        println!("{} {}", b[0], b[1]);
    }
}
