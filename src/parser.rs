use std::env;

//pub struct Command(String, String, String);

pub fn parse(){
    let input: Vec<String> = env::args().collect();
    dbg!(input);
}
