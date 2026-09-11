use std::env;

/// Tuple struct for command
/// Example: todo create --high "finish logging"
/// This will be constructed as Command("todo", vec!("create", "--high", "finish logging"))
#[derive(Debug)]
pub struct Command(String, Vec<String>);


pub fn parse() -> Command { 

    //env::args() returns an iterable, whose index 0 is irrelevant for our purposes 
    let mut input: Vec<String> = env::args().collect();
    let len = input.len();
    
    match len{
        2_usize => Command(input.remove(1), vec!(String::new())),
        3_usize => Command(input.remove(1), vec!(input.remove(1))),
        4_usize => Command(input.remove(1), vec!(input.remove(1), input.remove(1))),

    
        //TODO-> add help funciton or something when > 2 args (excluding the first "todo" in the
        //input")
        0_usize..=1_usize | 5_usize.. => Command(String::new(), vec!(String::new())),
    }
}
