use std::path::PathBuf;
use std::env;
use std::fs;

pub mod parser;
pub mod executor;

fn main() -> std::io::Result<()> {
    let path = PathBuf::from("todos");
    if !path.exists(){
        fs::create_dir("todos")?;
    }

    //env::args() returns an iterable, whose index 0 is irrelevant for our purposes 
    let input: Vec<String> = env::args().collect(); 
    let mut operation: String = String::new();
    let mut args_vec: Vec<&String> = Vec::new(); 
    
    parser::parse(&input, &mut operation, &mut args_vec);
    executor::execute(&operation, &args_vec);

    Ok(())
}
