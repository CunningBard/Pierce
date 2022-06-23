mod lexer;
mod parser;
mod code_gen;
mod ast_base;

use std::env;

fn main() {
    let args: Vec<String>  = env::args().collect();
    let path;
    if args.len() != 1 {
        path = args.get(1).unwrap();
    } else if args.len() > 1 {
        println!("arguments more than 1");
        std::process::exit(1);
    }
    else {
        println!("path to script not given");
        std::process::exit(1);
    }

    let file = std::fs::read_to_string(path).expect("File doesnt exists or couldnt be opened");

    println!("---------------------------------------\n\
          {}\n---------------------------------------", file);

    let mut parser = parser::Parser::new();
    parser.parse_text(file)

}
