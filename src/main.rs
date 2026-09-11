//use std::fs;
use std::env;

pub mod parser;

fn main() -> std::io::Result<()> {
    //fs::create_dir("todos")?;

    //env::args() returns an iterable, whose index 0 is irrelevant for our purposes 
    let input: Vec<String> = env::args().collect(); 
    let mut operation: String = String::new();

    let mut args_vec: Vec<&String> = Vec::new(); 
    
    parser::parse(&input, &mut operation, &mut args_vec);

    println!("input {:?}", input);
    println!("operation: {}", operation);
    println!("args: {:?}", args_vec);
   // let input: parser::Command = parser::parse();
    Ok(())
}
