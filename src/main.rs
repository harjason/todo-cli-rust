use std::fs;

pub mod parser;

fn main() -> std::io::Result<()> {
    fs::create_dir("todos")?;
    println!("welcome to todo app");
    parser::parse();
    Ok(())
}
